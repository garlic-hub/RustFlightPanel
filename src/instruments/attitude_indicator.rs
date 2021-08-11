use anyhow::Result;
use macroquad::prelude::*;

use super::instrument::*;

pub struct AttitudeIndicator {
    config: InstrumentConfig,
    textures: Vec<Texture2D>,
    roll_rotation: f32,
}

impl AttitudeIndicator {
    pub async fn create(config_path: &str) -> Result<Self> {
        let ic = InstrumentConfig::from_file(config_path)?;
        let textures_vec = ic.load_textures().await?;
        Ok(AttitudeIndicator {
            config: ic,
            textures: textures_vec,
            roll_rotation: 0.,
        })
    }
}

impl Instrument for AttitudeIndicator {
    fn render(&self, grid_width: f32, grid_height: f32) {
        // Draw the roll
        let roll_params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: self.roll_rotation,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(
            self.textures[0],
            grid_width * self.config.grid_x,
            grid_height * self.config.grid_y,
            WHITE,
            roll_params,
        );

        // Draw the pitch
        draw_texture(
            self.textures[1],
            grid_width * self.config.grid_x,
            grid_height * self.config.grid_y,
            WHITE,
        );

        // Draw the pointer
        draw_texture(
            self.textures[2],
            grid_width * self.config.grid_x,
            grid_height * self.config.grid_y,
            WHITE,
        );
    }

    fn update(&mut self, map: &InstrumentData) {
        self.roll_rotation = map.get("rot").unwrap().as_f64().unwrap() as f32;
    }
}
