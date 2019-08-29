extern crate clap;

use clap::{App, Arg};

mod cmd;
mod commander;
mod util;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    let app = App::new(name)
        .version(version)
        .about("An one-stop tool to manage Kubernetes infrastructures")
        .author(authors)
        .arg(
            Arg::with_name("environment")
                .short("e")
                .long("env")
                .takes_value(true)
                .value_name("STRING")
                .help("Sets an environment, defaults to \"dev\""),
        );

    let commander = cmd::get_commander(version, authors);
    let app = commander.add_subcommands(app);

    let matches = app.get_matches();
    let env = matches.value_of("environment").unwrap_or("dev").to_owned();

    commander.run(env, matches);
}
