mod commands;
mod docker;
mod light;
mod pipewire;
mod sound;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Tostadora")
        .version("0.1")
        .author("Juan Bautista Cortelezzi.")
        .about("It can manage processes in the system and control brightness as well as sound, but most importantly, it keeps me entertained with its development :)")
        .license("GPL-v3")
        .subcommand(
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
                ),
        )
        .subcommand(
            App::new("sound")
                .about("Handles the volume of the system")
                .arg(
                    Arg::new("signal")
                        .about("The signal id for the sound block in dwmblocks")
                        .long("signal")
                        .short('s')
                        .takes_value(true)
                        .global(true)
                        .default_value("10"),
                )
                .subcommand(
                    App::new("set")
                        .about("Sets the volume to the given value")
                        .arg(
                            Arg::new("VALUE")
                                .about("Percentage to set the volume of the system")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(App::new("inc").about("Increases volume by 10"))
                .subcommand(App::new("dec").about("Decreases volume by 10"))
                .subcommand(App::new("mute").about("Toggles mute"))
                .subcommand(
                    App::new("get")
                        .about("Shows the volume percentage of the system")
                        .arg(
                            Arg::new("format")
                                .about("Pretty display")
                                .long("format")
                                .short('f')
                                .takes_value(false),
                        ),
                ),
        )
        .subcommand(
            App::new("docker")
                .about("Starts and stops the docker service/s")
                .subcommand(App::new("start").about("Starts docker service/s"))
                .subcommand(App::new("stop").about("Stops docker service/s"))
                .subcommand(App::new("status").about("Status of docker service/s")),
        )
        .subcommand(
            App::new("pipewire")
                .about("Starts and stops the pipewire service/s")
                .subcommand(App::new("start").about("Starts pipewire service/s"))
                .subcommand(App::new("stop").about("Stops pipewire service/s"))
                .subcommand(App::new("status").about("Status of pipewire service/s")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("light", args)) => light::handle(args),
        Some(("sound", args)) => sound::handle(args),
        Some(("docker", args)) => docker::handle(args),
        Some(("pipewire", args)) => pipewire::handle(args),
        _ => println!("No subcommand was used"),
    }
}
