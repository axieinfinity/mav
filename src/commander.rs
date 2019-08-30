use clap::{App, ArgMatches, SubCommand};

pub struct Command<'a, T> {
    name: &'a str,
    about: &'a str,
    cmd: Box<dyn for<'x, 'y> Fn(App<'x, 'y>) -> App<'x, 'y> + 'a>,
    run: Box<dyn Fn(&T, &ArgMatches<'_>) -> () + 'a>,
}

impl<'a, T> Command<'a, T> {
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

    pub fn command<'b>(&'a self, version: &'b str, author: &'b str) -> App<'a, 'b> {
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

pub struct Commander<'a, T> {
    version: &'a str,
    author: &'a str,
    cmds: Vec<Command<'a, T>>,
}

impl<'a, T> Commander<'a, T> {
    pub fn new(version: &'a str, author: &'a str, cmds: Vec<Command<'a, T>>) -> Self {
        Commander {
            version,
            author,
            cmds,
        }
    }

    pub fn add_subcommands<'b>(&'a self, app: App<'a, 'b>) -> App<'a, 'b> {
        self.cmds.iter().fold(app, |app, cmd| {
            app.subcommand(cmd.command(self.version, self.author))
        })
    }

    pub fn run(&self, args: T, matches: ArgMatches<'_>) {
        for cmd in &self.cmds {
            if let Some(matches) = matches.subcommand_matches(cmd.name()) {
                cmd.run(&args, matches);
                break;
            }
        }
    }
}
