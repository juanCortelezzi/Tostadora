use clap::ArgMatches;

use crate::commands::run_cmd_notify;

pub fn handle(args: &ArgMatches) {
    match args.subcommand() {
        Some(("inc", args)) => handle_inc(args),
        Some(("dec", args)) => handle_dec(args),
        Some(("set", args)) => handle_set(args),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}

fn handle_inc(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("xbacklight", ["-inc", "10"], signal);
}

fn handle_dec(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("xbacklight", ["-dec", "10"], signal);
}

fn handle_set(args: &ArgMatches) {
    let value = args.value_of("VALUE").expect("VALUE not given");
    let signal = args.value_of("signal").expect("signal flag not found");
    run_cmd_notify("xbacklight", ["-set", value], signal);
}
