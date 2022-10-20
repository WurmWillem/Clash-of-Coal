use crate::building::BuildingKind;
use macroquad::prelude::*;

pub struct Player {
    pub held_building: BuildingKind,
}
impl Player {
    pub fn new() -> Self {
        Self {
            held_building: BuildingKind::None,
        }
    }

    pub fn draw_held_item(&self, textures: &Vec<Texture2D>) {
        if self.held_building == BuildingKind::None {
            return;
        }

        let building_draw_size = 60.;
        let params = DrawTextureParams {
            dest_size: Some(macroquad::prelude::Vec2::new(
                building_draw_size,
                building_draw_size,
            )),
            ..Default::default()
        };
        let index = self.held_building.get_texture_index();

        draw_texture_ex(
            textures[index],
            mouse_position().0 - building_draw_size / 2.,
            mouse_position().1 - building_draw_size / 2.,
            WHITE,
            params,
        )
    }
}
