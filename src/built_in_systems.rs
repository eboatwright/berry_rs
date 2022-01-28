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

//TODO
pub fn camera_update_system(master: &mut Master) {
	for (_entity, ()) in &mut master.world.query::<()>() {
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

//TODO
pub fn map_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, render_layer)) in &mut master.world.query::<(&Transform, &RenderLayer)>() {
		if layer == render_layer.0 {
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