use clap::ArgMatches;

use crate::commands::run_cmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => run_cmd(
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
        ),
        Some(("stop", _)) => run_cmd(
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
        ),
        _ => println!("No subcommand was used"),
    }
}
