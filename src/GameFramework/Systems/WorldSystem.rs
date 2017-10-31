use GameFramework::FrameworkTypes::*;
use super::DataManager::*;
use super::TestSystem::*;

pub(crate) struct WorldSystem {
    pub(in GameFramework) World: World,
    pub(in GameFramework) DataManager: DataManager
}

impl WorldSystem {
    pub fn new() -> Self {
        WorldSystem {
            World: World { 
                Name: "poop".to_string(),
                Systems: Vec::new(),
                Entities: Vec::new()
            },
            DataManager: DataManager::new()
        }
    }

    #[allow(dead_code)]
    pub fn test() -> Self {
        let mut dm: DataManager = DataManager::new();
        dm.NewEntity("pooptester", COMPONENT_FLAG_ACTOR);
        WorldSystem {
            World: World {
                Name: "pooptester".to_string(),
                Systems: vec![
                    Box::new(TestSystem)
                ],
                Entities: Vec::new()
            },
            DataManager: dm
        }
    }

    pub fn Update(&mut self) {
        for System in &self.World.Systems {
            System.Tick(&mut self.DataManager);
        }
    }
}
