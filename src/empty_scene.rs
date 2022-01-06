use crate::Master;
use hecs::World;
use crate::built_in_components::*;
use macroquad::prelude::*;

impl Master {
	pub fn load_empty_scene(&mut self, world: &mut World) {
		*world = World::new();
	}
}