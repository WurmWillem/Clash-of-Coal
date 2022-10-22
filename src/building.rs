use macroquad::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub struct Building {
    pub kind: BuildingKind,
    pub price: i32,
    pub gold_per_sec: i32,
    pub texture_index: usize,
}
impl Building {
    pub fn new(kind: BuildingKind) -> Self {
        let price = kind.get_price();
        let gold_per_sec = kind.get_gold_per_sec();
        let texture_index = kind.get_texture_index();
        Self {
            kind,
            price,
            gold_per_sec,
            texture_index,
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

    pub fn get_gold_per_sec(&self) -> i32 {
        match self {
            Self::Mine => 1,
            Self::Mine2 => 4,
            Self::Mine3 => 10,
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
