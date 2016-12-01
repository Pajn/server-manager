mod cli;
mod cli_interface;
mod config;
mod services;

#[macro_use]
extern crate clap;
extern crate inquirer;
extern crate yaml_rust;

use cli_interface::select_item;
use config::{Configuration, Environment, parse_configuration_file};
use services::service::{enter_ssh, run_command, run_task};

fn main() {
    let matches = cli::build_cli().get_matches();
    let config_file = matches.value_of("config").unwrap_or("srvm.yaml");

    let config = match parse_configuration_file(config_file) {
        Ok(config) => config,
        Err(config::Error::Io(_)) => {
            println!("Could not read config file");
            std::process::exit(1);
        }
        Err(config::Error::Yaml(error)) => {
            println!("Could not parse config file:\n  {}", error);
            std::process::exit(2);
        }
        Err(config::Error::YamlParse(error)) => {
            println!("Could not parse config file:\n  {}", error);
            std::process::exit(2);
        }
    };

    if let Some(_) = matches.subcommand_matches("list") {
        list_environments(config)
    } else if let Some(_) = matches.subcommand_matches("ssh") {
        let environment = select_environment(&config);
        enter_ssh(&environment.service).unwrap();
    } else if let Some(_) = matches.subcommand_matches("cmd") {
        let environment = select_environment(&config);
        run_command(&environment.service, "ls").unwrap();
    } else if let Some(_) = matches.subcommand_matches("run") {
        let environment = select_environment(&config);
        let task = select_item("Choose task:", environment.tasks.iter().collect());
        run_task(&environment.service, &task).unwrap();
    }
}

fn list_environments(config: Configuration) {
    for (name, environment) in config.environments {
        print!("{}", name);
        if Some(name) == config.default_evironment {
            print!("  [default]");
        }
        println!("");
        for (name, _) in environment.tasks {
            println!("  - {}", name);
        }
    }
}

fn select_environment(config: &Configuration) -> &Environment {
    let environments: Vec<(&String, &Environment)> = config.environments.iter().collect();

    select_item("Choose environment:", environments)
}
