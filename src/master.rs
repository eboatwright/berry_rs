use crate::resources::Resources;
use macroquad::prelude::*;
use crate::built_in_components::*;
use crate::built_in_systems::*;
use hecs::World;

pub struct Master {
	pub world: World,
	pub render_order: Vec<&'static str>,
	pub resources: Resources,
}

impl Default for Master {
	fn default() -> Master {
		Master {
			world: World::default(),
			render_order: vec![
				"shadow",
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
		rigidbody_update_system(self);
		button_update_system(self);
		slider_update_system(self);
		animator_update_system(self);
		particle_update_system(self);
		camera_update_system(self);
		sin_wave_update_system(self);
	}

	pub fn render(&self) {
		for layer in self.render_order.iter() {
			drop_shadow_render_system(self, layer);
			texture_render_system(self, layer);
			map_render_system(self, layer);
			rectangle_render_system(self, layer);
			text_render_system(self, layer);
		}
	}
}