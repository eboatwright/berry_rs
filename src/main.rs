#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod built_in_components;
mod built_in_systems;
mod master;
mod resources;

mod game_scene;

use crate::master::Master;
use macroquad::prelude::*;
use crate::built_in_components::*;

const SCREEN_WIDTH: i32 = 960 / 1;
const SCREEN_HEIGHT: i32 = 600 / 1;

fn window_conf() -> Conf {
    Conf {
        window_title: "empty project".to_string(),
        window_width: 960,
        window_height: 600,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut master = Master::default();
    master.resources.load().await;

    let game_render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    game_render_target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D {
        zoom: vec2(1.0 / SCREEN_WIDTH as f32 * 2.0, 1.0 / SCREEN_HEIGHT as f32 * 2.0),
        render_target: Some(game_render_target),
        ..Default::default()
    };

    master.load_game_scene();

    loop {
        master.update();

        for (_entity, transform) in &mut master.world.query::<&Transform>().with::<RenderCamera>() {
            camera.target = transform.position;
            break;
        }
        set_camera(&camera);

        clear_background(DARKGRAY);

        master.render();

        set_default_camera();

        let game_diff = vec2(
            screen_width() / SCREEN_WIDTH as f32,
            screen_height() / SCREEN_HEIGHT as f32
        );
        let aspect_diff = game_diff.x.min(game_diff.y);
        for (_entity, camera) in &mut master.world.query::<&mut RenderCamera>() {
            camera.zoom = aspect_diff;
            break;
        }

        let scaled_game_size = vec2(
            SCREEN_WIDTH as f32 * aspect_diff,
            SCREEN_HEIGHT as f32 * aspect_diff,
        );

        let padding = vec2(
            (screen_width() - scaled_game_size.x) * 0.5,
            (screen_height() - scaled_game_size.y) * 0.5,
        );

        clear_background(BLACK);

        draw_texture_ex(
            game_render_target.texture,
            padding.x,
            padding.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(scaled_game_size),
                ..Default::default()
            },
        );

        next_frame().await
    }
}