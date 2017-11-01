use super::{DataManager, System};

// This struct provides the world context to systems
pub struct World {
    pub name: String,
    pub systems: Vec<Box<System>>,
    pub dm: DataManager
}