use std::env;
use macroquad::prelude::*;
use macroquad::audio;
use macroquad::audio::Sound;
use crate::Master;

pub fn clamp_range<T: std::cmp::PartialOrd>(min: T, val: T, max: T) -> T {
	if val > max { max }
	else if val < min { min }
	else { val }
}

pub fn delta_time() -> f32 { get_frame_time() * 60.0 }

pub fn get_file_path(path: String) -> String {
	return if cfg!(wasm32_unknown_unknown) {
		path
	} else {
		let mut full_path = env::current_exe().unwrap();
		full_path.pop();
		full_path.push(&path);
		full_path.as_os_str().to_str().unwrap().to_string()
	}
}

pub async fn load_texture_file(file_path: String) -> Texture2D {
	load_texture(&get_file_path(file_path)).await.unwrap()
}

pub async fn load_sound_file(file_path: String) -> Sound {
	audio::load_sound(&get_file_path(file_path)).await.unwrap()
}

pub async fn load_font_file(file_path: String) -> Font {
	load_ttf_font(&get_file_path(file_path)).await.unwrap()
}

pub fn get_mouse_position(master: &Master) -> Vec2 {
	let mut mouse_pos = mouse_position();
	mouse_pos.0 -= screen_width() / 2.0;
	mouse_pos.0 /= master.zoom;
	mouse_pos.0 += master.camera.x;

	mouse_pos.1 -= screen_height() / 2.0;
	mouse_pos.1 /= master.zoom;
	mouse_pos.1 += master.camera.y;

	vec2(mouse_pos.0, mouse_pos.1)
}

pub fn get_input_axis(left: KeyCode, right: KeyCode) -> f32 {
	let mut input = 0.0;
	if is_key_down(left) {
		input -= 1.0;
	}
	if is_key_down(right) {
		input += 1.0;
	}
	input
}

pub fn get_movement_input(up: KeyCode, down: KeyCode, left: KeyCode, right: KeyCode) -> Vec2 {
	let vec = vec2(get_input_axis(left, right), get_input_axis(up, down));
	vec.normalize_or_zero()
}

