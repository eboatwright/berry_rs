use std::collections::HashMap;
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
				"player",
				"map",
			],
			camera_pos: Vec2::ZERO,
			zoom: 1.0,
		}
	}

	pub async fn init(&mut self, world: &mut World) {
		let mut map_special_collision = HashMap::new();
		let platform_collider = BoxCollider2D {
			size: vec2(12.0, 2.0),
			offset: vec2(0.0, 0.0),
		};
		for i in 6..11 {
			map_special_collision.insert(i, (platform_collider, false, true, false, false));
		}

		world.spawn((
			Map {
				tile_size: 12,
				tiles: Map::read("res/map/test_map"),
				ignore_collision: vec![5],
				special_collisions: map_special_collision,
				..Default::default()
			},
			Texture {
				render_layer: "map",
				texture: load_texture_file("res/img/tileset.png").await,
				..Default::default()
			},
		));

		world.spawn((
			Transform {
				..Default::default()
			},
			Rigidbody2D {
				friction: vec2(0.3, 0.0),
				gravity: vec2(0.0, 0.3),
				grounded_time: 5.0,
				..Default::default()
			},
			Dynamic,
			CollideWithMap,
			BoxCollider2D {
				size: vec2(2.0, 11.0),
				offset: vec2(7.0, 5.0),
			},
			Movement,
			Texture {
				render_layer: "player",
				texture: load_texture_file("res/img/test.png").await,
				color: WHITE,
				source: Rect {
					x: 0.0,
					y: 0.0,
					w: 16.0,
					h: 16.0,
				},
			},
			Animator {
				animations: vec![
					Animation {
						id: "idle",
						frames: vec![0, 1, 2, 3],
						frame_duration: 7.0,
					},
					Animation {
						id: "walk",
						frames: vec![4, 5, 6, 7, 8, 9],
						frame_duration: 7.0,
					},
					Animation {
						id: "jump",
						frames: vec![4],
						frame_duration: 1.0,
					},
				],
				..Default::default()
			},
			AnimateTexture,
		));
	}

	pub fn update(&mut self, world: &mut World) {
		self.time_since_start = get_time();
		for (_entity, (transform, rigidbody2d, animator, _movement)) in &mut world.query::<(&mut Transform, &mut Rigidbody2D, &mut Animator, &Movement)>() {
			let mut new_animation = "idle";
			
			if is_key_down(KeyCode::A) {
				rigidbody2d.velocity.x -= 1.0;
				transform.scale.x = -1.0;
				new_animation = "walk";
			}
			if is_key_down(KeyCode::D) {
				rigidbody2d.velocity.x += 1.0;
				transform.scale.x = 1.0;
				new_animation = "walk";
			}

			if rigidbody2d.grounded > 0.0 {
				if is_key_pressed(KeyCode::W) {
					rigidbody2d.velocity.y = -5.0;
					rigidbody2d.grounded = 0.0;
				}
			} else {
				new_animation = "jump";
			}

			animator.change_animation(new_animation);
		}
		rigidbody2d_system(world);
		for (_entity, (transform, _movement)) in &mut world.query::<(&Transform, &Movement)>() {
			self.camera_pos = transform.position.truncate().round() + vec2(8.0, 8.0);
		}
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

pub struct Movement;