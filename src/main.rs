use macroquad::prelude::*;

mod camera;
use camera::Camera;
mod resources;
use resources::Resources;
mod building;
use building::{Building, BuildingKind};

pub const SCREENSIZE: (f32, f32) = (720., 720.);

#[macroquad::main("Clash of Clans")]
async fn main() {
    request_new_screen_size(SCREENSIZE.0, SCREENSIZE.1); //Set new screensize

    //Initialize map and camera
    let map_tex = load_texture("map.png")
        .await
        .expect("failed to load map.png");

    let mut universe = Universe::new(map_tex).await;

    loop {
        clear_background(LIGHTGRAY);

        universe.draw();
        universe.update();

        next_frame().await;
    }
}

struct Universe {
    cam: Camera,
    resources: Resources,
    buildings: Vec<Vec<Building>>,
    building_textures: Vec<Texture2D>,
    map_tex: Texture2D,
}
impl Universe {
    const MAP_COORDS: Vec2 = Vec2::new(-0.5, -0.5);

    pub async fn new(map_tex: Texture2D) -> Self {
        let cam = Camera::new();

        let mut row = Vec::new();
        for _ in 0..10 {
            row.push(Building::new(BuildingKind::None))
        }
        let mut buildings = Vec::new();
        for _ in 0..10 {
            buildings.push(row.clone());
        }
        buildings[0][0] = Building::new(BuildingKind::Mine);

        let pickaxe = load_texture("pickaxe.png").await
        .expect("failed to load pickaxe.png");
        let building_textures = vec![pickaxe];
        
        let resources = Resources::new(0).await;
        Self {
            cam,
            resources,
            buildings,
            map_tex,
            building_textures,
        }
    }

    pub fn update(&mut self) {
        self.cam.update();
        if is_mouse_button_pressed(MouseButton::Left) {
            let target_x = (self.cam.origin.0 * -20.).round();
            let target_y = (self.cam.origin.1 * -20.).round();

            let orig_pos_x = ((mouse_position_local().x + 0.5) * 10.).floor();
            let orig_pos_y = ((mouse_position_local().y + 0.5) * 10.).floor();

            let grid_pos = Vec2::new(target_x + orig_pos_x, target_y + orig_pos_y);
            let grid_pos = check_if_safe(grid_pos);
            
            if let Some(pos) = grid_pos {
                self.buildings[pos.y as usize][pos.x as usize].update(&mut self.resources);
            }
        }
    }

    pub fn draw(&self) {
        self.cam.set_as_cam();

        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        };

        draw_texture_ex(
            self.map_tex,
            Universe::MAP_COORDS.x,
            Universe::MAP_COORDS.y,
            WHITE,
            params.clone(),
        );

        let mut y = 9.;
        for column in &self.buildings {
            let mut x = 0.;
            for building in column {
                building.draw((x, y), self.building_textures.clone());
                x += 1.;
            }
            y -= 1.;
        }

        self.resources.draw();
    }
}

fn check_if_safe(var: Vec2) -> Option<Vec2> {
    let x = if var.x < 0. && var.x > -1. {
        Some(0.)
    } else if var.x > 9. && var.x < 10. {
        Some(9.)
    } else if var.x < 10. && var.x > -1.{
        Some(var.x)
    } else {
        None
    };

    let y = if var.y < 0. && var.y > -1. {
        Some(0.)
    } else if var.y > 9. && var.y < 10. {
        Some(9.)
    } else if var.y < 10. && var.y > -1.{
        Some(var.y)
    } else {
        None
    };
    let mut vec: Vec2 = Vec2::new(0., 0.);

    vec.x = match x {
        Some(x) => x,
        None => return None,
    };
    vec.y = match y {
        Some(y) => y,
        None => return None,
    };
    
    Some(vec)
}
