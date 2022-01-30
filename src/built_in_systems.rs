use macroquad::rand::gen_range;
use hecs::Entity;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::Master;
use macroquad::prelude::*;
use crate::built_in_components::*;

//TODO
pub fn rigidbody_update_system(master: &mut Master) {
	for (_entity, (transform, rigidbody)) in &mut master.world.query::<(&mut Transform, &mut Rigidbody)>() {
		//rigidbody movement x
		//rigidbody map collision x
		//rigidbody entity collision x

		//rigidbody movement y
		//rigidbody map collision y
		//rigidbody entity collision y
	}
}

//TODO
pub fn button_update_system(master: &mut Master) {
	for (_entity, (transform, collider, button)) in &mut master.world.query::<(&Transform, &BoxCollider, &mut Button)>() {
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
		for (_entity, (transform, drop_shadow, texture)) in &mut master.world.query::<(&Transform, &DropShadow, &Texture)>() {
			if !rect_in_screen(&master.world, Rect {
				x: (transform.position.x + drop_shadow.offset.x).round(),
				y: (transform.position.y + drop_shadow.offset.y).round(),
				w: (texture.get_size().x * transform.scale.x.abs()).round(),
				h: (texture.get_size().y * transform.scale.y.abs()).round(),
			}) {
				continue;
			}

			draw_texture_ex(
				texture.texture,
				(transform.position.x - texture.get_size().x * transform.scale.x / 2.0 + texture.get_size().x / 2.0).round(),
				(transform.position.y - texture.get_size().y * transform.scale.y / 2.0 + texture.get_size().y / 2.0).round(),
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

		for (_entity, (transform, drop_shadow, rectangle)) in &mut master.world.query::<(&Transform, &DropShadow, &Rectangle)>() {
			if !rect_in_screen(&master.world, Rect {
				x: (transform.position.x + drop_shadow.offset.x).round(),
				y: (transform.position.y + drop_shadow.offset.y).round(),
				w: (rectangle.size.x * transform.scale.x).round(),
				h: (rectangle.size.y * transform.scale.y).round(),
			}) {
				continue;
			}

			draw_rectangle(
				transform.position.x.round(),
				transform.position.y.round(),
				(rectangle.size.x * transform.scale.x).round(),
				(rectangle.size.y * transform.scale.y).round(),
				rectangle.color,
			);
		}
	}
}

pub fn texture_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, texture, render_layer)) in &mut master.world.query::<(&Transform, &Texture, &RenderLayer)>() {
		if layer == render_layer.0 {
			if !rect_in_screen(&master.world, Rect {
				x: transform.position.x.round(),
				y: transform.position.y.round(),
				w: (texture.get_size().x * transform.scale.x.abs()).round(),
				h: (texture.get_size().y * transform.scale.y.abs()).round(),
			}) {
				continue;
			}

			draw_texture_ex(
				texture.texture,
				(transform.position.x - texture.get_size().x * transform.scale.x / 2.0 + texture.get_size().x / 2.0).round(),
				(transform.position.y - texture.get_size().y * transform.scale.y / 2.0 + texture.get_size().y / 2.0).round(),
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

pub fn map_render_system(master: &Master, layer: &'static str) {
	for (_entity, (map, texture, render_layer)) in &mut master.world.query::<(&Map, &Texture, &RenderLayer)>() {
		if layer == render_layer.0 {
			for y in 0..map.tiles.len() {
				for x in 0..map.tiles[0].len() {
					if map.tiles[y][x] != 0
					&& rect_in_screen(&master.world, Rect {
						x: x as f32 * map.tile_size as f32,
						y: y as f32 * map.tile_size as f32,
						w: map.tile_size as f32,
						h: map.tile_size as f32,
					}) {
						draw_texture_ex(
							texture.texture,
							x as f32 * map.tile_size as f32,
							y as f32 * map.tile_size as f32,
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
}

pub fn rectangle_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, rectangle, render_layer)) in &mut master.world.query::<(&Transform, &Rectangle, &RenderLayer)>() {
		if layer == render_layer.0 {
			if !rect_in_screen(&master.world, Rect {
				x: transform.position.x.round(),
				y: transform.position.y.round(),
				w: rectangle.size.x.round(),
				h: rectangle.size.y.round(),
			}) {
				continue;
			}

			draw_rectangle(
				transform.position.x.round(),
				transform.position.y.round(),
				rectangle.size.x.round(),
				rectangle.size.y.round(),
				rectangle.color,
			);
		}
	}
}

pub fn text_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, text, render_layer)) in &mut master.world.query::<(&Transform, &TextRenderer, &RenderLayer)>() {
		if layer == render_layer.0 {
			draw_text_ex(
				&text.text,
				transform.position.x.round(),
				transform.position.y.round(),
				TextParams {
					font_scale: (text.params.font_scale * transform.scale.y).round(),
					font_scale_aspect: (text.params.font_scale_aspect * transform.scale.x / transform.scale.y).round(),
					..text.params
				},
			);
		}
	}
}