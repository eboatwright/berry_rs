use macroquad::{
	prelude::*,
	audio,
	audio::Sound,
};
use crate::Master;
use std::env;

pub fn clamp_f32(min: f32, val: f32, max: f32) -> f32 {
	if val > max { max }
	else if val < min { min }
	else { val }
}

pub fn clamp_i32(min: i32, val: i32, max: i32) -> i32 {
	if val > max { max }
	else if val < min { min }
	else { val }
}

pub fn clamp_usize(min: usize, val: usize, max: usize) -> usize {
	if val > max { max }
	else if val < min { min }
	else { val }
}

pub fn delta_time() -> f32 { get_frame_time() * 60.0 }

pub fn get_file_path(path: &'static str) -> String {
	return if cfg!(wasm32_unknown_unknown) {
		path.to_string()
	} else {
		let mut full_path = env::current_exe().unwrap();
		full_path.pop();
		full_path.push(&path);
		full_path.as_os_str().to_str().unwrap().to_string()
	}
}

pub async fn load_texture_file(file_path: &'static str) -> Texture2D {
	load_texture(&get_file_path(file_path)).await.unwrap()
}

pub async fn load_sound_file(file_path: &'static str) -> Sound {
	audio::load_sound(&get_file_path(file_path)).await.unwrap()
}

pub async fn load_font_file(file_path: &'static str) -> Font {
	load_ttf_font(&get_file_path(file_path)).await.unwrap()
}

pub fn get_mouse_position(master: &Master) -> Vec2 {
	let mut mouse_pos = mouse_position();
	mouse_pos.0 -= screen_width() / 2.0;
	mouse_pos.0 /= master.zoom;
	mouse_pos.0 += master.camera_pos.x;

	mouse_pos.1 -= screen_height() / 2.0;
	mouse_pos.1 /= master.zoom;
	mouse_pos.1 += master.camera_pos.y;

	vec2(mouse_pos.0, mouse_pos.1)
}