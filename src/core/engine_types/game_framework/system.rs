use super::DataManager;

// System prototype
pub trait System {
    fn name(&self) -> &'static str;
    fn tick(&self, dm: &mut DataManager);
}