use clap::ArgMatches;

use crate::commands::TostCmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("inc", args)) => handle_inc(args),
        Some(("dec", args)) => handle_dec(args),
        Some(("set", args)) => handle_set(args),
        Some(("mute", args)) => handle_mute(args),
        _ => println!("No subcommand was used"),
    }
}

fn handle_inc(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", vec!["--change-volume", "+10"])
        .add_notify(signal)
        .run()
}

fn handle_dec(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", vec!["--change-volume", "-10"])
        .add_notify(signal)
        .run()
}

fn handle_set(args: &ArgMatches) {
    let value = args.value_of("VALUE").expect("VALUE not given");
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", vec!["--set-volume", value])
        .add_notify(signal)
        .run()
}

fn handle_mute(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("pulsemixer", vec!["--toggle-mute"])
        .add_notify(signal)
        .run()
}
