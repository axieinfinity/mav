use clap::Arg;
use clap_nested::Commander;

mod delete;
mod fix;
mod install;
mod start;
mod stop;
mod up;

pub fn commander<'a>() -> Commander<'a, (), str> {
    Commander::new()
        .options(|app| {
            app.arg(
                Arg::with_name("environment")
                    .short("e")
                    .long("env")
                    .global(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Sets an environment, defaults to \"dev\""),
            )
        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(delete::cmd())
        .add_cmd(fix::cmd())
        .add_cmd(install::cmd())
        .add_cmd(start::cmd())
        .add_cmd(stop::cmd())
        .add_cmd(up::cmd())
}
