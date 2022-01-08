use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use crate::resources::Resources;
use crate::built_in_components::*;
use crate::built_in_systems::*;
use macroquad::prelude::*;
use hecs::World;

pub struct Master {
	pub time_since_start: f64,
	pub camera: Rect,
	pub render_order: Vec<&'static str>,
	pub zoom: f32,
	pub resources: Resources,
}

impl Master {
	pub fn new() -> Master {
		Master {
			time_since_start: 0.0,
			camera: Rect {
				x: 0.0,
				y: 0.0,
				w: SCREEN_WIDTH as f32,
				h: SCREEN_HEIGHT as f32,
			},
			render_order: vec![
				"default",
				"particle",
				"ui",
			],
			zoom: 1.0,
			resources: Resources::new(),
		}
	}

	pub fn update(&mut self, world: &mut World) {
		self.time_since_start = get_time();
		rigidbody2d_update_system(world);
		animator_update_system(world);
		particle_update_system(world);
		camera_update_system(world, self);
		button_update_system(world, self);
		follow_update_system(world);
		sin_wave_update_system(world, self);
	}

	pub fn render(&mut self, world: &mut World) {
		for layer in self.render_order.iter() {
			texture_render_system(world, self.camera, layer);
			map_render_system(world, self.camera, layer);
			text_render_system(world, layer);
		}
	}
}