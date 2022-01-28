use hecs::Entity;
use hecs::World;
use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Parent(pub u32);

impl Parent {
	pub fn get(&self, world: &mut World) -> Entity {
		world.find_entity_from_id(self.0)
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Transform {
	pub position: Vec2,
	pub scale: Vec2,
	pub rotation: f32,
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vec2::ZERO,
			scale: Vec2::ONE,
			rotation: 0.0,
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct BoxCollider2D {
	pub size: Vec2,
	pub offset: Vec2,
}

impl Default for BoxCollider2D {
	fn default() -> Self {
		Self {
			size: Vec2::ONE,
			offset: Vec2::ZERO,
		}
	}
}

impl BoxCollider2D {
	pub fn overlaps(a: (&BoxCollider2D, &Transform),
					b: (&BoxCollider2D, &Transform)) -> bool {
		let a_position = a.1.position + a.0.offset;
		let a_size = a.0.size * a.1.scale;

		let b_position = b.1.position + b.0.offset;
		let b_size = b.0.size * b.1.scale;

		a_position.x < b_position.x + b_size.x &&
        a_position.x + a_size.x > b_position.x &&
        a_position.y < b_position.y + b_size.y &&
        a_position.y + a_size.y > b_position.y
	}
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Rigidbody2D {
	pub velocity: Vec2,
	pub gravity: Vec2,
	pub friction: Vec2,
	pub grounded: f32,
	pub grounded_time: f32,
}

//TODO
pub struct Button {}

//TODO
pub struct Slider {}

//TODO
pub struct Animation {}

//TODO
pub struct Animator {}

#[derive(Default, Copy, Clone)]
pub struct RenderCamera {
	pub position: Vec2,
	pub zoom: f32,
}

impl RenderCamera {
	pub fn mouse_position(&self) -> Vec2 {
		let mut mouse_pos = vec2(mouse_position().0, mouse_position().1);
		
		mouse_pos.x = (mouse_pos.x - screen_width() / 2.0) / self.zoom + self.position.x;
		mouse_pos.y = (mouse_pos.y - screen_height() / 2.0) / self.zoom + self.position.y;

		mouse_pos
	}
}

//TODO
pub struct Map {}

//TODO
pub struct ParticleSpawner {}

//TODO
pub struct Particle {}

//TODO
pub struct DropShadow {}

#[derive(Clone, PartialEq)]
pub struct RenderLayer(pub String);

impl Default for RenderLayer {
	fn default() -> Self {
		Self("default".to_string())
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Texture {
	pub texture: Texture2D,
	pub color: Color,
	pub source: Option<Rect>,
}

impl Default for Texture {
	fn default() -> Self {
		Self {
			texture: Texture2D::empty(),
			color: WHITE,
			source: None,
		}
	}
}

impl Texture {
	pub fn get_size(&self) -> Vec2 {
		return if let Some(source) = self.source {
			vec2(source.w, source.h)
		} else {
			vec2(self.texture.width(), self.texture.height())
		};
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Rectangle {
	pub size: Vec2,
	pub color: Color,
}

impl Default for Rectangle {
	fn default() -> Self {
		Self {
			size: Vec2::ONE,
			color: WHITE,
		}
	}
}

#[derive(Clone)]
pub struct TextRenderer {
	pub text: String,
	pub params: TextParams,
}

impl Default for TextRenderer {
	fn default() -> Self {
		Self {
			text: "default text".to_string(),
			params: TextParams {
				font_size: 16,
				..Default::default()
			},
		}
	}
}