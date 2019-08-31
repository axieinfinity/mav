use crate::commander::Command;
use crate::file_stem;
use crate::util;

use dialoguer::Confirmation;

pub fn get_command<'a>() -> Command<'a, str> {
    Command::new(
        file_stem!(),
        "Stops the development Minikube machine temporarily",
        |app| app,
        |env, _matches| {
            if env != "dev" {
                panic!("Only supported in \"dev\" environment.");
            }

            match util::get_minikube_status() {
                util::MinikubeStatus::Running => {
                    if Confirmation::new()
                        .with_text("Do you really want to stop the Minikube machine?")
                        .default(false)
                        .interact()
                        .unwrap()
                    {
                        util::Command::new("minikube", vec!["--profile=mav", "stop"]).run();
                    }
                }

                util::MinikubeStatus::Stopped | util::MinikubeStatus::Unknown => {}
            }
        },
    )
}
