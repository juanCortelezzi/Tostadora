use crate::commands::TostCmd;
use clap::{ArgMatches, Command};

pub fn get_command() -> Command<'static> {
    Command::new("docker")
        .about("Starts and stops the docker service/s")
        .subcommand(Command::new("start").about("Starts docker service/s"))
        .subcommand(Command::new("stop").about("Stops docker service/s"))
        .subcommand(Command::new("status").about("Status of docker service/s"))
}

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => start(),
        Some(("stop", _)) => stop(),
        Some(("status", _)) => status(),
        _ => println!("No subcommand was used"),
    }
}

fn start() {
    // systemctl start docker.socket docker
    TostCmd::new("systemctl", &["start", "docker.socket", "docker"]).run()
}

fn stop() {
    // systemctl stop docker.socket docker
    TostCmd::new("systemctl", &["stop", "docker.socket", "docker"]).run()
}

fn is_running() -> bool {
    // systemctl is-active --quiet docker
    TostCmd::new("systemctl", &["is-active", "--quiet", "docker"]).run_status()
}

fn status() {
    if is_running() {
        println!("docker is running");
    } else {
        println!("docker is not running");
    }
}
