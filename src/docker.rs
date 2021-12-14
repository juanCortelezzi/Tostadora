use clap::ArgMatches;

use crate::commands::TostCmd;

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
    TostCmd::new("systemctl", vec!["start", "docker.socket", "docker"]).run()
}

fn stop() {
    // systemctl stop docker.socket docker
    TostCmd::new("systemctl", vec!["stop", "docker.socket", "docker"]).run()
}

fn is_running() -> bool {
    // systemctl is-active --quiet docker
    TostCmd::new("systemctl", vec!["is-active", "--quiet", "docker"]).run_status()
}

fn status() {
    if is_running() {
        println!("docker is running");
    } else {
        println!("docker is not running");
    }
}
