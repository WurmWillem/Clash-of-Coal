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

#[macroquad::main("Clash of clans")]
async fn main() {
    request_new_screen_size(SCREENSIZE.0, SCREENSIZE.1); //Set new screensize

    let map_tex = load_texture("map.png").await.unwrap();
    let map = Map::new(map_tex);

    // Vars
    let mut camera_origin = (0., 0.);
    let mut offset = (0., 0.);
    
    // Game loop
    loop {
        // Clear screen
        clear_background(BLACK);
        
        if is_key_down(KeyCode::A) {
            camera_origin.0 -= 0.01;   //update where the camera looks at
        };
        if is_key_down(KeyCode::D) {
            camera_origin.0 += 0.01;   //update where the camera looks at
        };
        if is_key_down(KeyCode::S) {
            camera_origin.1 -= 0.01;   //update where the camera looks at
        };
        if is_key_down(KeyCode::W) {
            camera_origin.1 += 0.01;   //update where the camera looks at
        };

        // Update map position
        if is_mouse_button_pressed(MouseButton::Left) {
            offset = (
                mouse_position().0 / SCREENSIZE.0 - camera_origin.0,
                mouse_position().1 / SCREENSIZE.1 - camera_origin.1,
            );
        }
        if is_mouse_button_down(MouseButton::Left) {
            camera_origin.0 = mouse_position().0 / SCREENSIZE.0 - offset.0; 
            camera_origin.1 = mouse_position().1 / SCREENSIZE.1 - offset.1; 
        }

        set_camera(&Camera2D {
            target: vec2(camera_origin.0  * -CAMERA_MOVE_MULT, camera_origin.1 * CAMERA_MOVE_MULT),
            ..Default::default()
        });
        
        map.draw();

        // Next_frame
        next_frame().await;
    }
}
