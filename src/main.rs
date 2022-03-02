mod commands;
mod docker;
mod light;
mod pipewire;
mod sound;

fn main() {
    let matches = clap::Command::new("Tostadora")
        .version("0.1")
        .author("Juan Bautista Cortelezzi.")
        .about("It can manage processes in the system and control brightness as well as sound, but most importantly, it keeps me entertained with its development :)")
        .subcommand(light::get_command())
        .subcommand(sound::get_command())
        .subcommand(docker::get_command())
        .subcommand(pipewire::get_command())
        .get_matches();

    match matches.subcommand() {
        Some(("light", args)) => light::handle(args),
        Some(("sound", args)) => sound::handle(args),
        Some(("docker", args)) => docker::handle(args),
        Some(("pipewire", args)) => pipewire::handle(args),
        _ => println!("No subcommand was used"),
    }
}
