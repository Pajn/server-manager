#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("srvm", Shell::Bash, env!("OUT_DIR"));
    app.gen_completions("srvm", Shell::Zsh, env!("OUT_DIR"));
}