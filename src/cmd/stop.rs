use crate::commander::Command;
use crate::file_stem;
use crate::util;

pub fn get_command<'a>() -> Command<'a, str> {
    Command::new(
        file_stem!(),
        "Stops Minikube temporarily",
        |app| app,
        |env, _matches| {
            if env != "dev" {
                panic!("Only supported in \"dev\" environment.");
            }

            match util::get_minikube_status() {
                util::MinikubeStatus::Running => {
                    util::Command::new("minikube", vec!["--profile=mav", "stop"]).run();
                }

                util::MinikubeStatus::Stopped | util::MinikubeStatus::Unknown => {}
            }
        },
    )
}
