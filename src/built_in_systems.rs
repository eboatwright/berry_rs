use macroquad::audio::PlaySoundParams;
use macroquad::audio::play_sound;
use ::rand::thread_rng;
use ::rand::Rng;
use crate::util::get_mouse_position;
use crate::util::clamp_range;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::util::delta_time;
use crate::Master;
use crate::built_in_components::*;
use macroquad::prelude::*;
use hecs::World;
use hecs::Entity;

pub fn rigidbody2d_update_system(world: &mut World) {
	for (entity, (transform, rigidbody2d)) in &mut world.query::<(&mut Transform, &mut Rigidbody2D)>() {
		rigidbody2d.grounded -= delta_time();

		rigidbody2d.velocity.x += rigidbody2d.gravity.x;
		rigidbody2d.velocity.x *= 1.0 - rigidbody2d.friction.x;
		transform.position.x += rigidbody2d.velocity.x * delta_time();

		if let Ok(collider) = world.get::<BoxCollider2D>(entity) {
			for (_map_entity, map) in &mut world.query::<&Map>() {
				let mut tile_transform = Transform::default();
				let mut tile_collider;
				let mut collision = (true, true, true, true);
				let mut tile_pos = vec2(transform.position.x / map.tile_size as f32, transform.position.y / map.tile_size as f32);
				tile_pos.y = clamp_range(0.0, tile_pos.y, map.tiles.len() as f32 - 1.0);
				tile_pos.x = clamp_range(0.0, tile_pos.x, map.tiles[0].len() as f32 - 1.0);
				for y in (tile_pos.y - 10.0) as usize..(tile_pos.y + 10.0) as usize {
					for x in (tile_pos.x - 10.0) as usize..(tile_pos.x + 10.0) as usize {
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

			for (_other_entity, (other_transform, other_collider)) in &mut world.query::<(&Transform, &BoxCollider2D)>().without::<Rigidbody2D>() {
				if collider.overlaps(transform, other_collider, other_transform) {
					if rigidbody2d.velocity.x < 0.0 {
						rigidbody2d.velocity.x = 0.0;
						transform.position.x = other_transform.position.x + other_collider.offset.x - collider.offset.x + other_collider.size.x;
						if rigidbody2d.gravity.x < 0.0 {
							rigidbody2d.grounded = rigidbody2d.grounded_time;
						}
					}
					if rigidbody2d.velocity.x > 0.0 {
						rigidbody2d.velocity.x = 0.0;
						transform.position.x = other_transform.position.x + other_collider.offset.x - collider.size.x - collider.offset.x;
						if rigidbody2d.gravity.x > 0.0 {
							rigidbody2d.grounded = rigidbody2d.grounded_time;
						}
					}
				}
			}
		}

		rigidbody2d.velocity.y += rigidbody2d.gravity.y;
		rigidbody2d.velocity.y *= 1.0 - rigidbody2d.friction.y;
		transform.position.y += rigidbody2d.velocity.y * delta_time();

		if let Ok(collider) = world.get::<BoxCollider2D>(entity) {
			for (_map_entity, map) in &mut world.query::<&Map>() {
				let mut tile_transform = Transform::default();
				let mut tile_collider;
				let mut collision = (true, true, true, true);
				let mut tile_pos = vec2(transform.position.x / map.tile_size as f32, transform.position.y / map.tile_size as f32);
				tile_pos.y = clamp_range(0.0, tile_pos.y, map.tiles.len() as f32);
				tile_pos.x = clamp_range(0.0, tile_pos.x, map.tiles[0].len() as f32);
				for y in (tile_pos.y - 10.0) as usize..(tile_pos.y + 10.0) as usize {
					for x in (tile_pos.x - 10.0) as usize..(tile_pos.x + 10.0) as usize {
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

			for (_other_entity, (other_transform, other_collider)) in &mut world.query::<(&Transform, &BoxCollider2D)>().without::<Rigidbody2D>() {
				if collider.overlaps(transform, other_collider, other_transform) {
					if rigidbody2d.velocity.y < 0.0 {
						rigidbody2d.velocity.y = 0.0;
						transform.position.y = other_transform.position.y + other_collider.size.y - collider.offset.y + other_collider.offset.y;
						if rigidbody2d.gravity.y < 0.0 {
							rigidbody2d.grounded = rigidbody2d.grounded_time;
						}
					}
					if rigidbody2d.velocity.y > 0.0 {
						rigidbody2d.velocity.y = 0.0;
						transform.position.y = other_transform.position.y - collider.size.y - collider.offset.y + other_collider.offset.y;
						if rigidbody2d.gravity.y > 0.0 {
							rigidbody2d.grounded = rigidbody2d.grounded_time;
						}
					}
				}
			}
		}
	}
}

pub fn button_update_system(world: &mut World, master: &mut Master) {
	let mut functions: Vec<fn(&mut World, &mut Master)> = Vec::new();
	let mut to_update_shadow: Vec<(bool, Entity)> = Vec::new();
	for (entity, (transform, collider, button)) in &mut world.query::<(&Transform, &BoxCollider2D, &mut Button)>() {
		let mouse_transform = Transform {
			position: get_mouse_position(master).extend(0.0),
			..Default::default()
		};
		let mouse_collider = BoxCollider2D {
			size: vec2(1.0, 1.0),
			offset: Vec2::ZERO,
		};
		
		let mut target_offset = Vec2::ZERO;
		if mouse_collider.overlaps(&mouse_transform, collider, transform) {
			if !button.selected {
				button.selected = true;
				if let Some(sfx) = button.select_sfx {
					play_sound(sfx, PlaySoundParams { looped: false, volume: 1.0 });
				}
			}
			if world.get::<DropShadow>(entity).is_err() {
				to_update_shadow.push((true, entity));
			}
			target_offset = button.highlight_offset;
			if is_mouse_button_pressed(MouseButton::Left) {
				functions.push(button.function);
			}
		} else {
			button.selected = false;
			if world.get::<DropShadow>(entity).is_ok() {
				to_update_shadow.push((false, entity));
			}
		}

		if let Ok(mut render_offset) = world.get_mut::<RenderOffset>(entity) {
			render_offset.0 = render_offset.0.lerp(target_offset, button.animation_smooth);
		}
	}
	for function in functions {
		function(world, master);
	}
	for entity in to_update_shadow {
		if entity.0 {
			world.insert_one(entity.1, DropShadow {
				color: Color {
					r: 0.0,
					g: 0.0,
					b: 0.0,
					a: 0.4,
				},
				offset: vec2(0.0, 2.0),
			}).unwrap();
		} else {
			world.remove_one::<DropShadow>(entity.1).unwrap();
		}
	}
}

pub fn animator_update_system(world: &mut World) {
	for (entity, animator) in &mut world.query::<&mut Animator>() {
		animator.animation_timer -= delta_time();
		if animator.animation_timer <= 0.0 {
			animator.animation_timer = animator.current_animation.frame_duration;
			animator.animation_frame_index += 1;
			if animator.animation_frame_index >= animator.current_animation.frames.len() {
				if !animator.dont_interrupt {
					animator.animation_frame_index = 0;
				} else {
					animator.animation_frame_index -= 1;
				}
				animator.dont_interrupt = false;
			}
			if world.get::<DontAnimateTexture>(entity).is_err() {
				if let Ok(mut texture) = world.get_mut::<Texture>(entity) {
					texture.source.x = animator.current_frame() as f32 * texture.size().x;
				}
			}
		}
	}
}

pub fn particle_update_system(world: &mut World) {
	let mut to_spawn: Vec<(Transform, Rigidbody2D, Particle, Texture, RenderLayer)> = Vec::new();
	let mut rng = thread_rng();
	for (_entity, (transform, particle_spawner)) in &mut world.query::<(&Transform, &mut ParticleSpawner)>() {
		particle_spawner.spawn_timer -= delta_time();
		if particle_spawner.spawn_timer <= 0.0 {
			particle_spawner.spawn_timer = particle_spawner.spawn_rate;
			to_spawn.push((
				Transform {
					position: vec3(transform.position.x + rng.gen_range(particle_spawner.min_spawn_offset.x..particle_spawner.max_spawn_offset.x), transform.position.y + rng.gen_range(particle_spawner.min_spawn_offset.y..particle_spawner.max_spawn_offset.y), 0.0),
					scale: Vec3::ONE,
					rotation: Vec3::ZERO,
				},
				Rigidbody2D {
					velocity: vec2(rng.gen_range(particle_spawner.min_velocity.x..particle_spawner.max_velocity.x), rng.gen_range(particle_spawner.min_velocity.y..particle_spawner.max_velocity.y)),
					friction: particle_spawner.friction,
					gravity: particle_spawner.gravity,
					..Default::default()
				},
				Particle {
					life: particle_spawner.life,
				},
				Texture {
					texture: particle_spawner.texture,
					color: particle_spawner.color,
					..Default::default()
				},
				RenderLayer("particle"),
			));
		}
	}
	world.spawn_batch(to_spawn);

	let mut to_destroy: Vec<Entity> = Vec::new();
	for (entity, particle) in &mut world.query::<&mut Particle>() {
		particle.life -= delta_time();
		if particle.life <= 0.0 {
			to_destroy.push(entity);
			continue;
		}
	}
	for entity in to_destroy {
		world.despawn(entity).unwrap();
	}
}

pub fn sin_wave_update_system(world: &mut World, master: &Master) {
	for (entity, sin_wave) in &mut world.query::<&mut SinWave>() {
		sin_wave.value = f64::sin(master.time_since_start * sin_wave.speed + sin_wave.offset) * sin_wave.distance;
		if let Ok(mut render_offset) = world.get_mut::<RenderOffset>(entity) {
			render_offset.0.y = sin_wave.value as f32;
		}
	}
}

pub fn camera_update_system(world: &mut World, master: &mut Master) {
	for (_entity, (transform, camera_target)) in &mut world.query::<(&Transform, &CameraTarget)>() {
		master.camera_pos = master.camera_pos.lerp((transform.position + camera_target.offset).truncate(), camera_target.smoothing).round();
	}

	for (_entity, (transform, follow_camera)) in &mut world.query::<(&mut Transform, &FollowCamera)>() {
		transform.position = (master.camera_pos.extend(0.0) + follow_camera.offset).round();
	}
}

pub fn follow_update_system(world: &mut World) {
	for (_entity, (follower_transform, follow)) in &mut world.query::<(&mut Transform, &Follow)>() {
		for (target_entity, target_transform) in &mut world.query::<&Transform>().without::<Follow>() {
			if follow.id == target_entity.id() {
				follower_transform.position = target_transform.position + follow.offset;
			}
		}
	}
}

pub fn texture_render_system(world: &mut World, camera_pos: Vec2, layer: &'static str) {
	for (entity, (transform, texture, render_layer)) in &mut world.query::<(&Transform, &Texture, &RenderLayer)>() {
		if render_layer.0 == layer {
			let mut x_pos = transform.position.x - texture.size().x * transform.scale.x / 2.0 + texture.size().x / 2.0;
			let mut y_pos = transform.position.y - texture.size().y * transform.scale.y / 2.0 + texture.size().y / 2.0;

			if let Ok(render_offset) = world.get::<RenderOffset>(entity) {
				x_pos += render_offset.0.x;
				y_pos += render_offset.0.y;
			}

			if x_pos + texture.size().x * transform.scale.x < camera_pos.x - SCREEN_WIDTH as f32 / 2.0
			|| x_pos > camera_pos.x + SCREEN_WIDTH as f32 / 2.0
			|| y_pos + texture.size().y * transform.scale.y < camera_pos.y - SCREEN_HEIGHT as f32 / 2.0
			|| y_pos > camera_pos.y + SCREEN_HEIGHT as f32 / 2.0 {
				continue;
			}

			if let Ok(drop_shadow) = world.get::<DropShadow>(entity) {
				draw_texture_ex(
					texture.texture,
					x_pos.round() + drop_shadow.offset.x.round(),
					y_pos.round() + drop_shadow.offset.y.round(),
					drop_shadow.color,
					DrawTextureParams {
						dest_size: Some(vec2(texture.size().x * transform.scale.x, texture.size().y * transform.scale.y)),
						source: Some(if texture.source == Rect::default() {
							 Rect {
								x: 0.0,
								y: 0.0,
								w: texture.size().x,
								h: texture.size().y,
							}
						} else {
							texture.source
						}),
						rotation: transform.rotation.z,
						flip_x: false,
						flip_y: false,
						pivot: None,
					},
				);
			}

			draw_texture_ex(
				texture.texture,
				x_pos.round(),
				y_pos.round(),
				texture.color,
				DrawTextureParams {
					dest_size: Some(vec2(texture.size().x * transform.scale.x, texture.size().y * transform.scale.y)),
					source: Some(if texture.source == Rect::default() {
						 Rect {
							x: 0.0,
							y: 0.0,
							w: texture.size().x,
							h: texture.size().y,
						}
					} else {
						texture.source
					}),
					rotation: transform.rotation.z,
					flip_x: false,
					flip_y: false,
					pivot: None,
				},
			);
		}
	}
}

pub fn map_render_system(world: &mut World, camera_pos: Vec2, layer: &'static str) {
	for (_entity, (map, texture, render_layer)) in &mut world.query::<(&Map, &Texture, &RenderLayer)>() {
		if render_layer.0 == layer {
			for y in 0..map.tiles.len() {
				for x in 0..map.tiles[0].len() {
					if map.tiles[y][x] != 0
					&& (y as f32 + 1.0) * (map.tile_size as f32) > camera_pos.y - SCREEN_HEIGHT as f32 / 2.0
					&& (y as f32) * (map.tile_size as f32) < camera_pos.y + SCREEN_HEIGHT as f32 / 2.0
					&& (x as f32 + 1.0) * (map.tile_size as f32) > camera_pos.x - SCREEN_WIDTH as f32 / 2.0
					&& (x as f32) * (map.tile_size as f32) < camera_pos.x + SCREEN_WIDTH as f32 / 2.0 {
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

pub fn rectangle_render_system(world: &mut World, camera_pos: Vec2, layer: &'static str) {
	for (_entity, (transform, rectangle, render_layer)) in &mut world.query::<(&Transform, &Rectangle, &RenderLayer)>() {
		if render_layer.0 == layer
		&& transform.position.y + rectangle.size.y > camera_pos.y - SCREEN_HEIGHT as f32 / 2.0
		&& transform.position.y < camera_pos.y + SCREEN_HEIGHT as f32 / 2.0
		&& transform.position.x + rectangle.size.x > camera_pos.x - SCREEN_WIDTH as f32 / 2.0
		&& transform.position.x < camera_pos.x + SCREEN_WIDTH as f32 / 2.0 {
			draw_rectangle(
				transform.position.x.round(),
				transform.position.y.round(),
				rectangle.size.x,
				rectangle.size.y,
				rectangle.color,
			);
		}
	}
}

pub fn text_render_system(world: &mut World, layer: &'static str) {
	for (entity, (transform, text, render_layer)) in &mut world.query::<(&Transform, &TextRenderer, &RenderLayer)>() {
		if render_layer.0 == layer {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}
			if let Ok(drop_shadow) = world.get::<DropShadow>(entity) {
				draw_text_ex(
					text.text,
					transform.position.x.round() + drop_shadow.offset.x,
					(transform.position.y + text.params.font_size as f32 / 1.5 + offset.y).round() + drop_shadow.offset.y,
					TextParams {
						color: drop_shadow.color,
						..text.params
					},
				);
			}
			draw_text_ex(
				text.text,
				transform.position.x.round() + offset.x,
				(transform.position.y + text.params.font_size as f32 / 1.5 + offset.y).round(),
				text.params,
			);
		}
	}
}