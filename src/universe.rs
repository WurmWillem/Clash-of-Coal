use macroquad::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Resource {
    pub kind: i8,
    pub amount: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Camera {
     pub origin: (f32, f32),
     offset: (f32, f32),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
     pub held_building: String,
 }

#[derive(Deserialize, Serialize, Debug)]
pub struct Building {
    pub kind: String,
    pub price: i32,
    pub texture_index: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Resources {
    resources: Vec<Resource>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Universe {
    cam: Camera,
    player: Player,
    resources: Resources,
    buildings: Vec<Building>,
}

impl Universe {
     pub fn new() -> self {
          
     }
}