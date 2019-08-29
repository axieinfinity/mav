use clap::Arg;

use crate::commander::Command;
use crate::file_stem;

pub fn get_command<'a>() -> Command<'a, String> {
    Command::new(
        file_stem!(),
        "Installs all components",
        |cmd| {
            cmd.arg(
                Arg::with_name("debug")
                    .multiple(true)
                    .short("d")
                    .long("debug")
                    .takes_value(true)
                    .help("Prints debug information verbosely"),
            )
        },
        |env, matches| {
            println!("env = {}", env);
            println!("{:?}", matches);

            for value in matches.values_of("debug").unwrap() {
                println!("{:?}", value);
            }
        },
    )
}
