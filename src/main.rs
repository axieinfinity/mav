#![feature(async_closure)]

#[macro_use]
extern crate clap;

use clap::Arg;

mod cmd;
mod commander;
mod util;

fn main() {
    let app = clap::app_from_crate!().arg(
        Arg::with_name("environment")
            .short("e")
            .long("env")
            .takes_value(true)
            .value_name("STRING")
            .help("Sets an environment, defaults to \"dev\""),
    );

    let commander = cmd::get_commander();
    let app = commander.add_subcommands(app);

    let matches = app.get_matches();
    let env = matches.value_of("environment").unwrap_or("dev");

    commander.run(env, &matches);
}
