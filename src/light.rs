use crate::config::{CmdConfig, LightConfig};
use clap::{Arg, ArgMatches, Command};

pub fn get_command() -> Command<'static> {
    Command::new("light")
        .about("Handles the brightness of the display")
        .arg(
            Arg::new("signal")
                .long("signal")
                .short('s')
                .takes_value(true)
                .default_value("20")
                .global(true),
        )
        .subcommand(
            Command::new("set")
                .about("Sets the brightness of the display to the given value")
                .arg(Arg::new("VALUE").required(true).index(1)),
        )
        .subcommand(Command::new("inc").about("Increases the brightness of the display by 10"))
        .subcommand(Command::new("dec").about("Decreases the brightness of the display by 10"))
        .subcommand(Command::new("get").about("Shows the brightness percentage of the display"))
}

pub fn handle(args: &ArgMatches, command: LightConfig) {
    match args.subcommand() {
        Some(("inc", args)) => run_command(args, command.inc),
        Some(("dec", args)) => run_command(args, command.dec),
        Some(("set", args)) => run_command(args, command.set),
        Some(("get", args)) => run_command(args, command.get),
        _ => println!("No subcommand was used"),
    }
}

fn run_command(args: &ArgMatches, command: CmdConfig) {
    // let signal = args.value_of("signal").expect("signal flag not found");
    let mut process = std::process::Command::new(command.cmd);

    if let Some(mut arguments) = command.args {
        if let Some(value) = args.value_of("VALUE") {
            arguments.push(value.to_string());
        }

        process.args(arguments);
    }

    process.status().expect("to execute command");
}
