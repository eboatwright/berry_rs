use crate::resources::Resources;
use crate::built_in_components::*;
use crate::built_in_systems::*;
use macroquad::prelude::*;
use hecs::World;

pub struct Master {
	pub render_order: Vec<&'static str>,
	pub resources: Resources,
}

impl Default for Master {
	fn default() -> Master {
		Master {
			render_order: vec![
				"default",
				"particle",
				"ui",
			],
			resources: Resources::new(),
		}
	}
}

impl Master {
	pub fn update(&mut self, world: &mut World) {
	}

	pub fn render(&mut self, world: &mut World) {
		for layer in self.render_order.iter() {
		}
	}
}