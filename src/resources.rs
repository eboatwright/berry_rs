use macroquad::audio::play_sound;
use macroquad::audio::PlaySoundParams;
use crate::util::load_sound_file;
use macroquad::audio::Sound;
use crate::util::load_font_file;
use crate::util::load_texture_file;
use macroquad::prelude::*;

pub struct Resources {
}

impl Resources {
	pub fn new() -> Resources {
		Resources {
		}
	}

	pub async fn load(&mut self) {
	}

	pub fn play_sound(&self, sfx: Option<Sound>, looped: bool, volume: f32) {
		if sfx.is_none() {
			panic!("sound effect err");
		}
		play_sound(sfx.unwrap(), PlaySoundParams {
			looped,
			volume,
		});
	}
}