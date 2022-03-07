use clap::{Arg, ArgMatches, Command};

use crate::config::{CmdConfig, ServiceConfig};

pub fn get_commands() -> Vec<Command<'static>> {
    vec![
        Command::new("start")
            .about("Starts the service/s")
            .arg(Arg::new("SERVICE").required(true).index(1)),
        Command::new("stop")
            .about("Stops the service/s")
            .arg(Arg::new("SERVICE").required(true).index(1)),
        Command::new("status")
            .about("Status of the service/s")
            .arg(Arg::new("SERVICE").required(true).index(1)),
    ]
}

fn get_service(args: &ArgMatches, commands: Vec<ServiceConfig>) -> Option<ServiceConfig> {
    let service = args.value_of("SERVICE").unwrap();
    commands.into_iter().find(|s| s.name == service)
}

pub fn handle_start(args: &ArgMatches, commands: Vec<ServiceConfig>) {
    let command = get_service(args, commands);

    if command.is_none() {
        println!("no service named like that");
        return;
    }

    let command = command.unwrap().start;

    run_command(command)
}

pub fn handle_stop(args: &ArgMatches, commands: Vec<ServiceConfig>) {
    let command = get_service(args, commands);

    if command.is_none() {
        println!("no service named like that");
        return;
    }

    let command = command.unwrap().stop;

    run_command(command)
}

pub fn handle_status(args: &ArgMatches, commands: Vec<ServiceConfig>) {
    let command = get_service(args, commands);

    if command.is_none() {
        println!("no service named like that");
        return;
    }

    let command = command.unwrap().status;
    run_command(command)
}

fn run_command(command: CmdConfig) {
    let mut process = std::process::Command::new(command.cmd);

    if let Some(arguments) = command.args {
        process.args(arguments);
    }

    process.status().expect("to execute command");
}