pub fn string_to_keycode(s: String) -> KeyCode {
	match s.as_str() {
		"space" => KeyCode::Space,
		"apostrophe" => KeyCode::Apostrophe,
		"comma" => KeyCode::Comma,
		"minus" => KeyCode::Minus,
		"period" => KeyCode::Period,
		"slash" => KeyCode::Slash,
		"0" => KeyCode::Key0,
		"1" => KeyCode::Key1,
		"2" => KeyCode::Key2,
		"3" => KeyCode::Key3,
		"4" => KeyCode::Key4,
		"5" => KeyCode::Key5,
		"6" => KeyCode::Key6,
		"7" => KeyCode::Key7,
		"8" => KeyCode::Key8,
		"9" => KeyCode::Key9,
		"semicolon" => KeyCode::Semicolon,
		"equal" => KeyCode::Equal,
		"a" => KeyCode::A,
		"b" => KeyCode::B,
		"c" => KeyCode::C,
		"d" => KeyCode::D,
		"e" => KeyCode::E,
		"f" => KeyCode::F,
		"g" => KeyCode::G,
		"h" => KeyCode::H,
		"i" => KeyCode::I,
		"j" => KeyCode::J,
		"k" => KeyCode::K,
		"l" => KeyCode::L,
		"m" => KeyCode::M,
		"n" => KeyCode::N,
		"o" => KeyCode::O,
		"p" => KeyCode::P,
		"q" => KeyCode::Q,
		"r" => KeyCode::R,
		"s" => KeyCode::S,
		"t" => KeyCode::T,
		"u" => KeyCode::U,
		"v" => KeyCode::V,
		"w" => KeyCode::W,
		"x" => KeyCode::X,
		"y" => KeyCode::Y,
		"z" => KeyCode::Z,
		"left_bracket" => KeyCode::LeftBracket,
		"backslash" => KeyCode::Backslash,
		"right_bracket" => KeyCode::RightBracket,
		"grave_accent" => KeyCode::GraveAccent,
		"escape" => KeyCode::Escape,
		"enter" => KeyCode::Enter,
		"tab" => KeyCode::Tab,
		"backspace" => KeyCode::Backspace,
		"insert" => KeyCode::Insert,
		"delete" => KeyCode::Delete,
		"right" => KeyCode::Right,
		"left" => KeyCode::Left,
		"down" => KeyCode::Down,
		"up" => KeyCode::Up,
		"page_up" => KeyCode::PageUp,
		"page_down" => KeyCode::PageDown,
		"home" => KeyCode::Home,
		"end" => KeyCode::End,
		"caps_lock" => KeyCode::CapsLock,
		"scroll_lock" => KeyCode::ScrollLock,
		"num_lock" => KeyCode::NumLock,
		"print_screen" => KeyCode::PrintScreen,
		"pause" => KeyCode::Pause,
		"f1" => KeyCode::F1,
		"f2" => KeyCode::F2,
		"f3" => KeyCode::F3,
		"f4" => KeyCode::F4,
		"f5" => KeyCode::F5,
		"f6" => KeyCode::F6,
		"f7" => KeyCode::F7,
		"f8" => KeyCode::F8,
		"f9" => KeyCode::F9,
		"f10" => KeyCode::F10,
		"f11" => KeyCode::F11,
		"f12" => KeyCode::F12,
		"f13" => KeyCode::F13,
		"f14" => KeyCode::F14,
		"f15" => KeyCode::F15,
		"f16" => KeyCode::F16,
		"f17" => KeyCode::F17,
		"f18" => KeyCode::F18,
		"f19" => KeyCode::F19,
		"f20" => KeyCode::F20,
		"f21" => KeyCode::F21,
		"f22" => KeyCode::F22,
		"f23" => KeyCode::F23,
		"f24" => KeyCode::F24,
		"f25" => KeyCode::F25,
		"keypad_0" => KeyCode::Kp0,
		"keypad_1" => KeyCode::Kp1,
		"keypad_2" => KeyCode::Kp2,
		"keypad_3" => KeyCode::Kp3,
		"keypad_4" => KeyCode::Kp4,
		"keypad_5" => KeyCode::Kp5,
		"keypad_6" => KeyCode::Kp6,
		"keypad_7" => KeyCode::Kp7,
		"keypad_8" => KeyCode::Kp8,
		"keypad_9" => KeyCode::Kp9,
		"keypad_decimal" => KeyCode::KpDecimal,
		"keypad_divide" => KeyCode::KpDivide,
		"keypad_multiply" => KeyCode::KpMultiply,
		"keypad_subtract" => KeyCode::KpSubtract,
		"keypad_add" => KeyCode::KpAdd,
		"keypad_enter" => KeyCode::KpEnter,
		"keypad_equal" => KeyCode::KpEqual,
		"left_shift" => KeyCode::LeftShift,
		"left_ctrl" => KeyCode::LeftControl,
		"left_alt" => KeyCode::LeftAlt,
		"left_super" => KeyCode::LeftSuper,
		"right_shift" => KeyCode::RightShift,
		"right_ctrl" => KeyCode::RightControl,
		"right_alt" => KeyCode::RightAlt,
		"right_super" => KeyCode::RightSuper,
		"menu" => KeyCode::Menu,
		_ => KeyCode::Unknown,
	}
}

