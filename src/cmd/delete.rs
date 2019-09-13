use clap_nested::{file_stem, Command};
use colored::Colorize;
use dialoguer::Confirmation;

use crate::util;

pub fn cmd<'a>() -> Command<'a, str> {
    Command::new(file_stem!())
        .description("Deletes the development Minikube machine")
        .runner(|env, _matches| {
            if env != "dev" {
                panic!("Only supported in \"dev\" environment.");
            }

            if Confirmation::new()
                .with_text(&format!(
                    "Do you really want to {} the Minikube machine?",
                    "delete".red(),
                ))
                .default(false)
                .interact()
                .unwrap()
            {
                util::Command::new("minikube", vec!["--profile=mav", "delete"]).run();
            }
        })
}
