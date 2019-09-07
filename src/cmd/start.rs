use crate::commander::Command;
use crate::file_stem;
use crate::util;

use super::fix;

const MINIKUBE_CPUS: u16 = 2;
const MINIKUBE_DISK_SIZE: &'static str = "20000mb";
const MINIKUBE_ISO_VERSION: &'static str = "1.3.0";
const MINIKUBE_KUBERNETES_VERSION: &'static str = "1.15.2";
const MINIKUBE_MEMORY: &'static str = "2000mb";

pub fn get_command<'a>() -> Command<'a, str> {
    Command::new(
        file_stem!(),
        "Starts a Minikube machine for development",
        |app| app,
        |env, matches| {
            if env != "dev" {
                panic!("Only supported in \"dev\" environment.");
            }

            match util::get_minikube_status() {
                util::MinikubeStatus::Running => {}

                util::MinikubeStatus::Stopped => {
                    util::Command::new(
                        "minikube",
                        vec!["--profile=mav", "start", "--vm-driver=hyperkit"],
                    )
                    .run();
                }

                util::MinikubeStatus::Unknown => {
                    util::Command::new(
                        "minikube",
                        vec![
                            "--profile=mav",
                            "start",
                            &format!("--cpus={}", MINIKUBE_CPUS),
                            &format!("--disk-size={}", MINIKUBE_DISK_SIZE),
                            &format!(
                                "--iso-url=https://storage.googleapis.com/minikube/iso/minikube-v{}.iso",
                                MINIKUBE_ISO_VERSION,
                            ),
                            &format!("--kubernetes-version=v{}", MINIKUBE_KUBERNETES_VERSION),
                            &format!("--memory={}", MINIKUBE_MEMORY),
                            "--vm-driver=hyperkit",
                        ],
                    )
                    .run();
                }
            }

            util::Command::new("helm", vec!["init"]).run();

            fix::run(env, matches);
        },
    )
}
