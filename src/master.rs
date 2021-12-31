use crate::resources::Resources;
use crate::built_in_components::*;
use crate::built_in_systems::*;
use crate::util::load_texture_file;
use crate::util::load_sound_file;
use macroquad::prelude::*;
use hecs::World;

pub struct Master {
	pub time_since_start: f64,
	pub camera_pos: Vec2,
	pub render_order: Vec<&'static str>,
	pub zoom: f32,
	pub resources: Resources,
}

impl Master {
	pub fn new() -> Master {
		Master {
			time_since_start: 0.0,
			camera_pos: Vec2::ZERO,
			render_order: vec![
				"default",
			],
			zoom: 1.0,
			resources: Resources::empty(),
		}
	}

	pub fn load_empty_scene(&mut self, world: &mut World) {
		*world = World::new();
	}

	pub fn update(&mut self, world: &mut World) {
		self.time_since_start = get_time();
		rigidbody2d_update_system(world, self.camera_pos);
		animator_update_system(world);
		particle_update_system(world);
		camera_update_system(world, self);
		button_update_system(world, self);
		follow_update_system(world);
		sin_wave_update_system(world, self);
	}

	pub fn render(&mut self, world: &mut World) {
		for layer in self.render_order.iter() {
			texture_render_system(world, self.camera_pos, layer);
			map_render_system(world, self.camera_pos, layer);
			text_render_system(world, layer);
		}
	}
}