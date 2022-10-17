use macroquad::prelude::*;

// Map
struct Map {
    width: f32,
    height: f32,
}

// Clash of clans
#[macroquad::main("Clash of clans")]

// Main
async fn main() {
    // Map struct
    let mut map = Map {
        width: 300.0,
        height: 300.0,
    };

    // Vars
    let mut camera_origin = (400., 300.);
    let mut offset = (0., 0.);

    // Game loop
    loop {
        // Clear screen
        clear_background(BLACK);

        // Update map position
        if is_mouse_button_pressed(MouseButton::Left) {
            offset = (mouse_position().0 - camera_origin.0, mouse_position().1 - camera_origin.1);
        }
        if is_mouse_button_down(MouseButton::Left) {
            camera_origin = (mouse_position().0 - offset.0, mouse_position().1 - offset.1)
        }

        // Draw map
        draw_rectangle(camera_origin.0 - map.width / 2., camera_origin.1 - map.height / 2., map.width, map.height, GREEN);

        // Next_frame
        next_frame().await
    }
}