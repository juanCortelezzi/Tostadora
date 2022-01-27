use clap::{App, Arg, ArgMatches};

use crate::commands::TostCmd;

pub fn get_command() -> App<'static> {
    App::new("light")
        .about("Handles the brightness of the display")
        .arg(
            Arg::new("signal")
                .about("The signal id for the light block in dwmblocks")
                .long("signal")
                .short('s')
                .takes_value(true)
                .default_value("20")
                .global(true),
        )
        .subcommand(
            App::new("set")
                .about("Sets the brightness of the display to the given value")
                .arg(
                    Arg::new("VALUE")
                        .about("Percentage to set the brightness of the display")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(App::new("inc").about("Increases the brightness of the display by 10"))
        .subcommand(App::new("dec").about("Decreases the brightness of the display by 10"))
        .subcommand(
            App::new("get")
                .about("Shows the brightness percentage of the display")
                .arg(
                    Arg::new("format")
                        .about("Pretty display")
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
        Some(("get", args)) => handle_get(args),
        _ => println!("No subcommand was used"),
    }
}

fn handle_inc(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("xbacklight", &["-inc", "10"])
        .add_notify(signal)
        .run()
}

fn handle_dec(args: &ArgMatches) {
    let signal = args.value_of("signal").expect("signal flag not found");
    TostCmd::new("xbacklight", &["-dec", "10"])
        .add_notify(signal)
        .run()
}

fn handle_set(args: &ArgMatches) {
    let value = args.value_of("VALUE").expect("VALUE not given");
    let signal = args.value_of("signal").expect("signal flag not found");

    TostCmd::new("xbacklight", &["-set", value])
        .add_notify(signal)
        .run()
}

fn handle_get(args: &ArgMatches) {
    // Parse brightness from byte array to string
    let bri_parsed = String::from_utf8(TostCmd::new("xbacklight", &["-get"]).run_output().stdout)
        .expect("could not parse command output to utf8");

    let bri = bri_parsed.trim();

    if args.is_present("format") {
        println!("[  {} ]", bri);
    } else {
        println!("bri: {}", bri);
    }
}
