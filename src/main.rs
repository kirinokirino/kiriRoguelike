#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::clone_on_ref_ptr,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::let_underscore_must_use,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unwrap_used,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::string_add,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::future_not_send
)]

use macroquad::{
    clear_background, debug, draw_circle, draw_text, is_key_pressed, is_mouse_button_down,
    next_frame, set_camera, set_default_camera, warn, Camera2D, Color, KeyCode, MouseButton, Vec2,
};

mod graphics;
use graphics::layer::LAYER_DIMENSIONS;
use graphics::tile_atlas::TileAtlas;

mod world;
use world::{Generator, World};

mod entities;
use entities::entities::Entities;

mod camera;
use camera::{mouse_position_relative_to, Camera};
#[macroquad::main("kiriRoguelike")]
async fn main() {
    // Load assets.
    let tile_atlas = TileAtlas::default().await;

    // We need to save the state of the mouse button
    // to detect mouse clicks and not just "is pressed"
    let mut left_mouse_pressed = false;

    // Create main camera.
    let mut main_camera = Camera::default();

    // Create the world!
    let mut world = World::default();
    let mut generator = Generator::default();
    let mut entities = Entities::default();
    let mut score: i64 = 0;
    // The infinite game loop.
    loop {
        // ===========Input===========
        // Get the mouse position inside the game world.
        let mouse_position = mouse_position_relative_to(&main_camera);
        left_mouse_pressed = handle_mouse(left_mouse_pressed, mouse_position);

        entities.input(handle_keyboard(&mut main_camera));
        let player_pos = entities.player.entity.world_pos.clone();
        // ===========Update===========
        // Checks for input related to camera and changes it accordingly.

        // Update the world!
        world.update(&player_pos, &generator);
        entities.update(&world, &generator);
        main_camera.set_target(entities.player.entity.get_absolute_position().into());
        score = entities.player.score;
        // ===========Draw===========
        // Fill the canvas with white.
        clear_background(Color {
            0: [40, 40, 40, 255],
        });

        // --- Camera space, render game objects.
        let (target, zoom) = main_camera.get();
        set_camera(Camera2D {
            target,
            zoom,
            ..macroquad::Camera2D::default()
        });

        // Draw the world!
        let player = &entities.player;
        world.draw(&tile_atlas, &player);
        entities.draw(&tile_atlas);

        // Draw the mouse cursor.
        draw_circle(
            mouse_position.x(),
            mouse_position.y(),
            0.1,
            Color([100, 75, 120, 255]),
        );

        // --- Fixed screen space, render ui.
        set_default_camera();
        draw_text(
            format!("Collect coins! you have {}.", score).as_str(),
            5.,
            5.,
            30.,
            Color::new(40., 80., 170., 200.),
        );

        next_frame().await
    }
}

/// Handle the input from the keyboard.
fn handle_keyboard(camera: &mut Camera) -> (i8, i8) {
    camera.scroll(0.03, 0.97);
    let mut res = (0, 0);
    if is_key_pressed(KeyCode::Right) {
        res.0 = 1;
    }
    if is_key_pressed(KeyCode::Left) {
        res.0 = -1;
    }
    if is_key_pressed(KeyCode::Down) {
        res.1 = -1;
    }
    if is_key_pressed(KeyCode::Up) {
        res.1 = 1;
    }

    res
}

/// Handle the mouse. Print the coordinates where the mouse was clicked.
fn handle_mouse(left_mouse_pressed: bool, mouse_position: Vec2) -> bool {
    if is_mouse_button_down(MouseButton::Left) {
        if !left_mouse_pressed {
            debug!(
                "Mouse click x:{:.0}|{:.1} , y:{:.0}|{:.1}",
                (mouse_position.x() / f32::from(LAYER_DIMENSIONS)).floor(),
                mouse_position.x(),
                (mouse_position.y() / f32::from(LAYER_DIMENSIONS)).floor(),
                mouse_position.y(),
            );
        }

        true
    } else {
        false
    }
}
