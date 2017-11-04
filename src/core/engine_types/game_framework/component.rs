use std::any::Any;

pub trait Component {
    fn name(&self) -> &str;
    fn guid(&self) -> usize;
    fn component_id(&self) -> u64;
    fn entity_id(&self) -> usize;
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    // fn get_dm(&self) -> &DataManager;
    // fn register_component(&self)
    //     where Self: 'static + Sized,
    //     for<'r> Self: Fn(&'r str, usize, usize)
    // {
    //     self.get_dm().register_component(TypeId::of::<Self>(), &self);
    // }
    // fn test(&self) {
    //     self.get_dm().register_component(TypeId::of::<String>(), &self.new);
    // }
}
