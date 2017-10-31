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
    fn Name(&self) -> &'static str {
        "Actor Component"
    }
    fn GetGUID(&self) -> usize {
        self.GUID
    }
    fn GetComponentID(&self) -> FLAG_Components {
        COMPONENT_FLAG_ACTOR
    }
    fn GetAttachedEntityID(&self) -> usize {
        self.AttachedEntityID
    }
    fn GetAsAny(&self) -> &Any {
        self
    }
    fn getAsAny_mut(&mut self) -> &mut Any {
        &mut *self
    }
}