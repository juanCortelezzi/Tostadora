use clap::ArgMatches;

use crate::commands::run_cmd_notify;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("inc", args)) => handle_inc(args),
        Some(("dec", args)) => handle_dec(args),
        Some(("set", args)) => handle_set(args),
        Some(("mute", args)) => handle_mute(args),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}

fn handle_inc(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("pulsemixer", ["--change-volume", "+10"], signal);
}

fn handle_dec(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("pulsemixer", ["--change-volume", "-10"], signal);
}

fn handle_set(args: &ArgMatches) {
    let value = args.value_of("VALUE").expect("VALUE not given");
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("pulsemixer", ["--set-volume", value], signal);
}

fn handle_mute(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("pulsemixer", ["--toggle-mute"], signal);
}
