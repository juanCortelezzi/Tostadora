use clap::{Arg, ArgMatches, Command};

use crate::commands::TostCmd;

pub fn get_command() -> Command<'static> {
    Command::new("sound")
        .about("Handles the volume of the system")
        .arg(
            Arg::new("signal")
                .long("signal")
                .short('s')
                .takes_value(true)
                .global(true)
                .default_value("10"),
        )
        .subcommand(
            Command::new("set")
                .about("Sets the volume to the given value")
                .arg(Arg::new("VALUE").required(true).index(1)),
        )
        .subcommand(Command::new("inc").about("Increases volume by 10"))
        .subcommand(Command::new("dec").about("Decreases volume by 10"))
        .subcommand(Command::new("mute").about("Toggles mute"))
        .subcommand(
            Command::new("get")
                .about("Shows the volume percentage of the system")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .short('f')
                        .takes_value(false),
                ),
        )
}

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("inc", args)) => handle_inc(args),
        Some(("dec", args)) => handle_dec(args),
        Some(("set", args)) => handle_set(args),
        Some(("mute", args)) => handle_mute(args),
        Some(("get", args)) => handle_get(args),
        _ => println!("No subcommand was used"),
    }
}

fn handle_inc(args: &ArgMatches) {
    // signal is a defined value because it has a default
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", &["--change-volume", "+10"])
        .add_notify(signal)
        .run()
}

fn handle_dec(args: &ArgMatches) {
    // signal is a defined value because it has a default
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", &["--change-volume", "-10"])
        .add_notify(signal)
        .run()
}

fn handle_set(args: &ArgMatches) {
    let value = args.value_of("VALUE").expect("VALUE not given");
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", &["--set-volume", value])
        .add_notify(signal)
        .run()
}

fn handle_mute(args: &ArgMatches) {
    // signal is a defined value because it has a default
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", &["--toggle-mute"])
        .add_notify(signal)
        .run()
}

fn handle_get(args: &ArgMatches) {
    let needs_format = args.is_present("format");
    match crate::pipewire::is_running() {
        true => handle_get_running(needs_format),
        false => handle_get_sleeping(needs_format),
    }
}

fn handle_get_running(needs_format: bool) {
    // Parse volume from byte array to string
    let vol_parsed = String::from_utf8(
        TostCmd::new("pulsemixer", &["--get-volume"])
            .run_output()
            .stdout,
    )
    .expect("could not parse command output to utf8");

    // Parse string to single volume value, previous "50 50" (example)
    let (volume, _) = vol_parsed.trim().split_once(' ').unwrap();

    // Parse mute from "0" or "1" to bool
    let muted = match String::from_utf8(
        TostCmd::new("pulsemixer", &["--get-mute"])
            .run_output()
            .stdout,
    )
    .expect("could not parse command output to utf8")
    .trim()
    {
        "0" => false,
        "1" => true,
        _ => panic!("could not parse utf8 to bool"),
    };

    if !needs_format {
        println!("vol: {} - mut: {} - ser: running", volume, muted);
        return;
    }

    if muted {
        println!("[ ??? -1 ]");
    } else {
        println!("[ ??? {} ]", volume);
    }
}

fn handle_get_sleeping(needs_format: bool) {
    if needs_format {
        println!("[ ??? NS ]");
        return;
    }
    println!("vol: -1 - mut: -1 - ser: sleeping",);
}
