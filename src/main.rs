#![feature(async_closure)]

mod cmd;
mod util;

fn main() {
    cmd::commander().run(&());
}
