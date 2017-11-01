extern crate time;

use self::time::precise_time_ns;
use super::world_system::WorldSystem;

pub(crate) fn init() {
    let mut w_systems: Vec<WorldSystem> = Vec::new();
    w_systems.push(WorldSystem::new("world0"));
    main_loop(w_systems);
}

fn main_loop(mut w_systems: Vec<WorldSystem>) {
    const MS_PER_UPDATE: u64 = 16;
    let mut previous = precise_time_ns();
    let mut delay = 0u64;

    loop {
        let current = precise_time_ns();
        let elapsed = current - previous;
        previous = current;
        delay += elapsed;

        while delay >= MS_PER_UPDATE {
            delay -= MS_PER_UPDATE;
            for w in w_systems.iter_mut() {
                update(&mut *w);
            }
        }
    }
}

fn update(ws: &mut WorldSystem) {
    ws.update();
}
