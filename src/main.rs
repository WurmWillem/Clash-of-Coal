use macroquad::prelude::*;

// Map?
struct Map {
    width: f32,
    height: f32,
    curr_center: (f32, f32),
}

// Clash of clans
#[macroquad::main("Clash of clans")]

// Main
async fn main() {
    // Map struct
    let map = Map {
        width: 300.0,
        height: 300.0,
        curr_center: (screen_width() / 2.0 - 150.0, screen_height() / 2.0 - 150.0),
    };

    // Vars
    let mut mouse_pos: (f32, f32) = (-1., -1.);
    let mut offset_mouse: (f32, f32) = (0., 0.);

    // Game loop
    loop {
        // Clear screen
        clear_background(BLACK);

        // Update map position
        if is_mouse_button_down(MouseButton::Left) {
            mouse_pos = mouse_position();
        }
        if is_mouse_button_released(MouseButton::Left) {
            mouse_pos = (-1., -1.);
        }

        offset_mouse = (map.curr_center.0 - mouse_position().0, map.curr_center.1 - mouse_position().1);

        if mouse_pos != (-1., -1.) {
            let x: f32 = mouse_position().0 - offset_mouse.0;
            let y: f32 = mouse_position().1 - offset_mouse.1;
            map.curr_center = (x, y);
            print!("X: {}, Y: {}\n", map.curr_center.0, map.curr_center.1);
        }

        // Draw map
        draw_rectangle(map.curr_center.0 - map.width / 2., map.curr_center.1 - map.height / 2., map.width, map.height, GREEN);

        // Next_frame
        next_frame().await
    }
}