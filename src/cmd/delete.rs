use crate::commander::Command;
use crate::file_stem;
use crate::util;

pub fn get_command<'a>() -> Command<'a, str> {
    Command::new(
        file_stem!(),
        "Deletes Minikube machine",
        |app| app,
        |env, _matches| {
            if env != "dev" {
                panic!("Only supported in \"dev\" environment.");
            }

            util::Command::new("minikube", vec!["--profile=mav", "delete"]).run();
        },
    )
}
