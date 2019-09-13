use std::cmp;
use std::ffi::{OsStr, OsString};
use std::os::unix::fs::PermissionsExt;
use std::{fmt, fs};

use colored::Colorize;
use duct::{Expression, ToExecutable};
use futures::compat::{Compat, Future01CompatExt, Stream01CompatExt};
use futures::{stream, Future, FutureExt, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::r#async::Client;
use tokio::fs::File;
use which::which;

pub fn tokio_run<F: Future<Output = ()> + Send + 'static>(future: F) {
    tokio::run(Compat::new(Box::pin(
        future.map(|()| -> Result<(), ()> { Ok(()) }),
    )));
}

pub fn tokio_spawn<F: Future<Output = ()> + Send + 'static>(future: F) {
    tokio::spawn(Compat::new(Box::pin(
        future.map(|()| -> Result<(), ()> { Ok(()) }),
    )));
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

#[derive(Clone, Debug)]
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

    pub fn then<P, A, I>(mut self, program: P, args: A) -> Self
    where
        P: ToExecutable,
        A: IntoIterator<Item = I>,
        I: Into<OsString>,
    {
        self.exp = self.exp.then(duct::cmd(program, args));
        self
    }

    pub fn pipe<P, A, I>(mut self, program: P, args: A) -> Self
    where
        P: ToExecutable,
        A: IntoIterator<Item = I>,
        I: Into<OsString>,
    {
        self.exp = self.exp.pipe(duct::cmd(program, args));
        self
    }

    fn pipe_stdout(mut self) -> Self {
        self.exp = self.exp.stdout_handle(os_pipe::dup_stdout().unwrap());
        self
    }

    fn pipe_stderr(mut self) -> Self {
        self.exp = self.exp.stderr_handle(os_pipe::dup_stderr().unwrap());
        self
    }

    fn pipe_all(self) -> Self {
        self.pipe_stdout().pipe_stderr()
    }

    pub fn run(self) {
        let output = self.pipe_all().exp.unchecked().run().unwrap();

        if !output.status.success() {
            std::process::exit(output.status.code().unwrap_or(128));
        }
    }

    pub fn read(self) -> String {
        self.pipe_stderr().exp.read().unwrap()
    }

    pub fn read_unchecked(self) -> String {
        self.pipe_stderr().exp.unchecked().read().unwrap()
    }
}

pub fn check_install<C: AsRef<OsStr> + fmt::Display>(cmd: C) -> bool {
    print!(
        "{} {} {}",
        "Checking if".yellow(),
        cmd,
        "is installed..".yellow()
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

pub fn install_brew() {
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

        println!("Homebrew {}", "is installed successfully.".green());
    }
}

pub fn install_brew_formula<F: AsRef<OsStr> + Clone + fmt::Display>(formula: F) {
    install_brew_formulae(vec![formula])
}

pub fn install_brew_formulae<F, I>(formulae: F)
where
    F: IntoIterator<Item = I>,
    I: AsRef<OsStr> + Clone + fmt::Display,
{
    let mut formulae_to_install = vec![];
    let mut args = vec!["install".into()];

    for formula in formulae {
        if !check_install(&formula) {
            formulae_to_install.push(formula.clone());
            args.push(formula.as_ref().to_owned());
        }
    }

    if args.len() > 1 {
        install_brew();
        Command::new("brew", args).run();

        for formula in formulae_to_install {
            println!(
                "{} {} {}",
                "Homebrew formula".green(),
                formula,
                "is installed successfully.".green(),
            );
        }
    }
}

pub fn install_by_downloading<C, U>(cmd: C, url: U) -> DownloadingInstaller
where
    C: Into<String>,
    U: Into<String>,
{
    DownloadingInstaller::new().enqueue(cmd, url)
}

#[derive(Debug)]
pub struct DownloadingInstaller {
    items: Vec<DownloadedItem>,
}

#[derive(Debug)]
struct DownloadedItem {
    cmd: String,
    url: String,
    postinstall: Option<Command>,
}

impl DownloadedItem {
    pub fn new(cmd: String, url: String, postinstall: Option<Command>) -> Self {
        DownloadedItem {
            cmd,
            url,
            postinstall,
        }
    }
}

impl Clone for DownloadedItem {
    fn clone(&self) -> Self {
        DownloadedItem::new(self.cmd.clone(), self.url.clone(), self.postinstall.clone())
    }
}

impl DownloadingInstaller {
    pub fn new() -> Self {
        DownloadingInstaller { items: vec![] }
    }

    pub fn enqueue<C, U>(self, cmd: C, url: U) -> Self
    where
        C: Into<String>,
        U: Into<String>,
    {
        self.enqueue_with_postinstall(cmd, url, None)
    }

    pub fn enqueue_with_postinstall<C, U, P>(mut self, cmd: C, url: U, postinstall: P) -> Self
    where
        C: Into<String>,
        U: Into<String>,
        P: Into<Option<Command>>,
    {
        let cmd = cmd.into();

        if !check_install(&cmd) {
            let item = DownloadedItem::new(cmd, url.into(), postinstall.into());
            self.items.push(item);
        }

        self
    }

    pub fn run(self) {
        tokio_run(async move {
            let progress = MultiProgress::new();

            let style = ProgressStyle::default_bar()
                .template("{spinner:.green} {msg:10} [{bar:40.cyan/blue}] {bytes}/{total_bytes} (ETA: {eta})")
                .progress_chars("#>-");

            let mut items = vec![];

            for item in &self.items {
                let bar = ProgressBar::new(1);
                let bar = progress.add(bar);
                bar.set_style(style.clone());
                bar.set_message(&item.cmd[..cmp::min(10, item.cmd.len())]);

                items.push((item.to_owned(), bar));
            }

            tokio_spawn(
                stream::iter(items).for_each_concurrent(4, async move |(item, bar)| {
                    let res = Client::new().get(&item.url).send().compat().await.unwrap();
                    let total_size = res.content_length().unwrap_or_default();

                    bar.set_length(total_size);

                    let mut body = res.into_body().compat();
                    let mut file = File::create(item.cmd.clone()).compat().await.unwrap();

                    let mut downloaded_size = 0u64;

                    while let Some(Ok(chunk)) = body.next().await {
                        downloaded_size += chunk.len() as u64;
                        file = tokio::io::write_all(file, chunk).compat().await.unwrap().0;
                        bar.set_position(downloaded_size);
                    }

                    if let Some(postinstall) = item.postinstall {
                        postinstall.run();
                    } else {
                        fs::set_permissions(&item.cmd, fs::Permissions::from_mode(0o755)).unwrap();
                        fs::rename(&item.cmd, format!("/usr/local/bin/{}", item.cmd)).unwrap();
                    }

                    bar.finish_and_clear();
                }),
            );

            progress.join_and_clear().unwrap();

            for item in &self.items {
                println!("{} {}", item.cmd, "is installed successfully.".green());
            }
        });
    }
}

#[derive(PartialEq, Debug)]
pub enum MinikubeStatus {
    Running,
    Stopped,
    Unknown,
}

pub fn get_minikube_status() -> MinikubeStatus {
    let status = Command::new("minikube", vec!["--profile=mav", "status"]).read_unchecked();
    let status = status.lines().next().unwrap_or_default();
    let status = status.split_whitespace().take(2).last().unwrap_or_default();

    match status {
        "Running" => MinikubeStatus::Running,
        "Stopped" => MinikubeStatus::Stopped,
        _ => MinikubeStatus::Unknown,
    }
}

pub fn get_minikube_ip() -> String {
    Command::new("minikube", vec!["--profile=mav", "ip"]).read()
}
