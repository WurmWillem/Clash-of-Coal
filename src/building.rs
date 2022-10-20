use macroquad::prelude::*;

use crate::resources::Resources;

#[derive(Debug, Clone)]
pub struct Building {
    pub kind: BuildingKind,
    pub price: i32,
    pub texture_index: usize,
}
impl Building {
    pub fn new(kind: BuildingKind) -> Self {
        let price = kind.get_price();
        let texture_index = kind.get_texture_index();
        Self {
            kind,
            price,
            texture_index,
        }
    }

    pub fn update(&self, resources: &mut Resources) {
        //Updates the resources based on the kind of building
        match &self.kind {
            BuildingKind::Mine => resources.gold += 1,
            BuildingKind::None => (),
        }
    }

    pub fn draw(&self, pos: (f32, f32), textures: &Vec<Texture2D>) {
        let params = DrawTextureParams {
            flip_y: true,
            dest_size: Some(macroquad::prelude::Vec2::new(0.09, 0.09)),
            ..Default::default()
        };

        match self.kind {
            BuildingKind::Mine => {
                draw_texture_ex(
                    textures[0],
                    pos.0 * 0.1 - 0.495,
                    pos.1 * 0.1 - 0.495,
                    WHITE,
                    params,
                );
            }
            BuildingKind::None => (),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BuildingKind {
    None,
    Mine,
}
impl BuildingKind {
    pub fn get_price(&self) -> i32 {
        match self {
            Self::Mine => 10,
            _ => 0,
        }
    }

    pub fn get_texture_index(&self) -> usize {
        match self {
            Self::Mine => 0,
            Self::None => 99,
        }
    }
}
