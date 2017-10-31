use GameFramework::FrameworkTypes::*;
use GameFramework::Components::ActorComponent::*;
use std::collections::HashMap;

pub(in GameFramework) struct DataManager {
    Components: HashMap<FLAG_Components, HashMap<usize, Box<Component>>>,
    Entities: HashMap<usize, Box<Entity>>,
    ComponentGUIDCounter: usize,
    EntityGUIDCounter: usize
}

impl DataManager {
    pub fn new() -> DataManager {
        DataManager {
            Components: HashMap::new(),
            Entities: HashMap::new(),
            ComponentGUIDCounter: 0,
            EntityGUIDCounter: 0
        }
    }

    pub fn NewEntity(&mut self, name: &'static str, components: FLAG_Components) {
        self.Entities.insert(self.EntityGUIDCounter, Box::new(Entity::new(name, self.EntityGUIDCounter, components)));

        if components.intersects(COMPONENT_FLAG_ACTOR) {
            println!("poop");
            self.Components.entry(COMPONENT_FLAG_ACTOR).or_insert(HashMap::new());
            let map = self.Components.get_mut(&COMPONENT_FLAG_ACTOR).unwrap();
            map.insert(self.EntityGUIDCounter, Box::new(ActorComponent::new(self.ComponentGUIDCounter, self.EntityGUIDCounter)));
        }

        self.EntityGUIDCounter += 1;
        self.ComponentGUIDCounter += 1;
    }

    pub fn GetEntity(&self, key: &usize) -> Option<&Box<Entity>> {
        self.Entities.get(key)
    }

    pub fn getComponent(&self, entityID: usize, component: FLAG_Components) -> Option<&Box<Component>> {
        match self.Components.get(&component) {
            Some(map) => {
                match map.get(&entityID) {
                    Some(c) => return Some(c),
                    None => None
                }
            },
            None => None
        }
    }

    pub fn getComponent_mut(&mut self, entityID: usize, component: FLAG_Components) -> Option<&mut Box<Component>> {
        match self.Components.get_mut(&component) {
            Some(map) => {
                match map.get_mut(&entityID) {
                    Some(c) => return Some(&mut *c),
                    None => None
                }
            },
            None => None
        }
    }

    pub fn getComponents_mut(&mut self) -> &mut HashMap<FLAG_Components, HashMap<usize, Box<Component>>> {
        &mut self.Components
    }
}
