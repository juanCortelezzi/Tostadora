use clap::ArgMatches;

use crate::commands::TostCmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => start(),
        Some(("stop", _)) => stop(),
        Some(("status", _)) => status(),
        _ => println!("No subcommand was used"),
    };
}

fn start() {
    TostCmd::new(
        "systemctl",
        vec![
            "--user",
            "start",
            "pipewire.socket",
            "pipewire-pulse.socket",
            "pipewire",
            "wireplumber",
            "pipewire-pulse",
        ],
    )
    .run()
}

fn stop() {
    TostCmd::new(
        "systemctl",
        vec![
            "--user",
            "stop",
            "pipewire.socket",
            "pipewire-pulse.socket",
            "pipewire",
            "wireplumber",
            "pipewire-pulse",
        ],
    )
    .run()
}

pub fn is_running() -> bool {
    TostCmd::new(
        "systemctl",
        vec!["--user", "is-active", "--quiet", "pipewire"],
    )
    .run_status()
}

fn status() {
    if is_running() {
        println!("pipewire is running");
    } else {
        println!("pipewire is not running");
    }
}
