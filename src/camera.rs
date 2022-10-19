use macroquad::prelude::*;

use crate::{SCREENSIZE};

const CAMERA_MOVE_MULT: f32 = 2.;

pub struct Camera {
    origin: (f32, f32),
    offset: (f32, f32),
}
impl Camera {
    pub fn new() -> Self {
        Self {
            origin: (0., 0.),
            offset: (0., 0.),
        }
    }
    pub fn update(&mut self) {
        //update camera_origin based on key presses
        if is_key_down(KeyCode::A) {
            self.origin.0 -= 0.01;
        };
        if is_key_down(KeyCode::D) {
            self.origin.0 += 0.01;
        };
        if is_key_down(KeyCode::S) {
            self.origin.1 += 0.01;
        };
        if is_key_down(KeyCode::W) {
            self.origin.1 -= 0.01;
        };

        // Update camera position
        if is_mouse_button_pressed(MouseButton::Left) {
            self.offset = (
                mouse_position().0 / SCREENSIZE.0 - self.origin.0,
                mouse_position().1 / SCREENSIZE.1 - self.origin.1,
            );
        }
        if is_mouse_button_down(MouseButton::Left) {
            self.origin.0 = mouse_position().0 / SCREENSIZE.0 - self.offset.0;
            self.origin.1 = mouse_position().1 / SCREENSIZE.1 - self.offset.1;

            //println!("{:?}", self.origin);
        }

        //this function actually updates the camera based on the origin
        set_camera(&Camera2D {
            target: vec2(
                self.origin.0 * -CAMERA_MOVE_MULT,
                self.origin.1 * CAMERA_MOVE_MULT,
            ),
            ..Default::default()
        });
    }
}