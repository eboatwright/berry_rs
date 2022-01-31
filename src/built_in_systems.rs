use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;
use macroquad::rand::gen_range;
use hecs::Entity;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::Master;
use macroquad::prelude::*;
use crate::built_in_components::*;

pub fn rigidbody_update_system(master: &mut Master) {
	for (entity, (transform, rigidbody)) in &mut master.world.query::<(&mut Transform, &mut Rigidbody)>().without::<Static>() {
		rigidbody.velocity.x += rigidbody.gravity.x;
		rigidbody.velocity.x *= 1.0 - rigidbody.friction.x;
		transform.position.x += rigidbody.velocity.x;

		if let Ok(collider) = master.world.get::<BoxCollider>(entity) {
			for (_entity, map) in &mut master.world.query::<&Map>() {
				let mut tile_transform = Transform::default();
				let mut tile_collision: (BoxCollider, bool, bool, bool, bool);
				let tile_pos = transform.position.round() / map.tile_size as f32;

				for y in clamp(tile_pos.y - 10.0, 0.0, map.tiles.len() as f32) as usize..clamp(tile_pos.y + 10.0, 0.0, map.tiles.len() as f32) as usize {
					for x in clamp(tile_pos.x - 10.0, 0.0, map.tiles[0].len() as f32) as usize..clamp(tile_pos.x + 10.0, 0.0, map.tiles[0].len() as f32) as usize {
						println!("{}, {}", x, y);
						if map.tiles[y][x] != 0 {
							tile_transform.position = vec2(x as f32, y as f32) * map.tile_size as f32;
							tile_collision = match map.collisions.get(&map.tiles[y][x]) {
								Some(collision) => {
									*collision
								},
								None => (BoxCollider {
									size: vec2(map.tile_size as f32, map.tile_size as f32),
									offset: Vec2::ZERO,
								}, true, true, true, true),
							};
							if BoxCollider::overlaps((&collider, transform), (&tile_collision.0, &tile_transform)) {
								if rigidbody.velocity.x < 0.0
								&& tile_collision.3 {
									rigidbody.velocity.x = 0.0;
									transform.position.x = tile_transform.position.x + tile_collision.0.size.x - collider.offset.x + tile_collision.0.offset.x;
									if rigidbody.gravity.x < 0.0 {
										rigidbody.grounded = rigidbody.grounded_time;
									}
								}
								if rigidbody.velocity.x > 0.0
								&& tile_collision.4 {
									rigidbody.velocity.x = 0.0;
									transform.position.x = tile_transform.position.x - collider.size.x - collider.offset.x + tile_collision.0.offset.x;
									if rigidbody.gravity.x > 0.0 {
										rigidbody.grounded = rigidbody.grounded_time;
									}
								}
							}
						}
					}
				}
			}

			for (_other_entity, (other_transform, other_collider)) in &mut master.world.query::<(&Transform, &BoxCollider)>().with::<Static>() {
				if BoxCollider::overlaps((&collider, transform), (other_collider, other_transform)) {
					if rigidbody.velocity.x < 0.0 {
						rigidbody.velocity.x = 0.0;
						transform.position.x = other_transform.position.x + other_collider.size.x - collider.offset.x + other_collider.offset.x;
						if rigidbody.gravity.x < 0.0 {
							rigidbody.grounded = rigidbody.grounded_time;
						}
					}
					if rigidbody.velocity.x > 0.0 {
						rigidbody.velocity.x = 0.0;
						transform.position.x = other_transform.position.x - collider.size.x - collider.offset.x + other_collider.offset.x;
						if rigidbody.gravity.x > 0.0 {
							rigidbody.grounded = rigidbody.grounded_time;
						}
					}
				}
			}
		}

		rigidbody.velocity.y += rigidbody.gravity.y;
		rigidbody.velocity.y *= 1.0 - rigidbody.friction.y;
		transform.position.y += rigidbody.velocity.y;

		if let Ok(collider) = master.world.get::<BoxCollider>(entity) {
			for (_entity, map) in &mut master.world.query::<&Map>() {
				let mut tile_transform = Transform::default();
				let mut tile_collision: (BoxCollider, bool, bool, bool, bool);
				let tile_pos = transform.position.round() / map.tile_size as f32;

				for y in clamp(tile_pos.y - 10.0, 0.0, map.tiles.len() as f32 - 1.0) as usize..clamp(tile_pos.y + 10.0, 0.0, map.tiles.len() as f32 - 1.0) as usize {
					for x in clamp(tile_pos.x - 10.0, 0.0, map.tiles[0].len() as f32 - 1.0) as usize..clamp(tile_pos.x + 10.0, 0.0, map.tiles[0].len() as f32 - 1.0) as usize {
						if map.tiles[y][x] != 0 {
							tile_transform.position = vec2(x as f32, y as f32) * map.tile_size as f32;
							tile_collision = match map.collisions.get(&map.tiles[y][x]) {
								Some(collision) => {
									*collision
								},
								None => (BoxCollider {
									size: vec2(map.tile_size as f32, map.tile_size as f32),
									offset: Vec2::ZERO,
								}, true, true, true, true),
							};
							if BoxCollider::overlaps((&collider, transform), (&tile_collision.0, &tile_transform)) {
								if rigidbody.velocity.y < 0.0
								&& tile_collision.1 {
									rigidbody.velocity.y = 0.0;
									transform.position.y = tile_transform.position.y + tile_collision.0.size.y - collider.offset.y + tile_collision.0.offset.y;
									if rigidbody.gravity.y < 0.0 {
										rigidbody.grounded = rigidbody.grounded_time;
									}
								}
								if rigidbody.velocity.y > 0.0
								&& tile_collision.2 {
									rigidbody.velocity.y = 0.0;
									transform.position.y = tile_transform.position.y - collider.size.y - collider.offset.y + tile_collision.0.offset.y;
									if rigidbody.gravity.y > 0.0 {
										rigidbody.grounded = rigidbody.grounded_time;
									}
								}
							}
						}
					}
				}
			}

			for (_other_entity, (other_transform, other_collider)) in &mut master.world.query::<(&Transform, &BoxCollider)>().with::<Static>() {
				if BoxCollider::overlaps((&collider, transform), (other_collider, other_transform)) {
					if rigidbody.velocity.y < 0.0 {
						rigidbody.velocity.y = 0.0;
						transform.position.y = other_transform.position.y + other_collider.size.y - collider.offset.y + other_collider.offset.y;
						if rigidbody.gravity.y < 0.0 {
							rigidbody.grounded = rigidbody.grounded_time;
						}
					}
					if rigidbody.velocity.y > 0.0 {
						rigidbody.velocity.y = 0.0;
						transform.position.y = other_transform.position.y - collider.size.y - collider.offset.y + other_collider.offset.y;
						if rigidbody.gravity.y > 0.0 {
							rigidbody.grounded = rigidbody.grounded_time;
						}
					}
				}
			}
		}
	}
}

