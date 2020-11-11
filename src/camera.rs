use macroquad::{is_key_down, mouse_position, screen_height, screen_width, vec2, KeyCode, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    target: Vec2,
    zoom: Vec2,
}

impl Camera {
    pub const fn get(&self) -> (Vec2, Vec2) {
        (self.target, self.zoom)
    }

    pub fn set_target(&mut self, new_target: Vec2) {
        self.target = Vec2::new(new_target.x() - 0.5, new_target.y() + 0.5);
    }

    /// Get and handle the input related to the camera.
    pub fn scroll(&mut self, scroll_speed: f32, zoom_speed: f32) {
        /*
        // Move the camera:
        // UP
        if is_key_down(KeyCode::Comma) {
            self.target
                .set_y(self.target.y() + scroll_speed / self.zoom.x())
        }
        // DOWN
        if is_key_down(KeyCode::O) {
            self.target
                .set_y(self.target.y() - scroll_speed / self.zoom.x())
        }
        // LEFT
        if is_key_down(KeyCode::A) {
            self.target
                .set_x(self.target.x() - scroll_speed / self.zoom.x())
        }
        // RIGHT
        if is_key_down(KeyCode::E) {
            self.target
                .set_x(self.target.x() + scroll_speed / self.zoom.x())
        }
        */

        // Change the camera zoom:
        // Further
        if is_key_down(KeyCode::Apostrophe) {
            self.zoom.set_x(self.zoom.x() * zoom_speed);
            self.zoom.set_y(self.zoom.y() * zoom_speed);
        }
        // Closer
        if is_key_down(KeyCode::Period) {
            self.zoom.set_x(self.zoom.x() / zoom_speed);
            self.zoom.set_y(self.zoom.y() / zoom_speed);
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let starting_zoom = 0.10;
        Self {
            target: vec2(0.0, 0.0),
            zoom: vec2(
                starting_zoom,
                starting_zoom * screen_width() / screen_height(),
            ),
        }
    }
}

/// Get the mouse coordinates inside the game world.
pub fn mouse_position_relative_to(camera: &Camera) -> Vec2 {
    // Takes the mouse coordinates on window and translates that
    // to game world coordinates.
    let mouse = mouse_position();
    let center = Vec2::new(
        ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / camera.zoom.x())
            + camera.target.x(),
        ((-mouse.1 + screen_height() / 2.0)
            / (screen_height() / 2.0)
            / camera.zoom.x()
            / (screen_width() / screen_height()))
            + camera.target.y(),
    );
    Vec2::new(center.x(), center.y())
}
