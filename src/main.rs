extern crate lua;
extern crate estrelas_math;
#[macro_use]
extern crate bitflags;
#[allow(dead_code)]
mod core;

#[allow(unused_must_use)]
fn main() {
    let mut state = lua::State::new();
    state.open_libs();
    state.do_string("print('hello world!')");

    core::application::init();
}
