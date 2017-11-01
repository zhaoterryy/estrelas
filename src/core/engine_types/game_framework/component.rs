use std::any::Any;

pub trait Component {
    fn name(&self) -> &'static str;
    fn guid(&self) -> usize;
    fn component_id(&self) -> u64;
    fn entity_id(&self) -> usize;
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}