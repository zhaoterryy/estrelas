use Core::EngineTypes::Transform2D;
use GameFramework::FrameworkTypes::*;
use std::any::Any;

pub(in GameFramework) struct ActorComponent {
    pub Transform: Transform2D,
    GUID: usize,
    AttachedEntityID: usize
}

impl ActorComponent {
    pub fn new(GUID: usize, EntityGUID: usize) -> ActorComponent {
        ActorComponent {
            GUID: GUID,
            Transform: Default::default(),
            AttachedEntityID: EntityGUID
        }
    }
}

impl Component for ActorComponent {
    fn name(&self) -> &'static str {
        "Actor Component"
    }
    fn getGUID(&self) -> usize {
        self.GUID
    }
    fn getComponentID(&self) -> FLAG_Components {
        COMPONENT_FLAG_ACTOR
    }
    fn getAttachedEntityID(&self) -> usize {
        self.AttachedEntityID
    }
    fn getAsAny(&self) -> &Any {
        self
    }
    fn getAsAny_mut(&mut self) -> &mut Any {
        &mut *self
    }
}