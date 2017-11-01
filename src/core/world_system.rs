use super::engine_types::game_framework::*;

pub struct WorldSystem {
    world: World
}

impl WorldSystem {
    pub fn new(name: &'static str) -> Self {
        WorldSystem {
            world: World { 
                name: name.to_string(),
                systems: Vec::new(),
                dm: DataManager::new()
            }
        }
    }

    pub(in core) fn update(&mut self) {
        for sys in &self.world.systems {
            sys.tick(&mut self.world.dm);
        }
    }
}
