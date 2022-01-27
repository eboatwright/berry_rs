use crate::scene::Scene;
use crate::resources::Resources;
use crate::built_in_components::*;
use crate::built_in_systems::*;
use macroquad::prelude::*;
use hecs::World;

pub struct Master {
	pub current_scene: Scene,
	pub render_order: Vec<&'static str>,
	pub resources: Resources,
}

impl Default for Master {
	fn default() -> Master {
		Master {
			current_scene: Scene::default(),
			render_order: vec![
				"default",
				"particle",
				"ui",
			],
			resources: Resources::default(),
		}
	}
}

impl Master {
	pub fn update(&mut self) {
	}

	pub fn render(&self) {
		for layer in self.render_order.iter() {
		}
	}
}