pub fn button_update_system(master: &mut Master) {
	let mut functions: Vec<(ButtonClickFunction, Entity)> = Vec::new();
	let mouse_pos = get_mouse_position(&master.world);
	for (entity, (transform, collider, button)) in &mut master.world.query::<(&Transform, &BoxCollider, &mut Button)>() {
		button.hovering_over = BoxCollider::overlaps(
			(collider, transform),
			(&BoxCollider {
				size: Vec2::ONE,
				..Default::default()
			}, &Transform {
				position: mouse_pos,
				..Default::default()
			}),
		);
		let mut offset = Vec2::ZERO;
		let mut shadow_offset = Vec2::ZERO;
		if button.hovering_over {
			if !button.played_select_sfx {
				button.played_select_sfx = true;
				if let Some(sfx) = button.select_sfx {
					play_sound(sfx, PlaySoundParams {
						looped: false,
						volume: 1.0,
					});
				}
			}
			offset = button.highlight_offset;
			shadow_offset = button.shadow_highlight_offset;
			if is_mouse_button_pressed(MouseButton::Left) {
				functions.push((button.function.clone(), entity));
			}
		} else {
			button.played_select_sfx = false;
		}

		if let Ok(mut render_offset) = master.world.get_mut::<RenderOffset>(entity) {
			render_offset.0 = render_offset.0.lerp(offset, button.animation_smooth);
		}

		if let Ok(mut drop_shadow) = master.world.get_mut::<DropShadow>(entity) {
			drop_shadow.offset = drop_shadow.offset.lerp(shadow_offset, button.animation_smooth);
		}
	}

	for function in functions {
		function.0.0(master, function.1);
	}
}

pub fn slider_update_system(master: &mut Master) {
	for (_entity, (transform, button, slider)) in &mut master.world.query::<(&mut Transform, &Button, &mut Slider)>() {
		if button.hovering_over
		&& is_mouse_button_down(MouseButton::Left) {
			slider.dragging = true;
		}

		if is_mouse_button_released(MouseButton::Left) {
			slider.dragging = false;
		}

		let mouse_pos = get_mouse_position(&master.world);
		if slider.dragging {
			if slider.vertical {
				transform.position.y = mouse_pos.y;
			} else {
				transform.position.x = mouse_pos.x;
			}
		}
	}
}

