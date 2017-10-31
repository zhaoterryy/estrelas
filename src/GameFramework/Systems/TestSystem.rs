use GameFramework::FrameworkTypes::*;
use GameFramework::Systems::DataManager::*;

#[allow(dead_code)]
pub(crate) struct TestSystem;

impl System for TestSystem {
    fn Name(&self) -> &'static str {
        "Test System"
    }

    fn Tick(&self, dm: &mut DataManager) {
        for (_, comp) in dm.getAllComponents().iter() {
            println!("Found: {0} belonging to entity: {1}", comp.name(), comp.getAttachedEntityID());
        }
    }
}