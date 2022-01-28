use crate::Master;
use macroquad::prelude::*;
use crate::built_in_components::*;

//TODO
pub fn rigidbody2d_update_system(master: &mut Master) {
}

//TODO
pub fn button_update_system(master: &mut Master) {
}

//TODO
pub fn slider_update_system(master: &mut Master) {
}

//TODO
pub fn animator_update_system(master: &mut Master) {
}

//TODO
pub fn particle_update_system(master: &mut Master) {
}

//TODO
pub fn camera_update_system(master: &mut Master) {
}

//TODO
pub fn texture_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, render_layer)) in &mut master.world.query::<(&Transform, &RenderLayer)>() {
		if layer == render_layer.0 {
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

//TODO
pub fn text_render_system(master: &Master, layer: &'static str) {
	for (_entity, (transform, render_layer)) in &mut master.world.query::<(&Transform, &RenderLayer)>() {
		if layer == render_layer.0 {
		}
	}
}