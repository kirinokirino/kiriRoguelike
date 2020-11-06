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
#![allow(clippy::missing_docs_in_private_items)]

use macroquad::{
    clear_background, debug, draw_circle, draw_text, is_key_pressed, is_mouse_button_down,
    load_texture, next_frame, set_camera, set_default_camera, warn, Camera2D, Color, KeyCode,
    MouseButton, Vec2, BLACK, GRAY, WHITE,
};

mod camera;
use camera::{mouse_position_relative_to, Camera};
#[macroquad::main("kiriRoguelike")]
async fn main() {
    // Load assets.
    let texture = load_texture("assets/Tiles.png").await;

    // We need to save the state of the mouse button
    // to detect mouse clicks and not just "is pressed"
    let mut left_mouse_pressed = false;

    // Create main camera.
    let mut main_camera = Camera::default();

    // The infinite game loop.
    loop {
        // ===========Input===========
        // Get the mouse position inside the game world.
        let mouse_position = mouse_position_relative_to(&main_camera);
        left_mouse_pressed = handle_mouse(left_mouse_pressed, mouse_position);
        handle_keyboard(&mut main_camera);

        // ===========Update===========
        // Checks for input related to camera and changes it accordingly.

        // ===========Draw===========
        // Fill the canvas with white.
        clear_background(BLACK);

        // --- Camera space, render game objects.

        let (target, zoom) = main_camera.get();
        set_camera(Camera2D {
            target,
            zoom,
            ..macroquad::Camera2D::default()
        });

        // Draw the mouse cursor.
        draw_circle(
            mouse_position.x(),
            mouse_position.y(),
            0.1,
            Color([100, 75, 120, 255]),
        );

        // --- Fixed screen space, render ui.
        set_default_camera();
        draw_ui();

        next_frame().await
    }
}

/// Render the fixed screen ui. (after `set_default_camera()`)
fn draw_ui() {
    let text_color: Color = Color([100, 100, 100, 150]);
    draw_text(",aoe to move camera", 10.0, 0.0, 20.0, text_color);
    draw_text("'. to zoom camera", 10.0, 30.0, 20.0, text_color);
    draw_text(
        "arrow keys to move the player",
        10.0,
        60.0,
        20.0,
        text_color,
    );
}

/// Handle the input from the keyboard.
fn handle_keyboard(camera: &mut Camera) {
    camera.scroll(0.03, 0.97);
    if is_key_pressed(KeyCode::Right) {}
    if is_key_pressed(KeyCode::Left) {}
    if is_key_pressed(KeyCode::Down) {}
    if is_key_pressed(KeyCode::Up) {}
}

/// Handle the mouse. Print the click position.
fn handle_mouse(left_mouse_pressed: bool, mouse_position: Vec2) -> bool {
    if is_mouse_button_down(MouseButton::Left) {
        if !left_mouse_pressed {
            debug!(
                "Mouse click at relative x:{} , y:{}",
                mouse_position.x() as i32,
                mouse_position.y() as i32
            );
        }
        true
    } else {
        false
    }
}
