use super::Systems::DataManager::*;
use std::any::Any;

bitflags! {
	#[allow(non_camel_case_types)]
	flags FLAG_Components: u32 {
		#[allow(dead_code)]
		const COMPONENT_FLAG_NIL      = 0b00000000,
		#[allow(dead_code)]
		const COMPONENT_FLAG_TEST     = 0b00000001,
		#[allow(dead_code)]
		const COMPONENT_FLAG_ACTOR    = 0b00000010
	}
}

// Execute all the code paths to shut up warnings.
// FIX: https://github.com/rust-lang/rust/issues/24580
impl FLAG_Components {    
	#[allow(dead_code)]
	fn _dead_code(&mut self) {
		self.contains(COMPONENT_FLAG_NIL);
		self.insert(COMPONENT_FLAG_NIL);
		self.is_all();
		self.is_empty();
		self.bits();
		self.intersects(*self);
		self.remove(COMPONENT_FLAG_NIL);
		self.toggle(COMPONENT_FLAG_NIL);
		FLAG_Components::from_bits(0b00000000);
		FLAG_Components::from_bits_truncate(0b00000000);
	}
}

pub(in GameFramework) struct World {
	pub Name: String,
	pub Systems: Vec<Box<System>>,
	pub Entities: Vec<usize>
}

pub(in GameFramework) struct Entity {
	pub Name: String,
	pub GUID: usize,
	pub ComponentsFlag: FLAG_Components
}

impl Entity {
	pub fn new(name: &'static str, GUID: usize, CF: FLAG_Components) -> Entity {
		Entity {
			Name: name.to_string(),
			GUID: GUID,
			ComponentsFlag: CF
		}
	}
}

pub(in GameFramework) trait Component {
	fn name(&self) -> &'static str;
	fn getGUID(&self) -> usize;
	fn getComponentID(&self) -> FLAG_Components;
	fn getAttachedEntityID(&self) -> usize;
	fn getAsAny(&self) -> &Any;
	fn getAsAny_mut(&mut self) -> &mut Any;
}

pub(in GameFramework) trait System {
	fn Name(&self) -> &'static str;
	fn Tick(&self, dm: &mut DataManager);
}
