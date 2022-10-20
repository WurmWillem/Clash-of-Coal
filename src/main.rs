use macroquad::prelude::*;

mod building;
mod camera;
mod player;
mod resources;
mod shop;
mod universe;
use universe::Universe;

pub const SCREENSIZE: (f32, f32) = (720., 720.);

#[macroquad::main("Clash of Clans")]
async fn main() {
    request_new_screen_size(SCREENSIZE.0, SCREENSIZE.1); //Set new screensize

    let mut universe = Universe::new().await;

    loop {
        clear_background(LIGHTGRAY);

        universe.get_input();
        universe.draw();
        universe.update();

        next_frame().await;
    }
}
