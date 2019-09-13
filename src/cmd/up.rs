use clap_nested::{file_stem, Command, CommandLike, Commander};

pub fn pod<'a>() -> Command<'a, str> {
    Command::new("pod")
        .description("Spins up pods")
        .runner(|env, matches| {
            println!("pod: {}, {:?}", env, matches);
        })
}

pub fn service<'a>() -> Command<'a, str> {
    Command::new("service")
        .description("Spins up services")
        .runner(|env, matches| {
            println!("service: {}, {:?}", env, matches);
        })
}

pub fn cmd() -> impl CommandLike<str> {
    Commander::new()
        .add_cmd(pod())
        .add_cmd(service())
        .into_cmd(file_stem!(), "Spins up stuff")
}
