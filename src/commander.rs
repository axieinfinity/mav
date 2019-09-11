use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::path::Path;

use clap::{App, ArgMatches, SubCommand};

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

    pub fn command<'b>(&self, author: &'a str) -> App<'b, 'a> {
        (self.cmd)(
            SubCommand::with_name(self.name)
                .about(self.about)
                .author(author),
        )
    }

    pub fn run(&self, args: &T, matches: &ArgMatches<'_>) {
        (self.run)(args, matches)
    }
}

struct AppHelp {
    value: Vec<u8>,
    subcommands: HashMap<String, Box<AppHelp>>,
}

impl AppHelp {
    pub fn from_app(app: &App) -> AppHelp {
        let mut value = vec![];

        app.write_help(&mut value).unwrap();

        let mut help = AppHelp {
            value,
            subcommands: HashMap::new(),
        };

        for app in &app.p.subcommands {
            help.subcommands
                .insert(app.p.meta.name.clone(), Box::new(Self::from_app(app)));
        }

        help
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn subcommand(&self, name: &str) -> Option<&AppHelp> {
        self.subcommands.get(name).map(|b| b.borrow())
    }
}

pub struct Commander<'a, T: ?Sized> {
    cmds: Vec<Command<'a, T>>,
    help: Option<AppHelp>,
}

impl<'a, T: ?Sized> Commander<'a, T> {
    pub fn new(cmds: Vec<Command<'a, T>>) -> Self {
        Commander { cmds, help: None }
    }

    pub fn add_subcommands<'b>(&self, app: App<'b, 'a>) -> App<'b, 'a> {
        self.cmds.iter().fold(app, |app, cmd| {
            let author = app.p.meta.author.unwrap_or_default();
            app.subcommand(cmd.command(author))
        })
    }

    pub fn capture_help<'b>(&mut self, app: &mut App<'b, 'a>) {
        // Infer binary name
        if let Some(name) = env::args_os().next() {
            let path = Path::new(&name);

            if let Some(filename) = path.file_name() {
                if let Some(binary_name) = filename.to_os_string().to_str() {
                    if app.p.meta.bin_name.is_none() {
                        app.p.meta.bin_name = Some(binary_name.to_owned());
                    }
                }
            }
        }

        let mut tmp = vec![];
        // This hack is used to propagate all needed information to subcommands.
        app.p.gen_completions_to(clap::Shell::Bash, &mut tmp);

        self.help = Some(AppHelp::from_app(app));
    }

    pub fn run(&self, args: &T, matches: &ArgMatches<'_>) {
        for cmd in &self.cmds {
            if let Some(matches) = matches.subcommand_matches(cmd.name()) {
                cmd.run(args, matches);
                return;
            }
        }

        panic!("A subcommand was added without an execution handler.");
    }

    pub fn handle_error(&self, err: clap::Error) {
        match err.kind {
            clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => err.exit(),

            _ => {
                let mut msg = err.message;

                if let Some(index) = msg.find("\nUSAGE") {
                    let usage = msg.split_off(index);
                    let mut lines = usage.lines();

                    eprintln!("{}", msg);

                    lines.next();
                    lines.next();

                    if let Some(usage) = lines.next() {
                        let mut usage = usage.to_owned();

                        if let Some(index) = usage.find("[") {
                            usage.truncate(index);
                        }

                        let mut path: Vec<&str> = usage.split_whitespace().collect();

                        if path.len() > 0 {
                            path.remove(0);
                            self.eprintln_help(&path);
                        }
                    }
                } else {
                    eprintln!("{}", msg);
                }
            }
        }
    }

    fn eprintln_help(&self, path: &[&str]) {
        use std::io::Write;

        if let Some(help) = &self.help {
            let mut help = help;

            for &segment in path {
                match help.subcommand(segment) {
                    Some(inner) => help = inner,
                    None => return,
                }
            }

            std::io::stderr().write_all(help.value()).unwrap();
            eprintln!();
        }
    }
}
