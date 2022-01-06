use macroquad::audio::Sound;
use hecs::World;
use crate::Master;
use std::fs;
use std::collections::HashMap;
use crate::util::get_file_path;
use macroquad::prelude::*;

pub struct Transform {
	pub position: Vec3,
	pub scale: Vec3,
	pub rotation: Vec3,
}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Vec3::ZERO,
			scale: Vec3::ONE,
			rotation: Vec3::ZERO,
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct BoxCollider2D {
	pub size: Vec2,
	pub offset: Vec2,
}

impl BoxCollider2D {
	pub fn overlaps(&self, transform: &Transform, other: &BoxCollider2D, other_transform: &Transform) -> bool {
		let a_position = transform.position + self.offset.extend(0.0);
		let a_size = self.size.extend(0.0);

		let b_position = other_transform.position + other.offset.extend(0.0);
		let b_size = other.size.extend(0.0);

		a_position.x < b_position.x + b_size.x &&
        a_position.x + a_size.x > b_position.x &&
        a_position.y < b_position.y + b_size.y &&
        a_position.y + a_size.y > b_position.y
	}
}

#[derive(Default)]
pub struct Rigidbody2D {
	pub velocity: Vec2,
	pub friction: Vec2,
	pub gravity: Vec2,
	pub grounded: f32,
	pub grounded_time: f32,
}

pub struct Button {
	pub function: fn(&mut World, &mut Master),
	pub hovering_over: bool,
	pub highlight_offset: Vec2,
	pub animation_smooth: f32,
	pub select_sfx: Option<Sound>,
	pub selected: bool,
}

impl Default for Button {
	fn default() -> Button {
		Button {
			function: | _world, _master | {
			},
			hovering_over: false,
			highlight_offset: vec2(0.0, -4.0),
			animation_smooth: 0.2,
			select_sfx: None,
			selected: false,
		}
	}
}

#[derive(Default)]
pub struct SinWave {
	pub value: f64,
	pub speed: f64,
	pub distance: f64,
	pub offset: f64,
}

#[derive(Default)]
pub struct Animation {
	pub id: &'static str,
	pub frames: Vec<usize>,
	pub frame_duration: f32,
}

impl Animation {
	pub fn copy(&self) -> Animation {
		Animation {
			id: self.id,
			frames: self.frames.clone(),
			frame_duration: self.frame_duration,
		}
	}
}

pub struct DontAnimateTexture;

#[derive(Default)]
pub struct Animator {
	pub animation_timer: f32,
	pub animation_frame_index: usize,
	pub animations: Vec<Animation>,
	pub current_animation: Animation,
	pub dont_interrupt: bool,
}

impl Animator {
	pub fn change_animation(&mut self, animation_id: &'static str) {
		if !self.dont_interrupt
		&& animation_id != self.current_animation.id {
			for animation in self.animations.iter() {
				if animation.id == animation_id {
					self.current_animation = animation.copy();
					self.animation_timer = 0.0;
					self.animation_frame_index = 0;

					return;
				}
			}
		}
	}

	pub fn play_animation_uninterrupted(&mut self, animation_id: &'static str) {
		for animation in self.animations.iter() {
			if animation.id == animation_id {
				self.current_animation = animation.copy();
				self.animation_timer = 0.0;
				self.animation_frame_index = 0;
				self.dont_interrupt = true;

				return;
			}
		}
	}

	pub fn current_frame(&self) -> usize {
		self.current_animation.frames[self.animation_frame_index]
	}
}

#[derive(Default)]
pub struct Map {
	pub tile_size: i32,
	pub tiles: Vec<Vec<i32>>,
	pub ignore_collision: Vec<i32>,
	pub transparent_tiles: HashMap<i32, f32>,
	pub special_collisions: HashMap<i32, (BoxCollider2D, bool, bool, bool, bool)>,
	pub y_offsets: HashMap<i32, f32>,
}

impl Map {
	pub fn read(path: String) -> Vec<Vec<i32>> {
		let save = fs::read_to_string(format!("{}.blueberry-map", get_file_path(path))).unwrap();
		let mut tiles: Vec<Vec<i32>> = Vec::new();
		for line in save.split('\n') {
			let mut tile_line: Vec<i32> = Vec::new();
			for tile in line.split(',') {
				tile_line.push(tile.parse().unwrap());
			}
			tiles.push(tile_line);
		}
		tiles
	}
}

pub struct ParticleSpawner {
	pub life: f32,
	pub spawn_rate: f32,
	pub spawn_timer: f32,
	pub min_velocity: Vec2,
	pub max_velocity: Vec2,
	pub gravity: Vec2,
	pub friction: Vec2,
	pub texture: Texture2D,
	pub color: Color,
	pub min_spawn_offset: Vec2,
	pub max_spawn_offset: Vec2,
}

impl Default for ParticleSpawner {
	fn default() -> ParticleSpawner {
		ParticleSpawner {
			life: 60.0,
			spawn_rate: 10.0,
			spawn_timer: 0.0,
			min_velocity: vec2(-1.0, -1.0),
			max_velocity: vec2(1.0, 1.0),
			gravity: Vec2::ZERO,
			friction: Vec2::ZERO,
			texture: Texture2D::empty(),
			color: WHITE,
			min_spawn_offset: Vec2::ZERO,
			max_spawn_offset: Vec2::ZERO,
		}
	}
}

pub struct Particle {
	pub life: f32,
}

pub struct CameraTarget {
	pub smoothing: f32,
	pub offset: Vec3,
}

impl Default for CameraTarget {
	fn default() -> CameraTarget {
		CameraTarget {
			smoothing: 1.0,
			offset: Vec3::ZERO,
		}
	}
}

#[derive(Default)]
pub struct FollowCamera {
	pub offset: Vec3,
}

pub struct Follow {
	pub id: u32,
	pub offset: Vec3,
}

pub struct DropShadow {
	pub color: Color,
	pub offset: Vec2,
}

impl Default for DropShadow {
	fn default() -> DropShadow {
		DropShadow {
			color: Color {
				r: 0.0,
				g: 0.0,
				b: 0.0,
				a: 0.1,
			},
			offset: vec2(5.0, 5.0),
		}
	}
}

#[derive(Default)]
pub struct RenderOffset(pub Vec2);

pub struct Texture {
	pub render_layer: &'static str,
	pub texture: Texture2D,
	pub color: Color,
	pub source: Rect,
}

impl Default for Texture {
	fn default() -> Texture {
		Texture {
			render_layer: "default",
			texture: Texture2D::empty(),
			color: WHITE,
			source: Rect::default(),
		}
	}
}

impl Texture {
	pub fn size(&self) -> Vec2 {
		if self.source == Rect::default() {
			vec2(self.texture.width(), self.texture.height())
		} else {
			self.source.size()
		}
	}
}

pub struct TextRenderer {
	pub render_layer: &'static str,
	pub text: &'static str,
	pub params: TextParams,
}

impl Default for TextRenderer {
	fn default() -> TextRenderer {
		TextRenderer {
			render_layer: "default",
			text: "",
			params: TextParams {
				font: Font::default(),
				font_size: 16,
				font_scale: 1.0,
				font_scale_aspect: 1.0,
				color: WHITE,
			},
		}
	}
}