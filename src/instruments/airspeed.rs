use anyhow::Result;
use macroquad::prelude::*;

use super::instrument::*;

pub struct Airspeed {
    config: InstrumentConfig,
    textures: Vec<Texture2D>,
    rotation: f32,
}

impl Airspeed {
    pub async fn create(config_path: &str) -> Result<Self> {
        let ic = InstrumentConfig::from_file(config_path)?;
        let textures_vec = ic.load_textures().await?;
        Ok(Airspeed {
            config: ic,
            textures: textures_vec,
            rotation: 0.,
        })
    }
}

impl Instrument for Airspeed {
    fn render(&self, grid_width: f32, grid_height: f32) {
        draw_texture(
            self.textures[0],
            grid_width * self.config.grid_x,
            grid_height * self.config.grid_y,
            WHITE,
        );

        let airspeed_needle_params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: self.rotation,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            self.textures[1],
            grid_width * self.config.grid_x,
            grid_height * self.config.grid_y,
            WHITE,
            airspeed_needle_params,
        );
    }

    fn update(&mut self, map: &InstrumentData) {
        self.rotation = map.get("rot").unwrap().as_f64().unwrap() as f32;
    }
}
