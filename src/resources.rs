use macroquad::prelude::*;

pub struct Resources {
    pub gold: i32,
    tex: Vec<Texture2D>,
}
impl Resources {
    pub async fn new(gold: i32) -> Self {
        let gold_tex = load_texture("assets/coin.png")
            .await
            .expect("failed to load assets/coin.png");
        let gold_bar = load_texture("assets/gold bar.png")
            .await
            .expect("failed to load assets/gold bar.png");

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
}