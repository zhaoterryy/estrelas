use super::Component;
use std::any::Any;

pub struct RenderComponent {
    guid: usize,
    entity_id: usize,
    component_id: u64
}

impl RenderComponent {
    pub fn new_boxed(guid: usize, entity_id: usize) -> Box<Component> {
        Box::new(
        RenderComponent {
            guid: guid, 
            entity_id: entity_id,
            component_id: 0u64
        })
    }
}

impl Component for RenderComponent {
    fn name(&self) -> &str {
        "Render Component"
    }

    fn guid(&self) -> usize {
        self.guid
    }

    fn component_id(&self) -> u64 {
        self.component_id
    }

    fn entity_id(&self) -> usize {
        self.entity_id
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        &mut *self
    }
}