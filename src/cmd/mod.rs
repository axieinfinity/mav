use super::commander::Commander;

mod install;

pub fn get_commander<'a>(version: &'a str, author: &'a str) -> Commander<'a, String> {
    Commander::new(version, author, vec![install::get_command()])
}
