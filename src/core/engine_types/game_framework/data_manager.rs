use super::*;
use std::collections::HashMap;
use std::any::TypeId;

pub struct DataManager<'a> {
    components: HashMap<usize, Box<Component>>,
    components_type_to_guid: HashMap<u64, Vec<usize>>,
    components_entity_to_guid: HashMap<usize, Vec<usize>>,
    typeid_to_new_component_fn: HashMap<TypeId, Box<Fn(usize, usize) -> Box<Component> + 'a>>,
    entities: HashMap<usize, Box<Entity>>,
    entity_ids: Vec<usize>,
    component_counter: usize,
    entity_counter: usize
}

impl<'a> DataManager<'a> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            components_type_to_guid: HashMap::new(),
            components_entity_to_guid: HashMap::new(),
            typeid_to_new_component_fn: HashMap::new(),
            entities: HashMap::new(),
            entity_ids: Vec::new(),
            component_counter: 0,
            entity_counter: 0
        }
    }

    pub fn register_component<F>(&mut self, type_id: TypeId, func: F) where
        F: Fn(usize, usize) -> Box<Component> + 'a
    {
        self.typeid_to_new_component_fn.insert(type_id, Box::new(func));
    }

    pub fn new_entity(&mut self, name: &'static str, components: u64) {
        let entity_id = self.entity_counter;

        self.entities.insert(
            entity_id, 
            Box::new(Entity { 
                name: name.to_owned(), 
                guid: self.entity_counter, 
                components_flag: components 
        }));
        self.components_entity_to_guid.insert(self.entity_counter, Vec::new());

        self.new_components(entity_id, components);

        self.entity_ids.push(self.entity_counter);
        self.entity_counter += 1;
        self.component_counter += 1;
    }

    fn new_components(&mut self, entity_id: usize, components: u64) {
        // println!("poop");
        // self.components.insert(self.component_counter, Box::new(ActorComponent::new(self.component_counter, self.entity_counter)));

        // self.components_typeToGUID.entry(COMPONENT_FLAG_ACTOR).or_insert(Vec::new());
        // let typeVec = self.components_typeToGUID.get_mut(&COMPONENT_FLAG_ACTOR).unwrap();
        // typeVec.push(self.component_counter);

        // let entityVec = self.components_entityToGUID.get_mut(&self.entity_counter).unwrap();
        // entityVec.push(self.component_counter);
    }

    pub fn get_entity(&self, key: &usize) -> Option<&Box<Entity>> {
        self.entities.get(key)
    }

    pub fn get_component_with_attr(&self, entity_id: usize, component: u64) -> Option<&Box<Component>> {
        if let Some(a) = self.components_entity_to_guid.get(&entity_id) {
            if let Some(b) = self.components_type_to_guid.get(&component) {
                self.get_component(*a.iter().zip(b.iter()).filter(|&(a, b)| a == b).collect::<Vec<_>>()[0].0)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn get_component_with_attr_mut(&mut self, entity_id: usize, component: u64) -> Option<&mut Box<Component>> {
        let guid = match self.components_entity_to_guid.get(&entity_id) {
            Some(a) => {
                match self.components_type_to_guid.get(&component) {
                    Some(b) => {
                        Some(*a.iter().zip(b.iter()).filter(|&(a, b)| a == b).collect::<Vec<_>>()[0].0)
                    }
                    None => None
                }
            }
            None => None
        };

        if let Some(guid) = guid {
            self.get_component_mut(guid)
        } else {
            None
        }
    }

    pub fn get_component(&self, guid: usize) -> Option<&Box<Component>> {
        self.components.get(&guid)
    }

    pub fn get_component_mut(&mut self, guid: usize) -> Option<&mut Box<Component>> {
        self.components.get_mut(&guid)
    }

    pub fn get_all_components(&self) -> &HashMap<usize, Box<Component>> {
        &self.components
    }

}