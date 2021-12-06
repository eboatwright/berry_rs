use crate::Master;
use macroquad::prelude::*;
use crate::built_in_components::*;
use hecs::World;

// TODO: rigidbody entity collision?
pub fn rigidbody2d_system(world: &mut World) {
	for (entity, (transform, rigidbody2d, _dynamic)) in &mut world.query::<(&mut Transform, &mut Rigidbody2D, &Dynamic)>() {
		rigidbody2d.grounded -= 1.0;

		rigidbody2d.velocity.x += rigidbody2d.gravity.x;
		rigidbody2d.velocity.x *= 1.0 - rigidbody2d.friction.x;
		transform.position.x += rigidbody2d.velocity.x;

		if let Ok(collider) = world.get::<BoxCollider2D>(entity) {
			if let Ok(_collide_with_map) = world.get::<CollideWithMap>(entity) {
				for (_map_entity, map) in &mut world.query::<&Map>() {
					let mut tile_transform = Transform::default();
					let mut tile_collider;
					let mut collision = (true, true, true, true);
					for y in 0..map.tiles.len() {
						for x in 0..map.tiles[0].len() {
							if map.tiles[y][x] != 0
							&& !map.ignore_collision.contains(&map.tiles[y][x]) {
								tile_transform.position = vec3(x as f32 * map.tile_size as f32, y as f32 * map.tile_size as f32, 0.0);
								tile_collider = match map.special_collisions.get(&map.tiles[y][x]) {
									Some((collider, up_collision, down_collision, left_collision, right_collision)) => {
										collision = (*up_collision, *down_collision, *left_collision, *right_collision);
										*collider
									},
									None => BoxCollider2D {
										size: vec2(map.tile_size as f32, map.tile_size as f32),
										offset: Vec2::ZERO,
									},
								};
								if collider.overlaps(transform, &tile_collider, &tile_transform) {
									if rigidbody2d.velocity.x < 0.0
									&& collision.2 {
										rigidbody2d.velocity.x = 0.0;
										transform.position.x = tile_transform.position.x + tile_collider.size.x - collider.offset.x + tile_collider.offset.x;
										if rigidbody2d.gravity.x < 0.0 {
											rigidbody2d.grounded = rigidbody2d.grounded_time;
										}
									}
									if rigidbody2d.velocity.x > 0.0
									&& collision.3 {
										rigidbody2d.velocity.x = 0.0;
										transform.position.x = tile_transform.position.x - collider.size.x - collider.offset.x + tile_collider.offset.x;
										if rigidbody2d.gravity.x > 0.0 {
											rigidbody2d.grounded = rigidbody2d.grounded_time;
										}
									}
								}
								collision = (true, true, true, true);
							}
						}
					}
				}
			}
		}

		rigidbody2d.velocity.y += rigidbody2d.gravity.y;
		rigidbody2d.velocity.y *= 1.0 - rigidbody2d.friction.y;
		transform.position.y += rigidbody2d.velocity.y;

		if let Ok(collider) = world.get::<BoxCollider2D>(entity) {
			if let Ok(_collide_with_map) = world.get::<CollideWithMap>(entity) {
				for (_map_entity, map) in &mut world.query::<&Map>() {
					let mut tile_transform = Transform::default();
					let mut tile_collider;
					let mut collision = (true, true, true, true);
					for y in 0..map.tiles.len() {
						for x in 0..map.tiles[0].len() {
							if map.tiles[y][x] != 0
							&& !map.ignore_collision.contains(&map.tiles[y][x]) {
								tile_transform.position = vec3(x as f32 * map.tile_size as f32, y as f32 * map.tile_size as f32, 0.0);
								tile_collider = match map.special_collisions.get(&map.tiles[y][x]) {
									Some((collider, up_collision, down_collision, left_collision, right_collision)) => {
										collision = (*up_collision, *down_collision, *left_collision, *right_collision);
										*collider
									},
									None => BoxCollider2D {
										size: vec2(map.tile_size as f32, map.tile_size as f32),
										offset: Vec2::ZERO,
									},
								};
								if collider.overlaps(transform, &tile_collider, &tile_transform) {
									if rigidbody2d.velocity.y < 0.0
									&& collision.0 {
										rigidbody2d.velocity.y = 0.0;
										transform.position.y = tile_transform.position.y + tile_collider.size.y - collider.offset.y + tile_collider.offset.y;
										if rigidbody2d.gravity.y < 0.0 {
											rigidbody2d.grounded = rigidbody2d.grounded_time;
										}
									}
									if rigidbody2d.velocity.y > 0.0
									&& collision.1 {
										rigidbody2d.velocity.y = 0.0;
										transform.position.y = tile_transform.position.y - collider.size.y - collider.offset.y + tile_collider.offset.y;
										if rigidbody2d.gravity.y > 0.0 {
											rigidbody2d.grounded = rigidbody2d.grounded_time;
										}
									}
								}
								collision = (true, true, true, true);
							}
						}
					}
				}
			}
		}
	}
}

