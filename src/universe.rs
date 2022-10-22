use json::JsonValue;
use macroquad::prelude::*;

use crate::{
    building::{Building, BuildingKind},
    camera::Camera,
    player::Player,
    resources::Resources,
    shop::Shop,
};
use std::fs;

pub struct Universe {
    cam: Camera,
    player: Player,
    resources: Resources,
    shop: Shop,
    buildings: Vec<Vec<Building>>,
    building_textures: Vec<Texture2D>,
    map_tex: Texture2D,
    data: JsonValue,
}
impl Universe {
    const MAP_COORDS: Vec2 = Vec2::new(-0.5, -0.5);

    pub async fn new() -> Self {
        let cam = Camera::new();

        let player = Player::new();

        let mut row = Vec::new();
        for _ in 0..10 {
            row.push(Building::new(BuildingKind::None))
        }
        let mut buildings = Vec::new();
        for _ in 0..10 {
            buildings.push(row.clone());
        }

        let mine = load_texture("assets/pickaxe.png")
            .await
            .expect("failed to load pickaxe.png");
        let mine2 = load_texture("assets/pickaxe2.png")
            .await
            .expect("failed to load pickaxe2.png");
        let mine3 = load_texture("assets/pickaxe3.png")
            .await
            .expect("failed to load pickaxe3.png");
        let building_textures = vec![mine, mine2, mine3];

        let data = fs::read_to_string("resources.json").expect("failed to read resources.json");
        let data = json::parse(&format!(r#"{}"#, data)).expect("failed to parse resources.json");
        
        let resources = Resources::new(
            data["gold"].as_i32().unwrap()
        )
        .await;

        let map_tex = load_texture("assets/map.png")
            .await
            .expect("failed to load assets/map.png");

        let shop = Shop::new().await;

        Self {
            cam,
            player,
            resources,
            shop,
            buildings,
            map_tex,
            building_textures,
            data,
        }
    }

    pub fn get_input(&mut self) {
        //This checks if and what place on the grid is clicked and updates that place accordingly
        //For example by adding the players held building to that place
        if is_mouse_button_pressed(MouseButton::Left) {
            let target_x = (self.cam.origin.0 * -20.).round();
            let target_y = (self.cam.origin.1 * -20.).round();

            let orig_pos_x = ((mouse_position_local().x + 0.5) * 10.).floor();
            let orig_pos_y = ((mouse_position_local().y + 0.5) * 10.).floor();

            let grid_pos = Vec2::new(target_x + orig_pos_x, target_y + orig_pos_y);
            let grid_pos = check_if_safe(grid_pos);

            if let Some(pos) = grid_pos {
                if self.player.held_building != BuildingKind::None
                    && self.buildings[pos.y as usize][pos.x as usize].kind == BuildingKind::None
                {
                    self.buildings[pos.y as usize][pos.x as usize] =
                        Building::new(self.player.held_building);
                    self.player.held_building = BuildingKind::None;
                }
            }
        }

        if is_key_pressed(KeyCode::LeftControl) && is_key_down(KeyCode::S)
            || is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::S)
        {
            println!("save");
            self.save();
        }

        self.shop.get_input(&mut self.resources, &mut self.player);
    }

    pub fn update(&mut self) {
        self.cam.update(); //Updates the camera position, which affects the place of the map on the screen but not ui

        //This checks if a second has passed, because only then do we want to update
        let second_passed = (get_time() * 100.).floor() / 100. % 1. == 0.;
        if !second_passed {
            return;
        }

        //We update the resources for every building, for example by adding gold to the resources
        let mut gold_per_sec = 0;
        for column in &self.buildings {
            for building in column {
                self.resources.gold += building.gold_per_sec;
                gold_per_sec += building.gold_per_sec;
            }
        }
        self.resources.gold_per_sec = gold_per_sec;
    }

    pub fn draw(&self) {
        self.cam.set_as_cam(); //Make sure we're drawing in camera space because the map isn't UI

        //These params are used for specifying extra details on how to draw the textures, like size
        let map_params = DrawTextureParams {
            dest_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        };
        draw_texture_ex(
            self.map_tex,
            Universe::MAP_COORDS.x,
            Universe::MAP_COORDS.y,
            WHITE,
            map_params,
        );

        //Draw every building in the grid
        let mut y = 9.;
        for column in &self.buildings {
            let mut x = 0.;
            for building in column {
                building.draw((x, y), &self.building_textures);
                x += 1.;
            }
            y -= 1.;
        }

        set_default_camera(); //We want to draw UI, so we need to work with screen coordinates

        self.resources.draw();
        self.shop.draw(&self.building_textures);
        self.player.draw_held_item(&self.building_textures);
    }

    fn save(&mut self) {
        self.data["gold"] = self.resources.gold.into();
        fs::write("resources.json", format!(r#"{}"#, self.data.to_string())).unwrap();
    }
}

fn check_if_safe(var: Vec2) -> Option<Vec2> {
    let x = if var.x < 0. && var.x > -1. {
        Some(0.)
    } else if var.x > 9. && var.x < 10. {
        Some(9.)
    } else if var.x < 10. && var.x > -1. {
        Some(var.x)
    } else {
        None
    };

    let y = if var.y < 0. && var.y > -1. {
        Some(0.)
    } else if var.y > 9. && var.y < 10. {
        Some(9.)
    } else if var.y < 10. && var.y > -1. {
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
