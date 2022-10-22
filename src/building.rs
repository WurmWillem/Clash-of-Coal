use macroquad::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
            BuildingKind::Mine2 => resources.gold += 4,
            BuildingKind::Mine3 => resources.gold += 10,
            BuildingKind::None => (),
        }
    }

    pub fn draw(&self, pos: (f32, f32), textures: &Vec<Texture2D>) {
        if self.kind == BuildingKind::None {
            return;
        }

        let params = DrawTextureParams {
            flip_y: true,
            dest_size: Some(macroquad::prelude::Vec2::new(0.09, 0.09)),
            ..Default::default()
        };

        draw_texture_ex(
            textures[self.texture_index],
            pos.0 * 0.1 - 0.495,
            pos.1 * 0.1 - 0.495,
            WHITE,
            params,
        );
    }
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum BuildingKind {
    None,
    Mine,
    Mine2,
    Mine3,
}
impl BuildingKind {
    pub fn get_price(&self) -> i32 {
        match self {
            Self::Mine => 10,
            Self::Mine2 => 50,
            Self::Mine3 => 150,
            _ => 0,
        }
    }

    pub fn get_texture_index(&self) -> usize {
        let mut i = 0;
        for kind in BuildingKind::iter() {
            if kind == BuildingKind::None {
                continue;
            }
            if kind == *self {
                return i;
            }
            i += 1;
        }
        99
    }
}