pub fn animator_update_system(master: &mut Master) {
	for (entity, animator) in &mut master.world.query::<&mut Animator>() {
		animator.timer -= delta_time();
		if animator.timer <= 0.0 {
			animator.timer = animator.current_animation.frame_duration;
			animator.current_frame_index += 1;
			if animator.current_frame_index >= animator.current_animation.frames.len() {
				if !animator.dont_interrupt {
					animator.current_frame_index = 0;
				} else {
					animator.current_frame_index -= 1;
				}
				animator.dont_interrupt = false;
			}
			if master.world.get::<DontAnimateTexture>(entity).is_err() {
				if let Ok(mut texture) = master.world.get_mut::<Texture>(entity) {
					texture.source = Some(Rect {
						x: animator.get_frame() * texture.get_size().x,
						..texture.source.unwrap()
					});
				}
			}
		}
	}
}

pub fn particle_update_system(master: &mut Master) {
	let mut to_spawn: Vec<(Transform, Rigidbody, Particle, Texture, RenderLayer)> = Vec::new();
	for (_entity, (transform, particle_spawner)) in &mut master.world.query::<(&Transform, &mut ParticleSpawner)>() {
		particle_spawner.spawn_timer -= delta_time();
		if particle_spawner.spawn_timer <= 0.0 {
			particle_spawner.spawn_timer = particle_spawner.spawn_rate;
			to_spawn.push((
				Transform {
					position: transform.position + vec2(gen_range(particle_spawner.min_spawn_offset.x, particle_spawner.max_spawn_offset.x), gen_range(particle_spawner.min_spawn_offset.y, particle_spawner.max_spawn_offset.y)),
					..Default::default()
				},
				Rigidbody {
					velocity: vec2(gen_range(particle_spawner.min_velocity.x, particle_spawner.max_velocity.x), gen_range(particle_spawner.min_velocity.y, particle_spawner.max_velocity.y)),
					..particle_spawner.particle_rigidbody
				},
				Particle {
					life: particle_spawner.particle_life,
				},
				particle_spawner.particle_texture,
				RenderLayer("particle".to_string()),
			));
		}
	}
	master.world.spawn_batch(to_spawn);

	let mut to_destroy: Vec<Entity> = Vec::new();
	for (entity, particle) in &mut master.world.query::<&mut Particle>() {
		particle.life -= delta_time();
		if particle.life <= 0.0 {
			to_destroy.push(entity);
		}
	}
	for entity in to_destroy {
		master.world.despawn(entity).unwrap();
	}
}

pub fn camera_update_system(master: &mut Master) {
	for (_entity, (transform, camera)) in &mut master.world.query::<(&mut Transform, &RenderCamera)>() {
		if let Some(target) = master.world.find_entity_from_id(camera.target) {
			let target_transform = master.world.get::<Transform>(target).unwrap();
			transform.position = transform.position.lerp(
				target_transform.position + camera.follow_offset,
				camera.smoothing,
			);
		}
	}
}

pub fn drop_shadow_render_system(master: &Master, layer: &'static str) {
	if layer == "shadow" {
		for (entity, (transform, drop_shadow, texture)) in &mut master.world.query::<(&Transform, &DropShadow, &Texture)>() {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = master.world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}

			if !rect_in_screen(&master.world, Rect {
				x: (transform.position.x + drop_shadow.offset.x + offset.x).round(),
				y: (transform.position.y + drop_shadow.offset.y + offset.y).round(),
				w: (texture.get_size().x * transform.scale.x.abs()).round(),
				h: (texture.get_size().y * transform.scale.y.abs()).round(),
			}) {
				continue;
			}

			draw_texture_ex(
				texture.texture,
				(transform.position.x - texture.get_size().x * transform.scale.x / 2.0 + texture.get_size().x / 2.0 + offset.x).round(),
				(transform.position.y - texture.get_size().y * transform.scale.y / 2.0 + texture.get_size().y / 2.0 + offset.y).round(),
				drop_shadow.color,
				DrawTextureParams {
					dest_size: Some(texture.get_size() * transform.scale),
					source: texture.source,
					rotation: transform.rotation,
					flip_x: false,
					flip_y: false,
					pivot: None,
				}
			);
		}

		for (entity, (transform, drop_shadow, rectangle)) in &mut master.world.query::<(&Transform, &DropShadow, &Rectangle)>() {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = master.world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}

			if !rect_in_screen(&master.world, Rect {
				x: (transform.position.x + drop_shadow.offset.x + offset.x).round(),
				y: (transform.position.y + drop_shadow.offset.y + offset.y).round(),
				w: (rectangle.size.x * transform.scale.x).round(),
				h: (rectangle.size.y * transform.scale.y).round(),
			}) {
				continue;
			}

			draw_rectangle(
				(transform.position.x + drop_shadow.offset.x + offset.x).round(),
				(transform.position.y + drop_shadow.offset.y + offset.y).round(),
				(rectangle.size.x * transform.scale.x).round(),
				(rectangle.size.y * transform.scale.y).round(),
				drop_shadow.color,
			);
		}
	}
}

