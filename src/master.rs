use crate::{
	built_in_components::*,
	built_in_systems::*,
	util::{
		load_texture_file,
		// load_sound_file,
	},
};
use macroquad::prelude::*;
use hecs::World;

pub struct Master {
	pub time_since_start: f64,
	pub render_order: Vec<&'static str>,
	pub camera_pos: Vec2,
	pub zoom: f32,
}

impl Master {
	pub fn new() -> Master {
		Master {
			time_since_start: 0.0,
			render_order: vec![
				"default",
			],
			camera_pos: Vec2::ZERO,
			zoom: 1.0,
		}
	}

	pub async fn init(&mut self, world: &mut World) {
	}

	pub fn update(&mut self, world: &mut World) {
		self.time_since_start = get_time();
		rigidbody2d_system(world);
		animator_update_system(world);
	}

	pub fn render(&self, world: &mut World) {
		for layer in self.render_order.iter() {
			texture_render_system(world, layer);
			map_render_system(world, layer);
			text_render_system(world, layer);
		}
	}
}