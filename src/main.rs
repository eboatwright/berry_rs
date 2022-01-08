#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod built_in_components;
mod built_in_systems;
mod master;
mod resources;
mod util;

mod empty_scene;

use crate::master::Master;
use macroquad::prelude::*;
use hecs::World;

const SCREEN_WIDTH: i32 = 960 / 1;
const SCREEN_HEIGHT: i32 = 600 / 1;

fn window_conf() -> Conf {
    Conf {
        window_title: "empty project".to_string(),
        window_width: 960,
        window_height: 600,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut master = Master::new();
    master.resources.load().await;
    let mut world = World::new();

    master.load_empty_scene(&mut world);

    loop {
        master.update(&mut world);

        let game_render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        set_camera(&Camera2D {
            zoom: vec2(1.0 / SCREEN_WIDTH as f32 * 2.0, 1.0 / SCREEN_HEIGHT as f32 * 2.0),
            target: master.camera_pos,
            render_target: Some(game_render_target),
            ..Default::default()
        });
        clear_background(DARKGRAY);

        master.render(&mut world);

        set_default_camera();

        let game_diff_w = screen_width() / SCREEN_WIDTH as f32;
        let game_diff_h = screen_height() / SCREEN_HEIGHT as f32;
        let aspect_diff = game_diff_w.min(game_diff_h);
        master.zoom = aspect_diff;

        let scaled_game_size_w = SCREEN_WIDTH as f32 * aspect_diff;
        let scaled_game_size_h = SCREEN_HEIGHT as f32 * aspect_diff;

        let width_padding = (screen_width() - scaled_game_size_w) * 0.5;
        let height_padding = (screen_height() - scaled_game_size_h) * 0.5;

        clear_background(BLACK);

        game_render_target.texture.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            game_render_target.texture,
            width_padding,
            height_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_game_size_w, scaled_game_size_h)),
                ..Default::default()
            },
        );

        next_frame().await
    }
}