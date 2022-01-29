use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::Master;
use macroquad::prelude::*;
use crate::built_in_components::*;

//TODO
pub fn rigidbody_update_system(master: &mut Master) {
	for (_entity, ()) in &mut master.world.query::<()>() {
	}
}

//TODO
pub fn button_update_system(master: &mut Master) {
	for (_entity, ()) in &mut master.world.query::<()>() {
	}
}

//TODO
pub fn slider_update_system(master: &mut Master) {
	for (_entity, ()) in &mut master.world.query::<()>() {
	}
}

//TODO
pub fn animator_update_system(master: &mut Master) {
	for (_entity, ()) in &mut master.world.query::<()>() {
	}
}

//TODO
pub fn particle_update_system(master: &mut Master) {
	for (_entity, ()) in &mut master.world.query::<()>() {
	}
}

pub fn camera_update_system(master: &mut Master) {
	for (_entity, (transform, camera)) in &mut master.world.query::<(&mut Transform, &RenderCamera)>() {
		if let Some(target) = master.world.find_entity_from_id(camera.target) {
			let target_transform = master.world.get::<Transform>(target).unwrap();
			transform.position = transform.position.lerp(
				target_transform.position,
				camera.smoothing,
			);
		}
	}
}

pub fn texture_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, texture, render_layer)) in &mut master.world.query::<(&Transform, &Texture, &RenderLayer)>() {
		if layer == render_layer.0 {
			draw_texture_ex(
				texture.texture,
				transform.position.x.round(),
				transform.position.y.round(),
				texture.color,
				DrawTextureParams {
					dest_size: Some(texture.get_size()),
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
	let mut camera_pos = Vec2::ZERO;
	for (_entity, (transform, _camera)) in &mut master.world.query::<(&Transform, &RenderCamera)>() {
		camera_pos = transform.position;
	}
	for (_entity, (map, texture, render_layer)) in &mut master.world.query::<(&Map, &Texture, &RenderLayer)>() {
		if layer == render_layer.0 {
			for y in 0..map.tiles.len() {
				for x in 0..map.tiles[0].len() {
					if map.tiles[y][x] != 0
					&& (y as f32 + 1.0) * map.tile_size as f32 > camera_pos.y - SCREEN_HEIGHT as f32 * 0.5
					&& y as f32 * (map.tile_size as f32) < camera_pos.y + SCREEN_HEIGHT as f32 * 0.5
					&& (x as f32 + 1.0) * map.tile_size as f32 > camera_pos.x - SCREEN_WIDTH as f32 * 0.5
					&& x as f32 * (map.tile_size as f32) < camera_pos.x + SCREEN_WIDTH as f32 * 0.5 {
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
					font_scale: text.params.font_scale * transform.scale.y,
					font_scale_aspect: text.params.font_scale_aspect * transform.scale.x / transform.scale.y,
					..text.params
				},
			);
		}
	}
}