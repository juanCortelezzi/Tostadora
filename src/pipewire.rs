use crate::commands::TostCmd;
use clap::{App, ArgMatches};

pub fn get_command() -> App<'static> {
    App::new("pipewire")
        .about("Starts and stops the pipewire service/s")
        .subcommand(App::new("start").about("Starts pipewire service/s"))
        .subcommand(App::new("stop").about("Stops pipewire service/s"))
        .subcommand(App::new("status").about("Status of pipewire service/s"))
}

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("start", _)) => start(),
        Some(("stop", _)) => stop(),
        Some(("status", _)) => status(),
        _ => println!("No subcommand was used"),
    };
}

fn start() {
    // systemctl --user start pipewire.socket pipewire-pulse.socket pipewire wireplumber pipewire-pulse
    TostCmd::new(
        "systemctl",
        &[
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
    // systemctl --user stop pipewire.socket pipewire-pulse.socket pipewire wireplumber pipewire-pulse
    TostCmd::new(
        "systemctl",
        &[
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
    // systemctl --user is-active --quit pipewire
    TostCmd::new("systemctl", &["--user", "is-active", "--quiet", "pipewire"]).run_status()
}

fn status() {
    if is_running() {
        println!("pipewire is running");
    } else {
        println!("pipewire is not running");
    }
}
