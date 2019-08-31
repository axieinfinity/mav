use super::commander::Commander;

mod delete;
mod fix;
mod install;
mod start;
mod stop;

pub fn get_commander<'a>(version: &'a str, author: &'a str) -> Commander<'a, str> {
    Commander::new(
        version,
        author,
        vec![
            delete::get_command(),
            fix::get_command(),
            install::get_command(),
            start::get_command(),
            stop::get_command(),
        ],
    )
}
