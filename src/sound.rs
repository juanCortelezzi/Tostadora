use crate::config::{CmdConfig, SoundConfig};
use clap::{Arg, ArgMatches, Command};

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
        .subcommand(Command::new("get").about("Shows the volume percentage of the system"))
}

pub fn handle(args: &ArgMatches, command: SoundConfig) {
    match args.subcommand() {
        Some(("inc", _args)) => run_command(command.inc, None),
        Some(("dec", _args)) => run_command(command.dec, None),
        Some(("get", _args)) => run_command(command.get, None),
        Some(("mute", _args)) => run_command(command.mute, None),
        Some(("set", args)) => run_command(command.set, args.value_of("VALUE")),
        _ => println!("No subcommand was used"),
    }
}

fn run_command(command: CmdConfig, value: Option<&str>) {
    // let signal = args.value_of("signal").expect("signal flag not found");
    let mut process = std::process::Command::new(command.cmd);

    if let Some(mut arguments) = command.args {
        if let Some(value) = value {
            arguments.push(value.to_string());
        }

        process.args(arguments);
    }

    process.status().expect("to execute command");

    if let Some(signal) = command.signal {
        std::process::Command::new("pkill")
            .arg(format!("-RTMIN+{}", signal))
            .arg("dwmblocks")
            .status()
            .expect("to execute command");
    }
}
