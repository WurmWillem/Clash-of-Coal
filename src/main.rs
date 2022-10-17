use macroquad::prelude::*;

pub const SCREENSIZE: (f32, f32) = (720., 720.); //Change this if you want to, old was 640, 480
pub const CAMERA_MOVE_MULT: f32 = 2.;

struct Map {
    tex: Texture2D,
}
impl Map {
    pub fn new(tex: Texture2D) -> Self{
        Self { tex, }
    }

    pub fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        };

        draw_texture_ex(self.tex, 0., 0., WHITE, params.clone());
    }
}

struct Camera {
    origin: (f32, f32),
    offset: (f32, f32),
}
impl Camera {
    fn new() -> Self {
        Self { origin: (0., 0.), offset: (0., 0.) }
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
            self.origin.1 -= 0.01;   
        };
        if is_key_down(KeyCode::W) {
            self.origin.1 += 0.01;   
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
        }

        set_camera(&Camera2D {
            target: vec2(self.origin.0  * -CAMERA_MOVE_MULT, self.origin.1 * CAMERA_MOVE_MULT),
            ..Default::default()
        });
    }
}

#[macroquad::main("Clash of clans")]
async fn main() {
    request_new_screen_size(SCREENSIZE.0, SCREENSIZE.1); //Set new screensize

    //Initialize map and camera
    let map_tex = load_texture("map.png").await.unwrap();
    let map = Map::new(map_tex);
    let mut camera = Camera::new();
    
    loop {
        clear_background(BLACK);
        
        camera.update();
        map.draw();

        next_frame().await;
    }
}
