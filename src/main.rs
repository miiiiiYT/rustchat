pub mod util;
pub mod client;
pub mod interactive;
pub mod server;
pub mod ident;

use clap::{arg, command, ArgAction, ArgGroup, value_parser};

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
            .required_unless_present_any(["interactive","server"])
            .value_parser(value_parser!(String))
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

    if matches.get_flag("interactive") {
        interactive::main();
        return;
    } else if matches.get_flag("server") {
        server::main();
        return;
    } else if matches.get_flag("client") {
        client::main(matches.get_one::<String>("host").unwrap().to_string(), matches.get_one::<String>("name").unwrap().to_string());
        return;
    } else {
        unreachable!();
    }
}