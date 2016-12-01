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
        )
        (@subcommand run =>
            (about: "run a task in an environment")
        )
        (@subcommand cmd =>
            (about: "run a shell command in an environment")
        )
    )
}
