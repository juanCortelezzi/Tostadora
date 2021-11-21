use clap::ArgMatches;

use crate::commands::TostCmd;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("inc", args)) => handle_inc(args),
        Some(("dec", args)) => handle_dec(args),
        Some(("set", args)) => handle_set(args),
        Some(("get", args)) => handle_get(args),
        _ => println!("No subcommand was used"),
    }
}

fn handle_inc(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("xbacklight", vec!["-inc", "10"])
        .add_notify(signal)
        .run()
}

fn handle_dec(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("xbacklight", vec!["-dec", "10"])
        .add_notify(signal)
        .run()
}

fn handle_set(args: &ArgMatches) {
    let value = args.value_of("VALUE").expect("VALUE not given");
    let signal = args.value_of("signal").expect("signal flag not found");

    TostCmd::new("xbacklight", vec!["-set", value])
        .add_notify(signal)
        .run()
}

fn handle_get(args: &ArgMatches) {
    // Parse brightness from byte array to string
    let bri_parsed =
        String::from_utf8(TostCmd::new("xbacklight", vec!["-get"]).run_output().stdout)
            .expect("could not parse command output to utf8");

    let bri = bri_parsed.trim();

    if args.is_present("format") {
        println!("[  {} ]", bri);
    } else {
        println!("bri: {}", bri);
    }
}
