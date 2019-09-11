#![feature(async_closure)]

use clap::{App, AppSettings, Arg};

mod cmd;
mod commander;
mod util;

fn main() {
    let app = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .global_setting(AppSettings::GlobalVersion)
        .arg(
            Arg::with_name("environment")
                .short("e")
                .long("env")
                .global(true)
                .takes_value(true)
                .value_name("STRING")
                .help("Sets an environment, defaults to \"dev\""),
        );

    let mut commander = cmd::get_commander();
    let mut app = commander.add_subcommands(app);

    commander.capture_help(&mut app);

    match app.get_matches_safe() {
        Ok(matches) => {
            let env = matches.value_of("environment").unwrap_or("dev");
            commander.run(env, &matches);
        }

        Err(err) => match err.kind {
            clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => err.exit(),
            _ => commander.handle_error(err),
        },
    }
}
