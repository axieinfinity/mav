use clap::{App, ArgMatches, SubCommand};
use colored::Colorize;

pub struct Command<'a, T: ?Sized> {
    name: &'a str,
    about: &'a str,
    cmd: Box<dyn for<'x, 'y> Fn(App<'x, 'y>) -> App<'x, 'y> + 'a>,
    run: Box<dyn Fn(&T, &ArgMatches<'_>) -> () + 'a>,
}

impl<'a, T: ?Sized> Command<'a, T> {
    pub fn new<N, A, C, R>(name: N, about: A, cmd: C, run: R) -> Self
    where
        N: Into<&'a str>,
        A: Into<&'a str>,
        C: for<'x, 'y> Fn(App<'x, 'y>) -> App<'x, 'y> + 'a,
        R: Fn(&T, &ArgMatches<'_>) -> () + 'a,
    {
        Command {
            name: name.into(),
            about: about.into(),
            cmd: Box::new(cmd),
            run: Box::new(run),
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn command<'b>(&self, version: &'a str, author: &'a str) -> App<'b, 'a> {
        (self.cmd)(
            SubCommand::with_name(self.name)
                .version(version)
                .about(self.about)
                .author(author),
        )
    }

    pub fn run(&self, args: &T, matches: &ArgMatches<'_>) {
        (self.run)(args, matches)
    }
}

pub struct Commander<'a, T: ?Sized> {
    cmds: Vec<Command<'a, T>>,
}

impl<'a, T: ?Sized> Commander<'a, T> {
    pub fn new(cmds: Vec<Command<'a, T>>) -> Self {
        Commander { cmds }
    }

    pub fn add_subcommands<'b>(&self, app: App<'b, 'a>) -> App<'b, 'a> {
        self.cmds.iter().fold(app, |app, cmd| {
            let version = app.p.meta.version.unwrap_or_default();
            let author = app.p.meta.author.unwrap_or_default();
            app.subcommand(cmd.command(version, author))
        })
    }

    pub fn run(&self, args: &T, matches: &ArgMatches<'_>) {
        for cmd in &self.cmds {
            if let Some(matches) = matches.subcommand_matches(cmd.name()) {
                cmd.run(args, matches);
                return;
            }
        }

        println!(
            "{}\n\nFor more information try {}",
            matches.usage(),
            "--help".green()
        );
    }
}
