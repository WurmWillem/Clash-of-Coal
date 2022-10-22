use macroquad::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    building::{Building, BuildingKind},
    player::Player,
    resources::Resources,
};

//Variables for drawing and hitbox sizes for buildings in shop
const X_OFFSET: f32 = 80.;
const X_INCREASE: f32 = 70.;
const Y: f32 = 590.;
const BUILDING_SIZE: f32 = 50.;

pub struct Shop {
    buildings: Vec<Building>,
    textures: Vec<Texture2D>,
}
impl Shop {
    pub async fn new() -> Self {
        let mut buildings = Vec::new();
        for kind in BuildingKind::iter() {
            if kind != BuildingKind::None {
                buildings.push(Building::new(kind));
            }
        }

        let bar_tex = load_texture("assets/shop_bar.png")
            .await
            .expect("failed to load assets/shop_bar.png");

        let textures = vec![bar_tex];

        Self {
            buildings,
            textures,
        }
    }

    pub fn get_input(&mut self, resources: &mut Resources, player: &mut Player) {
        set_default_camera(); //Needed because we want the ui to be static, so we want to draw in screen space instead of our camera space

        //Checks if a building in the shop is pressed and acts accordingly
        //For example by making the players held building the building in the shop and subtracting the price to the players gold
        let mut new_price = None;
        let mut x = 0.;
        for building in &self.buildings {
            let pos_x = x * X_INCREASE + X_OFFSET;

            //Check if building in shop is pressed
            if mouse_position().0 >= pos_x
                && mouse_position().0 <= pos_x + BUILDING_SIZE
                && mouse_position().1 >= Y
                && mouse_position().0 <= Y + BUILDING_SIZE
                && is_mouse_button_pressed(MouseButton::Left)
            {
                if resources.gold >= building.price {
                    player.held_building = building.kind;
                    resources.gold -= building.price;
                    new_price = Some(building.price / 5);
                    break;
                }
            }
            x += 1.;
        }
        if let Some(price) = new_price {
            self.buildings[x as usize].price += price;
        }
    }

    pub fn draw(&self, building_textures: &Vec<Texture2D>) {
        let shop_params = DrawTextureParams {
            dest_size: Some(macroquad::prelude::Vec2::new(600., 120.)),
            ..Default::default()
        };
        draw_texture_ex(self.textures[0], 60., 570., WHITE, shop_params);

        let building_params = DrawTextureParams {
            dest_size: Some(macroquad::prelude::Vec2::new(BUILDING_SIZE, BUILDING_SIZE)),
            ..Default::default()
        };

        //The actual drawing occurs here
        let mut x = 0.;
        for building in &self.buildings {
            if building.kind == BuildingKind::None {
                return;
            }
            draw_texture_ex(
                building_textures[building.texture_index],
                x * X_INCREASE + X_OFFSET,
                Y,
                WHITE,
                building_params.clone(),
            );
            draw_text(&building.price.to_string(), x * 70. + 80., 675., 30., BLACK);

            x += 1.;
        }
    }
}
