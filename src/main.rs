use macroquad::prelude::*;

// Map
struct Map {
    width: f32,
    height: f32,
}

struct Object {
    x: f32,
    y: f32,
}

// Clash of clans
#[macroquad::main("Clash of clans")]

// Main
async fn main() {
    // Map struct
    let map = Map {
        width: 300.0,
        height: 300.0,
    };

    // Object struct
    let house = Object {
        // Coords relative to map
        x: 100.,
        y: 100.,
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
            camera_origin = (mouse_position().0 - offset.0, mouse_position().1 - offset.1);
        }

        // Draw map
        draw_rectangle(camera_origin.0 - map.width / 2., camera_origin.1 - map.height / 2., map.width, map.height, WHITE);
        
        // Make a loop for this-------------
        draw_rectangle(camera_origin.0 - map.width / 2. + 0., camera_origin.1 - map.height / 2., 50., 50., GREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 50., camera_origin.1 - map.height / 2., 50., 50., DARKGREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 100., camera_origin.1 - map.height / 2., 50., 50., GREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 150., camera_origin.1 - map.height / 2., 50., 50., DARKGREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 200., camera_origin.1 - map.height / 2., 50., 50., GREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 250., camera_origin.1 - map.height / 2., 50., 50., DARKGREEN);

        draw_rectangle(camera_origin.0 - map.width / 2. + 0., camera_origin.1 - map.height / 2. + 50., 50., 50., DARKGREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 50., camera_origin.1 - map.height / 2. + 50., 50., 50., GREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 100., camera_origin.1 - map.height / 2. + 50., 50., 50., DARKGREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 150., camera_origin.1 - map.height / 2. + 50., 50., 50., GREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 200., camera_origin.1 - map.height / 2. + 50., 50., 50., DARKGREEN);
        draw_rectangle(camera_origin.0 - map.width / 2. + 250., camera_origin.1 - map.height / 2. + 50., 50., 50., GREEN);

        // Draw house on map (black square)
        draw_rectangle(camera_origin.0 - map.width / 2. + house.x, camera_origin.1 - map.height / 2. + house.y, 10., 10., BLACK);

        // Next_frame
        next_frame().await;
    }
}