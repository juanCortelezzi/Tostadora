mod config;
mod light;
mod services;
mod sound;

fn main() {
    let matches = clap::Command::new("Tostadora")
        .version("0.1")
        .author("Juan Bautista Cortelezzi.")
        .about("It can manage processes in the system and control brightness as well as sound, but most importantly, it keeps me entertained with its development :)")
        .subcommand(light::get_command())
        .subcommand(sound::get_command())
        .subcommands(services::get_commands())
        .get_matches();

    let config_file = config::get_config();

    match matches.subcommand() {
        Some(("light", args)) => light::handle(args, config_file.light),
        Some(("sound", args)) => sound::handle(args, config_file.sound),
        Some(("start", args)) => services::handle_start(args, config_file.services),
        Some(("stop", args)) => services::handle_stop(args, config_file.services),
        Some(("status", args)) => services::handle_status(args, config_file.services),
        _ => println!("No subcommand was used"),
    }
}

// fn notify_dwm(&self) {
//     if let Some(signal) = self.needs_notify {
//         Command::new("pkill")
//             .arg(format!("-RTMIN+{}", signal))
//             .arg("dwmblocks")
//             .status()
//             .expect("to execute command");
//     }
// }
