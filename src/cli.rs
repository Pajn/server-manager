use clap::App;

pub fn build_cli() -> App<'static, 'static> {
    clap_app!(srvm =>
        (version: "0.1")
        (author: "Rasmus Eneman <rasmus@eneman.eu>")
        (about: "Perform tasks on remote servers")
        (@arg config: -c --config +takes_value "Sets a custom config file")
        (@subcommand list =>
            (about: "list environments and tasks")
        )
        (@subcommand ssh =>
            (about: "ssh into an environment")
            (@arg environment: "environment to ssh into")
        )
        (@subcommand run =>
            (about: "run a task in an environment")
            (@arg task: "task to run")
            (@arg environment: -e --environment +takes_value "environment to run the task in")
        )
        (@subcommand cmd =>
            (about: "run a shell command in an environment")
            (@arg environment: -e --environment +takes_value "environment to run the task in")
            (@setting AllowExternalSubcommands)
        )
    )
}
