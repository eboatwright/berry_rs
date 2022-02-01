use macroquad::prelude::*;
use crate::built_in_components::*;
use crate::Master;

impl Master {
	pub fn load_game_scene(&mut self) {
		self.world.clear();

		let camera = self.world.spawn((
			Transform::default(),
			RenderCamera::default(),
		));
	}
}