pub fn keycode_to_string(key: KeyCode) -> String {
	match key {
		KeyCode::Space => "space".to_string(),
		KeyCode::Apostrophe => "apostrophe".to_string(),
		KeyCode::Comma => "comma".to_string(),
		KeyCode::Minus => "minus".to_string(),
		KeyCode::Period => "period".to_string(),
		KeyCode::Slash => "slash".to_string(),
		KeyCode::Key0 => "0".to_string(),
		KeyCode::Key1 => "1".to_string(),
		KeyCode::Key2 => "2".to_string(),
		KeyCode::Key3 => "3".to_string(),
		KeyCode::Key4 => "4".to_string(),
		KeyCode::Key5 => "5".to_string(),
		KeyCode::Key6 => "6".to_string(),
		KeyCode::Key7 => "7".to_string(),
		KeyCode::Key8 => "8".to_string(),
		KeyCode::Key9 => "9".to_string(),
		KeyCode::Semicolon => "semicolon".to_string(),
		KeyCode::Equal => "equal".to_string(),
		KeyCode::A => "a".to_string(),
		KeyCode::B => "b".to_string(),
		KeyCode::C => "c".to_string(),
		KeyCode::D => "d".to_string(),
		KeyCode::E => "e".to_string(),
		KeyCode::F => "f".to_string(),
		KeyCode::G => "g".to_string(),
		KeyCode::H => "h".to_string(),
		KeyCode::I => "i".to_string(),
		KeyCode::J => "j".to_string(),
		KeyCode::K => "k".to_string(),
		KeyCode::L => "l".to_string(),
		KeyCode::M => "m".to_string(),
		KeyCode::N => "n".to_string(),
		KeyCode::O => "o".to_string(),
		KeyCode::P => "p".to_string(),
		KeyCode::Q => "q".to_string(),
		KeyCode::R => "r".to_string(),
		KeyCode::S => "s".to_string(),
		KeyCode::T => "t".to_string(),
		KeyCode::U => "u".to_string(),
		KeyCode::V => "v".to_string(),
		KeyCode::W => "w".to_string(),
		KeyCode::X => "x".to_string(),
		KeyCode::Y => "y".to_string(),
		KeyCode::Z => "z".to_string(),
		KeyCode::LeftBracket => "left_bracket".to_string(),
		KeyCode::Backslash => "backslash".to_string(),
		KeyCode::RightBracket => "right_bracket".to_string(),
		KeyCode::GraveAccent => "grave_accent".to_string(),
		KeyCode::Escape => "escape".to_string(),
		KeyCode::Enter => "enter".to_string(),
		KeyCode::Tab => "tab".to_string(),
		KeyCode::Backspace => "backspace".to_string(),
		KeyCode::Insert => "insert".to_string(),
		KeyCode::Delete => "delete".to_string(),
		KeyCode::Right => "right".to_string(),
		KeyCode::Left => "left".to_string(),
		KeyCode::Down => "down".to_string(),
		KeyCode::Up => "up".to_string(),
		KeyCode::PageUp => "page_up".to_string(),
		KeyCode::PageDown => "page_down".to_string(),
		KeyCode::Home => "home".to_string(),
		KeyCode::End => "end".to_string(),
		KeyCode::CapsLock => "caps_lock".to_string(),
		KeyCode::ScrollLock => "scroll_lock".to_string(),
		KeyCode::NumLock => "num_lock".to_string(),
		KeyCode::PrintScreen => "print_screen".to_string(),
		KeyCode::Pause => "pause".to_string(),
		KeyCode::F1 => "f1".to_string(),
		KeyCode::F2 => "f2".to_string(),
		KeyCode::F3 => "f3".to_string(),
		KeyCode::F4 => "f4".to_string(),
		KeyCode::F5 => "f5".to_string(),
		KeyCode::F6 => "f6".to_string(),
		KeyCode::F7 => "f7".to_string(),
		KeyCode::F8 => "f8".to_string(),
		KeyCode::F9 => "f9".to_string(),
		KeyCode::F10 => "f10".to_string(),
		KeyCode::F11 => "f11".to_string(),
		KeyCode::F12 => "f12".to_string(),
		KeyCode::F13 => "f13".to_string(),
		KeyCode::F14 => "f14".to_string(),
		KeyCode::F15 => "f15".to_string(),
		KeyCode::F16 => "f16".to_string(),
		KeyCode::F17 => "f17".to_string(),
		KeyCode::F18 => "f18".to_string(),
		KeyCode::F19 => "f19".to_string(),
		KeyCode::F20 => "f20".to_string(),
		KeyCode::F21 => "f21".to_string(),
		KeyCode::F22 => "f22".to_string(),
		KeyCode::F23 => "f23".to_string(),
		KeyCode::F24 => "f24".to_string(),
		KeyCode::F25 => "f25".to_string(),
		KeyCode::Kp0 => "keypad_0".to_string(),
		KeyCode::Kp1 => "keypad_1".to_string(),
		KeyCode::Kp2 => "keypad_2".to_string(),
		KeyCode::Kp3 => "keypad_3".to_string(),
		KeyCode::Kp4 => "keypad_4".to_string(),
		KeyCode::Kp5 => "keypad_5".to_string(),
		KeyCode::Kp6 => "keypad_6".to_string(),
		KeyCode::Kp7 => "keypad_7".to_string(),
		KeyCode::Kp8 => "keypad_8".to_string(),
		KeyCode::Kp9 => "keypad_9".to_string(),
		KeyCode::KpDecimal => "keypad_decimal".to_string(),
		KeyCode::KpDivide => "keypad_divide".to_string(),
		KeyCode::KpMultiply => "keypad_multiply".to_string(),
		KeyCode::KpSubtract => "keypad_subtract".to_string(),
		KeyCode::KpAdd => "keypad_add".to_string(),
		KeyCode::KpEnter => "keypad_enter".to_string(),
		KeyCode::KpEqual => "keypad_equal".to_string(),
		KeyCode::LeftShift => "left_shift".to_string(),
		KeyCode::LeftControl => "left_ctrl".to_string(),
		KeyCode::LeftAlt => "left_alt".to_string(),
		KeyCode::LeftSuper => "left_super".to_string(),
		KeyCode::RightShift => "right_shift".to_string(),
		KeyCode::RightControl => "right_ctrl".to_string(),
		KeyCode::RightAlt => "right_alt".to_string(),
		KeyCode::RightSuper => "right_super".to_string(),
		KeyCode::Menu => "menu".to_string(),
		_ => "".to_string(),
	}
}