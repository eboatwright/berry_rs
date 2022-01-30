use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use std::collections::HashMap;
use macroquad::audio::Sound;
use crate::Master;
use hecs::Entity;
use hecs::World;
use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Parent(pub u32);

impl Parent {
	pub fn get(&self, world: &mut World) -> Option<Entity> {
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
pub struct BoxCollider {
	pub size: Vec2,
	pub offset: Vec2,
}

impl Default for BoxCollider {
	fn default() -> Self {
		Self {
			size: Vec2::ONE,
			offset: Vec2::ZERO,
		}
	}
}

impl BoxCollider {
	pub fn overlaps(a: (&BoxCollider, &Transform), b: (&BoxCollider, &Transform)) -> bool {
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
pub struct Rigidbody {
	pub velocity: Vec2,
	pub gravity: Vec2,
	pub friction: Vec2,
	pub grounded: f32,
	pub grounded_time: f32,
}

impl Rigidbody {
	pub fn grounded(&self) -> bool {
		self.grounded > 0.0
	}
}

#[derive(Copy, Clone)]
pub struct ButtonClickFunction(pub fn(&mut Master, Entity));

impl Default for ButtonClickFunction {
	fn default() -> Self {
		Self(|_master, _entity| {
		})
	}
}

#[derive(Copy, Clone, Default)]
pub struct Button {
	pub function: ButtonClickFunction,
	pub highlight_offset: Vec2,
	pub animation_smooth: f32,
	pub hovering_over: bool,
	pub select_sfx: Option<Sound>,
	pub played_select_sfx: bool,
	pub shadow_color: Color,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Slider {
	pub limits: Vec2,
	pub vertical: bool,
	pub dragging: bool,
}

impl Default for Slider {
	fn default() -> Self {
		Self {
			limits: vec2(-20.0, 20.0),
			vertical: false,
			dragging: false,
		}
	}
}

pub struct DontAnimateTexture;

#[derive(Clone, PartialEq)]
pub struct Animation {
	pub name: &'static str,
	pub frames: Vec<usize>,
	pub frame_duration: f32,
	pub dont_interrupt: bool,
}

impl Default for Animation {
	fn default() -> Self {
		Self {
			name: "animation",
			frames: vec![],
			frame_duration: 1.0,
			dont_interrupt: false,
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct Animator {
	pub timer: f32,
	pub animations: Vec<Animation>,
	pub current_animation: Animation,
	pub current_frame_index: usize,
	pub dont_interrupt: bool,
}

impl Default for Animator {
	fn default() -> Self {
		Self {
			timer: 0.0,
			animations: Vec::new(),
			current_animation: Animation::default(),
			current_frame_index: 0,
			dont_interrupt: false,
		}
	}
}

impl Animator {
	pub fn change_animation(&mut self, name: &'static str) {
		if !self.dont_interrupt
		&& name != self.current_animation.name {
			for animation in self.animations.iter() {
				if animation.name == name {
					self.dont_interrupt = animation.dont_interrupt;

					self.current_animation = animation.clone();
					self.current_frame_index = 0;
					self.timer = 0.0;

					return;
				}
			}
		}
	}

	pub fn get_frame(&self) -> f32 {
		self.current_animation.frames[self.current_frame_index] as f32
	}
}

#[derive(Copy, Clone)]
pub struct RenderCamera {
	pub target: u32,
	pub follow_offset: Vec2,
	pub smoothing: f32,
	pub zoom: f32,
}

impl Default for RenderCamera {
	fn default() -> Self {
		Self {
			target: u32::MAX,
			follow_offset: Vec2::ZERO,
			smoothing: 1.0,
			zoom: 0.0,
		}
	}
}

pub fn get_mouse_position(world: &World) -> Vec2 {
	for (_entity, (transform, camera)) in &mut world.query::<(&Transform, &RenderCamera)>() {
		let mut mouse_pos = vec2(mouse_position().0, mouse_position().1);
	
		mouse_pos.x = (mouse_pos.x - screen_width() / 2.0) / camera.zoom + transform.position.x;
		mouse_pos.y = (mouse_pos.y - screen_height() / 2.0) / camera.zoom + transform.position.y;

		return mouse_pos;
	}
	panic!("'mouse_position' error: no camera!");
}

pub fn camera_bounds(world: &World) -> Rect {
	for (_entity, transform) in &mut world.query::<&Transform>().with::<RenderCamera>() {
		return Rect {
			x: transform.position.x - SCREEN_WIDTH as f32 / 2.0,
			y: transform.position.y - SCREEN_HEIGHT as f32 / 2.0,
			w: transform.position.x + SCREEN_WIDTH as f32 / 2.0,
			h: transform.position.y + SCREEN_HEIGHT as f32 / 2.0,
		};
	}
	panic!("'camera_bounds' error: no camera!");
}

pub fn rect_in_screen(world: &World, rect: Rect) -> bool {
	let camera_bounds = camera_bounds(&world);
	!(rect.x + rect.w < camera_bounds.x
	|| rect.x > camera_bounds.w
	|| rect.y + rect.h < camera_bounds.y
	|| rect.y > camera_bounds.h)
}

#[derive(Clone, PartialEq)]
pub struct Map {
	pub tile_size: u16,
	pub tiles: Vec<Vec<u16>>,
	pub special_collision: HashMap<u16, (BoxCollider, bool, bool, bool, bool)>,
	pub colors: HashMap<u16, Color>,
	pub y_source_offsets: HashMap<u16, f32>,
}

impl Default for Map {
	fn default() -> Self {
		Self {
			tile_size: 16,
			tiles: Vec::new(),
			special_collision: HashMap::new(),
			colors: HashMap::new(),
			y_source_offsets: HashMap::new(),
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct ParticleSpawner {
	pub particle_life: f32,
	pub spawn_rate: f32,
	pub spawn_timer: f32,
	pub min_velocity: Vec2,
	pub max_velocity: Vec2,
	pub particle_rigidbody: Rigidbody,
	pub particle_texture: Texture,
	pub min_spawn_offset: Vec2,
	pub max_spawn_offset: Vec2,
}

impl Default for ParticleSpawner {
	fn default() -> Self {
		Self {
			particle_life: 30.0,
			spawn_rate: 5.0,
			spawn_timer: 0.0,
			min_velocity: vec2(-3.0, -3.0),
			max_velocity: vec2(3.0, 3.0),
			particle_rigidbody: Rigidbody {
				friction: vec2(0.1, 0.1),
				..Default::default()
			},
			particle_texture: Texture::default(),
			min_spawn_offset: vec2(-8.0, -8.0),
			max_spawn_offset: vec2(8.0, 8.0),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Particle {
	pub life: f32,
}

#[derive(Copy, Clone, PartialEq)]
pub struct DropShadow {
	pub color: Color,
	pub offset: Vec2,
}

impl Default for DropShadow {
	fn default() -> Self {
		Self {
			color: Color {
				r: 0.0,
				g: 0.0,
				b: 0.0,
				a: 0.5,
			},
			offset: vec2(-2.0, 2.0),
		}
	}
}

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
			source.size()
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