use GameFramework::FrameworkTypes::*;
use GameFramework::Components::ActorComponent::*;
use std::collections::HashMap;

pub(in GameFramework) struct DataManager {
    components: HashMap<usize, Box<Component>>,
    components_typeToGUID: HashMap<FLAG_Components, Vec<usize>>,
    components_entityToGUID: HashMap<usize, Vec<usize>>,
    entities: HashMap<usize, Box<Entity>>,
    activeEntityIDs: Vec<usize>,
    component_counter: usize,
    entity_counter: usize
}

impl DataManager {
    pub fn new() -> DataManager {
        DataManager {
            components: HashMap::new(),
            components_typeToGUID: HashMap::new(),
            components_entityToGUID: HashMap::new(),
            entities: HashMap::new(),
            activeEntityIDs: Vec::new(),
            component_counter: 0,
            entity_counter: 0
        }
    }

    pub fn NewEntity(&mut self, name: &'static str, components: FLAG_Components) {
        self.entities.insert(self.entity_counter, Box::new(Entity::new(name, self.entity_counter, components)));
        self.components_entityToGUID.insert(self.entity_counter, Vec::new());

        if components.intersects(COMPONENT_FLAG_ACTOR) {
            println!("poop");
            self.components.insert(self.component_counter, Box::new(ActorComponent::new(self.component_counter, self.entity_counter)));

            self.components_typeToGUID.entry(COMPONENT_FLAG_ACTOR).or_insert(Vec::new());
            let typeVec = self.components_typeToGUID.get_mut(&COMPONENT_FLAG_ACTOR).unwrap();
            typeVec.push(self.component_counter);

            let entityVec = self.components_entityToGUID.get_mut(&self.entity_counter).unwrap();
            entityVec.push(self.component_counter);
        }

        self.activeEntityIDs.push(self.entity_counter);
        self.entity_counter += 1;
        self.component_counter += 1;
    }

    pub fn GetEntity(&self, key: &usize) -> Option<&Box<Entity>> {
        self.entities.get(key)
    }

    pub fn getComponent_with_attr(&self, entityID: usize, component: FLAG_Components) -> Option<&Box<Component>> {
         match self.components_entityToGUID.get(&entityID) {
            Some(a) => {
                match self.components_typeToGUID.get(&component) {
                    Some(b) => {
                        self.getComponent(a.iter().zip(b.iter()).filter(|&(a, b)| a == b).collect::<Vec<_>>()[0].0)
                    }
                    None => None
                }
            }
            None => None
        }
    }

    pub fn getComponent_with_attr_mut(&mut self, entityID: usize, component: FLAG_Components) -> Option<&mut Box<Component>> {
        let guid: Option<usize> = match self.components_entityToGUID.get(&entityID) {
            Some(a) => {
                match self.components_typeToGUID.get(&component) {
                    Some(b) => {
                        Some(*a.iter().zip(b.iter()).filter(|&(a, b)| a == b).collect::<Vec<_>>()[0].0)
                    }
                    None => None
                }
            }
            None => None
        };

        if let Some(guid) = guid {
            self.getComponent_mut(&guid)
        } else {
            None
        }
    }

    pub fn getComponent(&self, GUID: &usize) -> Option<&Box<Component>> {
        self.components.get(&GUID)
    }

    pub fn getComponent_mut(&mut self, GUID: &usize) -> Option<&mut Box<Component>> {
        self.components.get_mut(&GUID)
    }

    pub fn getAllComponents(&self) -> &HashMap<usize, Box<Component>> {
        &self.components
    }
}
