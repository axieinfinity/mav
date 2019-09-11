use super::commander::Commander;

mod delete;
mod fix;
mod install;
mod start;
mod stop;

pub fn get_commander<'a>() -> Commander<'a, str> {
    Commander::new(vec![
        delete::get_command(),
        fix::get_command(),
        install::get_command(),
        start::get_command(),
        stop::get_command(),
    ])
}
