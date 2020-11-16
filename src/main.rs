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
use graphics::tile_atlas::TileAtlas;

mod tile_types;

mod coords;
use coords::{get_checked_position, AbsolutePosition, ChunkPosition, LocalPosition, CHUNK_SIZE};

mod world;
use world::World;

mod generator;
use generator::Generator;

mod entities;
use entities::entities::Entities;

mod camera;
use camera::{mouse_position_relative_to, Camera};

#[macroquad::main("kiriRoguelike")]
async fn main() {
    // Load tile atlas from the default file.
    let tile_atlas = TileAtlas::default().await;

    // Create the world, place that holds terrain.
    let mut world = World::default();
    // Create and seed the generator.
    let generator = Generator::new(0);
    // Create the container for all of the entities.
    let mut entities = Entities::default();
    // Just a number to show the score.
    let mut score: i64;

    // Create main camera.
    let mut main_camera = Camera::default();

    // We need to save the state of the mouse button
    // to detect mouse clicks, as opposed to just "is pressed" state.
    let mut left_mouse_pressed = false;

    // The position of the tile we clicked.
    let mut cursor;

    // The infinite game loop.
    loop {
        // ===========Input===========
        // Get the Vec2 position of the mouse inside the game world.
        let mouse_position = mouse_position_relative_to(&main_camera);

        // Select the clicked on tile and update the left mouse button state.
        let (updated_left_mouse_pressed, updated_cursor) =
            handle_mouse(left_mouse_pressed, mouse_position);
        left_mouse_pressed = updated_left_mouse_pressed;
        cursor = updated_cursor;

        // Print the info about the tile we clicked.
        if let Some(cursor) = cursor {
            println!("Terrain: {}", world.get_tile(&cursor).unwrap());
            if let Some(entity) = entities.get_mut_entity_at_pos(&cursor) {
                println!("{}", entity);
                entity.removed = true;
            }
        }

        // Entities container handles player movement.
        entities.input(handle_keyboard(&mut main_camera));
        // ===========Update===========

        // We need to know the player's chunk to see what chunks need to be
        // loaded or unloaded.
        let player_pos = entities.player.entity.chunk_pos.clone();
        // Load or generate the chunks near the player.
        world.update(&player_pos, &generator);

        // Load, generate or update all of the entities.
        entities.update(&world, &generator);
        // We need to get the new score to show it on screen.
        score = entities.player.score;

        // Point the camera to the new position of the player before the drawing stage.
        main_camera.set_target(entities.player.entity.get_absolute_position_f32().into());

        // ===========Draw===========
        // Fill the canvas with the background color.
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

        // We draw everything besides the ui in camera space.
        // World needs to know the players location to know what terrain is visible
        // and how far it is to make it less visible.
        let player = &entities.player;
        world.draw(&tile_atlas, &player);
        // Entities container already knows about the player.
        entities.draw(&tile_atlas);

        // Draw the mouse cursor. As a small circle.
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
/// Returns the direction for the player to set the destination to.
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
/// Return the absolute position to be able to see what was clicked.
fn handle_mouse(
    left_mouse_pressed: bool,
    mouse_position: Vec2,
) -> (bool, Option<AbsolutePosition>) {
    if is_mouse_button_down(MouseButton::Left) {
        let (mut mouse_x, mut mouse_y) = (mouse_position.x(), mouse_position.y());
        mouse_x = mouse_x.floor();
        mouse_y = mouse_y.floor();
        let chunk_dimensions = f32::from(CHUNK_SIZE);
        let (world_x, world_y) = (
            (mouse_x / chunk_dimensions).floor(),
            (mouse_y / chunk_dimensions).floor(),
        );

        if mouse_x < 0.0 {
            mouse_x = chunk_dimensions - (-mouse_x % chunk_dimensions);
        }
        if mouse_y < 0.0 {
            mouse_y = chunk_dimensions - (-mouse_y % chunk_dimensions);
        }

        if !left_mouse_pressed {
            let (x, y) = (
                mouse_x.abs() % chunk_dimensions,
                mouse_y.abs() % chunk_dimensions,
            );
            debug!(
                "World {world_x:.0}:{world_y:.0} | Position {x:.0}:{y:.0}",
                world_x = world_x,
                x = x,
                world_y = world_y,
                y = y
            );
            let AbsolutePosition {
                chunk: chunk_pos,
                local: local_pos,
            } = get_checked_position(
                ChunkPosition {
                    x: world_x as i32,
                    y: world_y as i32,
                },
                LocalPosition {
                    x: (x + 1.) as i16,
                    y: y as i16,
                },
            );
            return (
                true,
                Some(AbsolutePosition {
                    chunk: chunk_pos,
                    local: local_pos,
                }),
            );
        }
        return (true, None);
    }
    (false, None)
}