pub fn texture_render_system(world: &mut World, layer: &'static str) {
	for (_entity, (transform, texture)) in &mut world.query::<(&Transform, &Texture)>() {
		if texture.render_layer == layer {
			draw_texture_ex(
				texture.texture,
				transform.position.x.round() - texture.source.w * transform.scale.x / 2.0 + texture.source.w / 2.0,
				transform.position.y.round() - texture.source.h * transform.scale.y / 2.0 + texture.source.h / 2.0,
				texture.color,
				DrawTextureParams {
					dest_size: Some(vec2(texture.source.w * transform.scale.x, texture.source.h * transform.scale.y)),
					source: Some(texture.source),
					rotation: transform.rotation.z,
					flip_x: false,
					flip_y: false,
					pivot: None,
				},
			);
		}
	}
}

pub fn map_render_system(world: &mut World, layer: &'static str) {
	for (_entity, (map, texture)) in &mut world.query::<(&Map, &Texture)>() {
		if texture.render_layer == layer {
			for y in 0..map.tiles.len() {
				for x in 0..map.tiles[0].len() {
					if map.tiles[y][x] != 0 {
						draw_texture_ex(
							texture.texture,
							x as f32 * map.tile_size as f32,
							y as f32 * map.tile_size as f32,
							texture.color,
							DrawTextureParams {
								dest_size: Some(vec2(map.tile_size as f32, map.tile_size as f32)),
								source: Some(Rect {
									x: (map.tiles[y][x] - 1) as f32 * map.tile_size as f32,
									y: match map.y_offsets.get(&map.tiles[y][x]) {
										Some(y_offset) => *y_offset,
										None => 0.0,
									},
									w: map.tile_size as f32,
									h: map.tile_size as f32,
								}),
								rotation: 0.0,
								flip_x: false,
								flip_y: false,
								pivot: None,
							},
						);
					}
				}
			}
		}
	}
}

pub fn animator_update_system(world: &mut World) {
	for (entity, animator) in &mut world.query::<&mut Animator>() {
		animator.animation_timer -= 1.0;
		if animator.animation_timer <= 0.0 {
			animator.animation_timer = animator.current_animation.frame_duration;
			animator.animation_frame_index += 1;
			if animator.animation_frame_index >= animator.current_animation.frames.len() {
				animator.animation_frame_index = 0;
				animator.dont_interrupt = false;
			}
			if let Ok(_animate_texture) = world.get::<AnimateTexture>(entity) {
				if let Ok(mut texture) = world.get_mut::<Texture>(entity) {
					texture.source.x = animator.current_frame() as f32 * texture.source.w;
				}
			}
		}
	}
}

pub fn sin_wave_update_system(world: &mut World, master: &Master) {
	for (_entity, sin_wave) in &mut world.query::<&mut SinWave>() {
		sin_wave.value = f64::sin(master.time_since_start * sin_wave.speed + sin_wave.offset) * sin_wave.distance;
	}
}

pub fn text_render_system(world: &mut World, layer: &'static str) {
	for (_entity, (transform, text)) in &mut world.query::<(&Transform, &TextRenderer)>() {
		if text.render_layer == layer {
			draw_text_ex(
				text.text,
				transform.position.x.round(),
				transform.position.y.round(),
				text.params,
			);
		}
	}
}