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
    TostCmd::new("systemctl", vec!["start", "docker.socket", "docker"])
        .add_privilleges()
        .run()
}

fn stop() {
    TostCmd::new("systemctl", vec!["stop", "docker.socket", "docker"])
        .add_privilleges()
        .run()
}

fn is_running() -> bool {
    TostCmd::new("systemctl", vec!["is-active", "--quiet", "docker"]).run_status()
}

fn status() {
    if is_running() {
        println!("docker is running");
    } else {
        println!("docker is not running");
    }
}
