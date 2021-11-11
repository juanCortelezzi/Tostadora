use clap::ArgMatches;

use crate::commands::run_cmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => run_cmd("doas", ["systemctl", "start", "docker.socket", "docker"]),
        Some(("stop", _)) => run_cmd("doas", ["systemctl", "stop", "docker.socket", "docker"]),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}
