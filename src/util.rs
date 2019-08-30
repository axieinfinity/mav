use std::ffi::{OsStr, OsString};

use colored::Colorize;
use duct::{Expression, ToExecutable};
use std::fmt::Display;
use which::which;

#[macro_export]
macro_rules! file_stem {
    () => {
        std::path::Path::new(file!())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    };
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Os {
    MacOs,
    Linux,
    Windows,
    Other,
}

#[cfg(target_os = "macos")]
pub const OS: Os = Os::MacOs;
#[cfg(target_os = "linux")]
pub const OS: Os = Os::Linux;
#[cfg(target_os = "windows")]
pub const OS: Os = Os::Windows;
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub const OS: Os = Os::Other;

pub fn command_exists<C: AsRef<OsStr>>(cmd: C) -> bool {
    which(cmd).is_ok()
}

pub struct Command {
    exp: Expression,
}

impl Command {
    pub fn new<P, A, I>(program: P, args: A) -> Self
    where
        P: ToExecutable,
        A: IntoIterator<Item = I>,
        I: Into<OsString>,
    {
        Command {
            exp: duct::cmd(program, args),
        }
    }

    pub fn run(&mut self) {
        let output = self.pipe_all().exp.unchecked().run().unwrap();

        if !output.status.success() {
            std::process::exit(output.status.code().unwrap_or(128));
        }
    }

    pub fn read(&mut self) -> String {
        self.pipe_stderr().exp.read().unwrap()
    }

    fn pipe_stdout(&mut self) -> &mut Self {
        self.exp = self.exp.stdout_handle(os_pipe::dup_stdout().unwrap());
        self
    }

    fn pipe_stderr(&mut self) -> &mut Self {
        self.exp = self.exp.stderr_handle(os_pipe::dup_stderr().unwrap());
        self
    }

    fn pipe_all(&mut self) -> &mut Self {
        self.pipe_stdout().pipe_stderr()
    }
}

pub fn check_install<C: AsRef<OsStr> + Copy + Display>(cmd: C) -> bool {
    print!(
        "{}{}{}",
        "Checking if ".yellow(),
        cmd,
        " is installed..".yellow()
    );

    let installed = command_exists(cmd);

    println!(
        " {}",
        if installed {
            "✓".green()
        } else {
            "✘".red()
        }
    );

    installed
}

pub fn install_brew_if_needed() {
    if !check_install("brew") {
        println!("Installing brew..");

        let script = Command::new(
            "curl",
            vec![
                "-fsSL",
                "https://raw.githubusercontent.com/Homebrew/install/master/install",
            ],
        )
        .read();

        Command::new("ruby", vec!["-e", &script]).run();
    }
}

pub fn install_brew_formulae_if_needed<F, I>(formulae: F)
where
    F: IntoIterator<Item = I>,
    I: AsRef<OsStr> + Into<OsString> + Copy + Display,
{
    let mut args = vec!["install".into()];

    for formula in formulae {
        if !check_install(formula) {
            args.push(formula.into());
        }
    }

    if args.len() > 1 {
        install_brew_if_needed();
        Command::new("brew", args).run();
    }
}

pub fn install_brew_formula_if_needed<F: AsRef<OsStr> + Into<OsString> + Copy + Display>(
    formula: F,
) {
    install_brew_formulae_if_needed(vec![formula])
}
