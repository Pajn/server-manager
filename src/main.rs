mod cli;
mod cli_interface;
mod config;
mod services;

#[macro_use]
extern crate clap;
extern crate inquirer;
extern crate yaml_rust;

use std::iter::Iterator;
use clap::ArgMatches;
use cli_interface::select_item;
use config::{Configuration, Environment, Task, parse_configuration_file};
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
    } else if let Some(matches) = matches.subcommand_matches("ssh") {
        let environment = get_environment(&config, &matches);
        enter_ssh(&environment.service).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("cmd") {
        let environment = get_environment(&config, &matches);
        match matches.subcommand() {
            (external, Some(matches)) => {
                let args: Vec<&str> = matches.values_of("")
                    .map_or_else(Vec::new, Iterator::collect);
                let cmd = format!("{} {}", external, args.join(" "));
                run_command(&environment.service, &cmd).unwrap();
            }
            _ => {
                println!("No command specified");
                std::process::exit(1);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let environment = get_environment(&config, &matches);
        let task = get_task(&environment, &matches);
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

fn get_environment<'a>(config: &'a Configuration, matchers: &'a ArgMatches) -> &'a Environment {
    match matchers.value_of("environment") {
        Some(environment) => {
            match config.environments.get(environment) {
                Some(environment) => environment,
                None => {
                    println!("Invalid environment {}", environment);
                    std::process::exit(1);
                }
            }
        }
        None => select_environment(config),
    }
}

fn get_task<'a>(environment: &'a Environment, matchers: &'a ArgMatches) -> &'a Task {
    match matchers.value_of("task") {
        Some(task) => {
            match environment.tasks.get(task) {
                Some(task) => task,
                None => {
                    println!("Invalid task {}", task);
                    std::process::exit(1);
                }
            }
        }
        None => select_item("Choose task:", environment.tasks.iter().collect()),
    }
}

fn select_environment(config: &Configuration) -> &Environment {
    let environments: Vec<(&String, &Environment)> = config.environments.iter().collect();

    select_item("Choose environment:", environments)
}
