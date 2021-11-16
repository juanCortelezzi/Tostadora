use clap::ArgMatches;

use crate::commands::run_cmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => start(),
        Some(("stop", _)) => stop(),
        Some(("status", _)) => status(),
        _ => println!("No subcommand was used"),
    }
}

fn start() {
    run_cmd("doas", ["systemctl", "start", "docker.socket", "docker"]);
}

fn stop() {
    run_cmd("doas", ["systemctl", "stop", "docker.socket", "docker"]);
}

fn is_running() -> bool {
    run_cmd("systemctl", ["is-active", "--quiet", "docker"])
}

fn status() {
    if is_running() {
        println!("docker is running");
    } else {
        println!("docker is not running");
    }
}
