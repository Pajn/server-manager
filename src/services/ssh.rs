use std::process::Command;
use config::SshConfiguration;

impl SshConfiguration {
    fn as_command<'a>(&self) -> Command {
        let connection_string = if let Some(ref user) = self.user {
            format!("{}@{}", user, self.host)
        } else {
            self.host.to_owned()
        };

        let mut command = Command::new("ssh");
        command.arg(connection_string);

        if let Some(port) = self.port {
            command.arg(format!("-p {}", port));
        }

        if let Some(ref key_file) = self.key_file {
            command.arg(format!("-i{}", key_file));
        }

        command
    }
}

pub fn enter_ssh(config: &SshConfiguration) -> Result<(), ()> {
    let mut cmd = config.as_command();

    if let Ok(mut child) = cmd.spawn() {
        child.wait().map(|_| ()).map_err(|_| ())
    } else {
        Err(())
    }
}

pub fn run_command(config: &SshConfiguration, command: &str) -> Result<(), ()> {
    let mut cmd = config.as_command();
    cmd.arg(command);

    if let Ok(mut child) = cmd.spawn() {
        child.wait().map(|_| ()).map_err(|_| ())
    } else {
        Err(())
    }
}
