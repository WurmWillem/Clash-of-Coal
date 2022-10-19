use macroquad::prelude::*;
use std::fs;

pub struct Resources {
     gold: i32,
     tex: Vec<Texture2D>,
}

impl Resources {
     pub async fn new(gold: i32) -> Self {
          let gold_tex = load_texture("coin.png")
             .await
             .expect("Couldnt load coin png");
          let gold_bar = load_texture("gold bar.png")
             .await
             .expect("Couldnt load gold bar png");
 
          let tex = vec![gold_tex, gold_bar];

          Self { gold, tex }
     }
 
     pub fn draw(&self) {
         set_default_camera();
 
         let resource_param = DrawTextureParams {
             dest_size: Some(Vec2::new(50., 50.)),
             ..Default::default()
         };
         let bar_param = DrawTextureParams {
             dest_size: Some(Vec2::new(250., 50.)),
             ..Default::default()
         };
 
         draw_texture_ex(self.tex[0], 100., 10., WHITE, resource_param.clone());
 
         draw_texture_ex(self.tex[1], 160., 10., WHITE, bar_param.clone());
         draw_text(&self.gold.to_string(), 170., 51., 40., BLACK)
     }

     pub fn load_gold(&self) {          
          let contents = fs::read_to_string("resources.txt")
              .expect("Should have been able to read the file");
          
          self.gold = contents.parse::<i32>().unwrap();
     }
}