use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::Read;
use std::string::String;
use yaml_rust::{ScanError, Yaml, YamlLoader};
use yaml_rust::yaml::Hash;

#[derive(Clone, Debug)]
pub struct SshConfiguration {
    pub host: String,
    pub port: Option<u16>,
    pub key_file: Option<String>,
    pub user: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Service {
    Ssh(SshConfiguration),
}

#[derive(Clone, Debug)]
pub enum Task {
    Command(String),
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub service: Service,
    pub tasks: HashMap<String, Task>,
}

#[derive(Clone, Debug)]
pub struct Configuration {
    pub default_evironment: Option<String>,
    pub environments: HashMap<String, Environment>,
}

pub enum Error {
    Io(io::Error),
    Yaml(String),
    YamlParse(ScanError),
}

fn parse_ssh_configuration(config: &Yaml) -> Result<SshConfiguration, Error> {
    if let Some(host) = config.as_str() {
        Ok(SshConfiguration {
            host: host.to_owned(),
            port: None,
            key_file: None,
            user: None,
        })
    } else if let Some(config) = config.as_hash() {
        let host = config.get(&Yaml::String("host".to_owned()))
            .and_then(Yaml::as_str)
            .map(Into::into)
            .ok_or(Error::Yaml("SSH host must be a string".to_owned()))?;
        Ok(SshConfiguration {
            host: host,
            port: config.get(&Yaml::String("port".to_owned()))
                .and_then(Yaml::as_i64)
                .map(|p| p as u16),
            key_file: config.get(&Yaml::String("key_file".to_owned()))
                .and_then(Yaml::as_str)
                .map(|key_file| key_file.trim())
                .map(Into::into),
            user: config.get(&Yaml::String("user".to_owned()))
                .and_then(Yaml::as_str)
                .map(Into::into),
        })
    } else {
        Err(Error::Yaml("SSH configuration must be a string or a hash".to_owned()))
    }
}
fn parse_service(service: &Hash) -> Result<Service, Error> {
    if let Some(config) = service.get(&Yaml::String("ssh".to_owned())) {
        parse_ssh_configuration(config).map(Service::Ssh)
    } else {
        Err(Error::Yaml("Invalid service type".to_owned()))
    }
}
fn parse_task(task: &Yaml) -> Result<Task, Error> {
    if let Some(command) = task.as_str() {
        Ok(Task::Command(command.to_owned()))
    } else if let Some(task) = task.as_hash() {
        let task_type = task.get(&Yaml::String("type".to_owned()))
            .and_then(Yaml::as_str)
            .ok_or(Error::Yaml("A task definition must have a type property".to_owned()))?;

        match task_type {
            "command" => {
                task.get(&Yaml::String("command".to_owned()))
                    .and_then(Yaml::as_str)
                    .ok_or(Error::Yaml("The command property must be a string".to_owned()))
                    .map(|command| Task::Command(command.to_owned()))
            }
            _ => Err(Error::Yaml(format!("Invalid type \"{}\"", task_type))),
        }
    } else {
        Err(Error::Yaml("A task definition must be a string (for a command) or a hash".to_owned()))
    }
}

fn parse_environment_config(config: &Hash) -> Result<Environment, Error> {
    let service = config.get(&Yaml::String("service".to_owned()))
        .and_then(Yaml::as_hash)
        .ok_or(Error::Yaml("A service definition must be a hash".to_owned()))
        .and_then(parse_service)?;
    let tasks = config.get(&Yaml::String("tasks".to_owned()))
        .or(Some(&Yaml::Hash(BTreeMap::new())))
        .and_then(Yaml::as_hash)
        .ok_or(Error::Yaml("The tasks property must be a hash".to_owned()))
        .and_then(|tasks| {
            tasks.iter()
                .map(|(name, config)| {
                    name.as_str()
                        .ok_or(Error::Yaml("Task names must be strings".to_owned()))
                        .and_then(|name| parse_task(config).map(|config| (name.to_owned(), config)))
                })
                .collect()
        })?;

    Ok(Environment {
        service: service,
        tasks: tasks,
    })
}

pub fn parse_configuration_file(file: &str) -> Result<Configuration, Error> {
    let mut file_contents = String::new();
    File::open(file).map_err(Error::Io)?
        .read_to_string(&mut file_contents)
        .map_err(Error::Io)?;
    let yaml = YamlLoader::load_from_str(&file_contents).map_err(Error::YamlParse)?.remove(0);
    let config = yaml.as_hash()
        .ok_or(Error::Yaml("The configuration file must be a hash".to_owned()))?;
    let environments = config.get(&Yaml::String("environments".to_owned()))
        .or(Some(&Yaml::Hash(BTreeMap::new())))
        .and_then(Yaml::as_hash)
        .ok_or(Error::Yaml("The environments property must be a hash".to_owned()))
        .and_then(|environments| {
            environments.iter()
                .map(|(name, config)| {
                    name.as_str()
                        .ok_or(Error::Yaml("Environment names must be strings".to_owned()))
                        .and_then(|name| {
                            config.as_hash()
                                .ok_or(Error::Yaml("Environment definitions must be a hash"
                                    .to_owned()))
                                .and_then(parse_environment_config)
                                .map(|config| (name.to_owned(), config))
                        })
                })
                .collect()
        })?;

    Ok(Configuration {
        default_evironment: config.get(&Yaml::String("default_evironment".to_owned()))
            .and_then(Yaml::as_str)
            .map(Into::into),
        environments: environments,
    })
}
