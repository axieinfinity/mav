use clap::Arg;

use crate::commander::Command;
use crate::file_stem;
use crate::util;

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
            match util::OS {
                util::Os::MacOs => {
                    util::install_brew_formula_if_needed("hyperkit");
                }

                _ => panic!("OS not supported."),
            }

            println!("env = {}", env);
            println!("{:?}", matches);

            for value in matches.values_of("debug").unwrap() {
                println!("{:?}", value);
            }
        },
    )
}
