extern crate lua;
#[allow(dead_code)]
#[allow(non_snake_case)]
mod Core;

#[allow(unused_must_use)]
fn main() {
    let mut state = lua::State::new();
    state.open_libs();
    state.do_string("print('hello world!')");
    {
        use Core::Math::CommonMath::*;
        use Core::Math::Vector2::*;
        println!("{}", KINDA_SMALL_NUMBER);
        let vec = Vector2::new(5.0, 5.0);
        let vec2 = Vector2 { x: 2.0, y: 3.0 };
        println!("{}", vec + vec2);

        println!("{}", invsqrt(5));
        println!("{}", vec != Vector2::new(5.1, 5.0));

        println!("{}", vec.UnitVector());
    }

}
