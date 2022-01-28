use hecs::Entity;
use hecs::World;
use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Parent(pub u32);

impl Parent {
	pub fn get(&self, world: &mut World) -> Entity {
		world.find_entity_from_id(self.0)
	}
}

pub struct Transform {}

pub struct BoxCollider2D {}

pub struct Rigidbody2D {}

pub struct Button {}

pub struct Slider {}

pub struct SinWave {}

pub struct Animation {}

pub struct Animator {}

#[derive(Default, Copy, Clone)]
pub struct RenderCamera {
	pub position: Vec2,
	pub zoom: f32,
}

impl RenderCamera {
	pub fn get_mouse_position(&self) -> Vec2 {
		let mut mouse_pos = vec2(mouse_position().0, mouse_position().1);
		
		mouse_pos.x = (mouse_pos.x - screen_width() / 2.0) / self.zoom + self.position.x;
		mouse_pos.y = (mouse_pos.y - screen_height() / 2.0) / self.zoom + self.position.y;

		mouse_pos
	}
}

pub struct Map {}

pub struct ParticleSpawner {}

pub struct Particle {}

pub struct DropShadow {}

pub struct RenderOffset {}

pub struct RenderLayer {}

pub struct Texture {}

pub struct Rectangle {}

pub struct TextRenderer {}