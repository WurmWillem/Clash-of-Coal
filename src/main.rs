use macroquad::prelude::*;

mod camera;
use camera::Camera;

pub const SCREENSIZE: (f32, f32) = (720., 720.); //Change this if you want to, old was 640, 480

struct Resources {
    gold: i32,
    tex: Vec<Texture2D>,
}
impl Resources {
    pub async fn new(gold: i32) -> Self {
        let gold_tex = load_texture("coin.png")
            .await
            .expect("Couldnt load coin png");
        let gold_bar = load_texture("gold bar.png")
            .await
            .expect("Couldnt load gold bar png");

        let tex = vec![gold_tex, gold_bar];
        Self { gold, tex }
    }

    pub fn draw(&self) {
        set_default_camera();

        let resource_param = DrawTextureParams {
            dest_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        };
        let bar_param = DrawTextureParams {
            dest_size: Some(Vec2::new(250., 50.)),
            ..Default::default()
        };

        draw_texture_ex(self.tex[0], 100., 10., WHITE, resource_param.clone());

        draw_texture_ex(self.tex[1], 160., 10., WHITE, bar_param.clone());
        draw_text(&self.gold.to_string(), 170., 51., 40., BLACK)
    }
}

#[macroquad::main("Clash of Clans")]
async fn main() {
    request_new_screen_size(SCREENSIZE.0, SCREENSIZE.1); //Set new screensize

    //Initialize map and camera
    let map_tex = load_texture("map.png")
        .await
        .expect("failed to load map.png");
    let map = Map::new(map_tex);

    let resources = Resources::new(0).await;

    let mut cam = Camera::new();

    loop {
        clear_background(LIGHTGRAY);

        cam.update(); //make sure to update the camera before drawing in camera space
        map.draw(&cam);
        resources.draw();

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

    pub fn draw(&self, cam: &Camera) {
        cam.set_as_cam();

        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        };

        draw_texture_ex(self.tex, -0.5, -0.5, WHITE, params);
    }
}
