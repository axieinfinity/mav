use crate::commander::Command;
use crate::file_stem;
use crate::util;

const KUBECTL_VERSION: &'static str = "1.15.3";
const MINIKUBE_VERSION: &'static str = "1.3.1";
const HELM_VERSION: &'static str = "2.14.3";
const HELMFILE_VERSION: &'static str = "0.82.0";

pub fn get_command<'a>() -> Command<'a, str> {
    Command::new(
        file_stem!(),
        "Installs all prerequisites",
        |app| app,
        |env, _matches| {
            let platform = match util::OS {
                util::Os::MacOs => "darwin",
                util::Os::Linux => "linux",
                _ => panic!("OS not supported."),
            };

            let kubectl_url = format!(
                "https://storage.googleapis.com/kubernetes-release/release/v{}/bin/{}/amd64/kubectl",
                KUBECTL_VERSION,
                platform,
            );

            let docker_machine_driver_hyperkit_url = format!(
                "https://storage.googleapis.com/minikube/releases/v{}/docker-machine-driver-hyperkit",
                MINIKUBE_VERSION,
            );

            let minikube_url = format!(
                "https://storage.googleapis.com/minikube/releases/v{}/minikube-{}-amd64",
                MINIKUBE_VERSION, platform,
            );

            let helm_url = format!(
                "https://get.helm.sh/helm-v{}-{}-amd64.tar.gz",
                HELM_VERSION, platform,
            );

            let helmfile_url = format!(
                "https://github.com/roboll/helmfile/releases/download/v{}/helmfile_{}_amd64",
                HELMFILE_VERSION, platform,
            );

            match env {
                "dev" => {
                    match util::OS {
                        util::Os::MacOs => {
                            util::install_brew_formula("hyperkit");
                        }

                        _ => panic!("OS not supported."),
                    }

                    let helm_location: &str = &format!("{}-amd64/helm", platform);

                    util::install_by_downloading("kubectl", kubectl_url)
                        .enqueue_with_postinstall(
                            "docker-machine-driver-hyperkit",
                            docker_machine_driver_hyperkit_url,
                            util::Command::new(
                                "sudo",
                                vec![
                                    "install",
                                    "-o",
                                    "root",
                                    "-m",
                                    "4755",
                                    "docker-machine-driver-hyperkit",
                                    "/usr/local/bin/",
                                ],
                            )
                            .then("rm", vec!["-f", "docker-machine-driver-hyperkit"]),
                        )
                        .enqueue("minikube", minikube_url)
                        .enqueue_with_postinstall(
                            "helm",
                            helm_url,
                            util::Command::new("tar", vec!["-xzvf", "helm"])
                                .then("mv", vec![helm_location, "/usr/local/bin/"])
                                .then("rm", vec!["-rf", &format!("{}-amd64", platform)])
                                .then("rm", vec!["-f", "helm"]),
                        )
                        .enqueue("helmfile", helmfile_url)
                        .run();
                }

                _ => {
                    util::install_by_downloading("kubectl", kubectl_url).run();
                }
            }
        },
    )
}