pub fn texture_render_system(master: &Master, layer: &'static str) {
	for (entity, (transform, texture, render_layer)) in &mut master.world.query::<(&Transform, &Texture, &RenderLayer)>() {
		if layer == render_layer.0 {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = master.world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}

			if !rect_in_screen(&master.world, Rect {
				x: (transform.position.x + offset.x).round(),
				y: (transform.position.y + offset.y).round(),
				w: (texture.get_size().x * transform.scale.x.abs()).round(),
				h: (texture.get_size().y * transform.scale.y.abs()).round(),
			}) {
				continue;
			}

			draw_texture_ex(
				texture.texture,
				(transform.position.x - texture.get_size().x * transform.scale.x / 2.0 + texture.get_size().x / 2.0 + offset.x).round(),
				(transform.position.y - texture.get_size().y * transform.scale.y / 2.0 + texture.get_size().y / 2.0 + offset.y).round(),
				texture.color,
				DrawTextureParams {
					dest_size: Some((texture.get_size() * transform.scale).round()),
					source: texture.source,
					rotation: transform.rotation,
					flip_x: false,
					flip_y: false,
					pivot: None,
				}
			);
		}
	}
}

// TODO: implement transform?
pub fn map_render_system(master: &Master, layer: &'static str) {
	for (entity, (map, texture, render_layer)) in &mut master.world.query::<(&Map, &Texture, &RenderLayer)>() {
		if layer == render_layer.0 {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = master.world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}
			for y in 0..map.tiles.len() {
				for x in 0..map.tiles[0].len() {
					if map.tiles[y][x] == 0
					&& !rect_in_screen(&master.world, Rect {
						x: x as f32 * map.tile_size as f32 + offset.x,
						y: y as f32 * map.tile_size as f32 + offset.y,
						w: map.tile_size as f32,
						h: map.tile_size as f32,
					}) {
						continue;
					}
					draw_texture_ex(
						texture.texture,
						x as f32 * map.tile_size as f32 + offset.x,
						y as f32 * map.tile_size as f32 + offset.y,
						match map.colors.get(&(map.tiles[y][x] - 1)) {
							Some(color) => *color,
							None => WHITE,
						},
						DrawTextureParams {
							dest_size: Some(vec2(map.tile_size as f32, map.tile_size as f32)),
							source: Some(Rect {
								x: (map.tiles[y][x] - 1) as f32 * map.tile_size as f32,
								y: match map.y_source_offsets.get(&(map.tiles[y][x] - 1)) {
									Some(y) => *y,
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

pub fn rectangle_render_system(master: &Master, layer: &'static str) {
	for (entity, (transform, rectangle, render_layer)) in &mut master.world.query::<(&Transform, &Rectangle, &RenderLayer)>() {
		if layer == render_layer.0 {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = master.world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}

			if !rect_in_screen(&master.world, Rect {
				x: (transform.position.x + offset.x).round(),
				y: (transform.position.y + offset.y).round(),
				w: rectangle.size.x.round(),
				h: rectangle.size.y.round(),
			}) {
				continue;
			}

			draw_rectangle(
				(transform.position.x + offset.x).round(),
				(transform.position.y + offset.y).round(),
				rectangle.size.x.round(),
				rectangle.size.y.round(),
				rectangle.color,
			);
		}
	}
}

pub fn text_render_system(master: &Master, layer: &'static str) {
	for (entity, (transform, text, render_layer)) in &mut master.world.query::<(&Transform, &TextRenderer, &RenderLayer)>() {
		if layer == render_layer.0 {
			let mut offset = Vec2::ZERO;
			if let Ok(render_offset) = master.world.get::<RenderOffset>(entity) {
				offset = render_offset.0;
			}

			// TODO: Check if in screen

			draw_text_ex(
				&text.text,
				(transform.position.x + offset.x).round(),
				(transform.position.y + offset.y).round(),
				TextParams {
					font_scale: (text.params.font_scale * transform.scale.y).round(),
					font_scale_aspect: (text.params.font_scale_aspect * transform.scale.x / transform.scale.y).round(),
					..text.params
				},
			);
		}
	}
}