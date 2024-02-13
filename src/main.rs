pub mod util;

use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, ArgGroup, Command};

/* #[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Set rustchat to act as a client
    #[arg(short, long, default_value_t = true)]
    client: bool,

    /// Set rustchat to act as a server
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// The host to connect to
    #[arg(short, long)]
    host: String,

    /// Your name to use
    #[arg(short, long)]
    name: String,

    /// Whether or not to use rustchat interactively
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Display help
    #[arg(long, action = clap::ArgAction::Count)]
    help: bool,
} */

fn main() {
    let matches = command!()
        .disable_help_flag(true)
        .arg(
            arg!(
                -c --client "Set rustchat to act as client"
            )
            .required(false)
            .action(ArgAction::SetTrue)
            .default_value("true")
        )
        .arg(arg!(
            -s --server "Set rustchat to act as server"
            )
            .required(false)
            .action(ArgAction::SetTrue)
        )
        .arg(arg!(
                -h --host <HOST> "The host to connect to"
            )
            .required_unless_present("interactive")
        )
        .arg(arg!(
                -n --name <NAME> "Your preferred username"
            )
            .required_unless_present("interactive")
        )
        .arg(
            arg!(
                -i --interactive "Whether or not to use rustchat interactively"
            )
            .required(false)
            .action(ArgAction::SetTrue)
        )
        .arg(
            arg!(
                --help "Display help"
            )
            .action(ArgAction::Help)
        )
        .group(
            ArgGroup::new("mode")
                .args(&["server","client"])
                .required(false)
        )
        .get_matches();

    loop {
        if matches.get_flag("interactive") {
            unimplemented!();
        } else {
            
        }
    }
}
