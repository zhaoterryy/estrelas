extern crate lua;
#[macro_use]
extern crate bitflags;
#[allow(dead_code)]
#[allow(non_snake_case)]
mod Core;
#[allow(non_snake_case)]
mod System;
#[allow(non_snake_case)]
mod GameFramework;

#[allow(unused_must_use)]
fn main() {
    let mut state = lua::State::new();
    state.open_libs();
    state.do_string("print('hello world!')");

    System::Application::Init();
}
