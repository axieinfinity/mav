use clap_nested::{file_stem, Command};

use crate::util;

pub fn cmd<'a>() -> Command<'a, str> {
    Command::new(file_stem!())
        .description("Fixes local network for the development Minikube machine")
        .runner(|env, _matches| {
            if env != "dev" {
                panic!("Only supported in \"dev\" environment.");
            }

            match util::OS {
                util::Os::MacOs => {
                    // Add a resolver

                    util::Command::new("sudo", vec!["mkdir", "-p", "/etc/resolver"])
                        .then("sudo", vec!["rm", "-f", "/etc/resolver/mav"])
                        .then(
                            "sudo",
                            vec![
                                "bash",
                                "-c",
                                &vec![
                                    "cat <<EOF >/etc/resolver/mav",
                                    "nameserver 10.96.0.10",
                                    "domain svc.cluster.local",
                                    "options ndots:5",
                                    "EOF",
                                ]
                                .join("\n"),
                            ],
                        )
                        .run();

                    // Add routes

                    if util::get_minikube_status() == util::MinikubeStatus::Running {
                        let netstat = util::Command::new("netstat", vec!["-nr"]).read();
                        let mut netstat = netstat.lines();

                        if netstat
                            .find(|&line| line.starts_with("10.96/12 "))
                            .is_some()
                        {
                            util::Command::new("sudo", vec!["route", "-n", "delete", "10.96/12"])
                                .run();
                        }

                        if netstat.find(|&line| line.starts_with("172.17 ")).is_some() {
                            util::Command::new("sudo", vec!["route", "-n", "delete", "172.17/16"])
                                .run();
                        }

                        let ip = util::get_minikube_ip();

                        util::Command::new("sudo", vec!["route", "-n", "add", "10.96.0.0/12", &ip])
                            .then("sudo", vec!["route", "-n", "add", "172.17.0.0/16", &ip])
                            .run();
                    }

                    // Allow any hosts via involved interfaces

                    let interfaces = util::Command::new("ifconfig", vec!["bridge100"])
                        .pipe("grep", vec!["member"])
                        .pipe("awk", vec!["{print $2}"])
                        .read();

                    for interface in interfaces.lines() {
                        util::Command::new(
                            "sudo",
                            vec!["ifconfig", "bridge100", "-hostfilter", interface],
                        )
                        .run();
                    }
                }

                _ => panic!("OS not supported."),
            }
        })
}
