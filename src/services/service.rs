use config::{Service, Task};
use services::ssh;

pub fn enter_ssh(service: &Service) -> Result<(), ()> {
    match service {
        &Service::Ssh(ref config) => ssh::enter_ssh(config),
    }
}

pub fn run_command(service: &Service, command: &str) -> Result<(), ()> {
    match service {
        &Service::Ssh(ref config) => ssh::run_command(config, command),
    }
}

pub fn run_task(service: &Service, task: &Task) -> Result<(), ()> {
    match task {
        &Task::Command(ref command) => run_command(service, command),
    }
}
