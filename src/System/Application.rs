extern crate time;

use self::time::precise_time_ns;
use GameFramework::Systems::WorldSystem::*;

pub(crate) fn Init() {
    let mut wSystems: Vec<WorldSystem> = Vec::new();
    wSystems.push(WorldSystem::test());
    Mainloop(wSystems);
}

fn Mainloop(mut wSystems: Vec<WorldSystem>) {
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
            for w in wSystems.iter_mut() {
                Update(&mut *w);
            }
        }
    }
}

fn Update(ws: &mut WorldSystem) {
    ws.Update();
}