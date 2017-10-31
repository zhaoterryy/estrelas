use GameFramework::FrameworkTypes::*;
use GameFramework::Systems::DataManager::*;
use GameFramework::Components::ActorComponent::*;

#[allow(dead_code)]
pub(crate) struct TestSystem;

impl System for TestSystem {
    fn Name(&self) -> &'static str {
        "Test System"
    }

    fn Tick(&self, dm: &mut DataManager) {
        // let ActorComp: &mut ActorComponent = match dm.getComponent_mut(0, COMPONENT_FLAG_ACTOR) {
        //     Some(comp) => {
        //         match comp.getAsAny_mut().downcast_mut::<ActorComponent>() {
        //             Some(ret) => ret,
        //             None => panic!("poop")
        //         }
        //     },
        //     None => panic!("poop")
        // };
        // println!("Found: {0} belonging to entityID: {1}", ActorComp.Name(), ActorComp.GetAttachedEntityID());
        // println!("{}", ActorComp.Transform.Position);
        // ActorComp.Transform.Position.x += 1.0;

        for (_,compList) in dm.getComponents_mut().iter() {
            for (k, v) in compList.iter() {
                println!("Found: {0} belonging to entity: {1}", v.Name(), v.GetAttachedEntityID());
            }
        }
    }
}