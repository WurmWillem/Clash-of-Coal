use macroquad::prelude::*;
//use strum_macros::EnumIter;
use crate::SCREENSIZE;

pub const CAMERA_MOVE_MULT: f32 = 2.;

pub struct Camera {
    pub origin: (f32, f32),
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

    pub fn set_as_cam(&self) {
        set_camera(&Camera2D {
            target: vec2(
                self.origin.0 * -CAMERA_MOVE_MULT,
                self.origin.1 * CAMERA_MOVE_MULT,
            ),
            ..Default::default()
        });
    }
}
