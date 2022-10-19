use macroquad::{texture::{Texture2D, DrawTextureParams, draw_texture_ex}, prelude::WHITE};

use crate::resources::Resources;


#[derive(Debug, Clone)]
pub struct Building {
    kind: BuildingKind,
}
impl Building {
    pub fn new(kind: BuildingKind) -> Self {
        Self { kind }
    }

    pub fn update(&self, resources: &mut Resources) {
        match &self.kind {
            BuildingKind::Mine => resources.gold += 1,
            BuildingKind::None => (),
        }
    }

    pub fn draw(&self, pos: (f32, f32), textures: Vec<Texture2D>) {
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
                    params.clone(),
                );
            },
            BuildingKind::None => (),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuildingKind {
    None,
    Mine,
}
