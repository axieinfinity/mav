use crate::commander::Command;
use crate::file_stem;
use crate::util;

use colored::Colorize;
use dialoguer::Confirmation;

pub fn get_command<'a>() -> Command<'a, str> {
    Command::new(
        file_stem!(),
        "Deletes the development Minikube machine",
        |app| app,
        |env, _matches| {
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
        },
    )
}
