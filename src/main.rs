use macroquad::prelude::*;

mod camera;
use camera::Camera;

mod file_IO;
use file_IO::*;

pub const SCREENSIZE: (f32, f32) = (720., 720.); //Change this if you want to, old was 640, 480

#[macroquad::main("Clash of Clans")]
async fn main() {
    request_new_screen_size(SCREENSIZE.0, SCREENSIZE.1); //Set new screensize

    //Initialize map and camera
    let map_tex = load_texture("map.png").await.unwrap();
    let map = Map::new(map_tex);
    let mut camera = Camera::new();

    load_resources();

    loop {
        clear_background(BLACK);

        camera.update();
        map.draw();

        next_frame().await;
    }
}

struct Map {
    tex: Texture2D,
}
impl Map {
    pub fn new(tex: Texture2D) -> Self {
        Self { tex }
    }

    pub fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        };

        draw_texture_ex(self.tex, 0., 0., WHITE, params);
    }
}