use clap::ArgMatches;

use crate::commands::run_cmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => start(),
        Some(("stop", _)) => stop(),
        Some(("status", _)) => status(),
        _ => println!("No subcommand was used"),
    };
}

fn start() {
    run_cmd(
        "systemctl",
        [
            "--user",
            "start",
            "pipewire.socket",
            "pipewire-pulse.socket",
            "pipewire",
            "wireplumber",
            "pipewire-pulse",
        ],
    );
}

fn stop() {
    run_cmd(
        "systemctl",
        [
            "--user",
            "stop",
            "pipewire.socket",
            "pipewire-pulse.socket",
            "pipewire",
            "wireplumber",
            "pipewire-pulse",
        ],
    );
}

pub fn is_running() -> bool {
    run_cmd("systemctl", ["--user", "is-active", "--quiet", "pipewire"])
}

fn status() {
    if is_running() {
        println!("pipewire is running");
    } else {
        println!("pipewire is not running");
    }
}
