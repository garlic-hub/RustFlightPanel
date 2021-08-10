use anyhow::{Context, Result};
use macroquad::texture::{load_texture, Texture2D};
use serde::Deserialize;
use serde_json::Value;
use serde_yaml::from_str;

use std::collections::HashMap;
use std::fs::read_to_string;

pub type InstrumentData = HashMap<String, Value>;

#[derive(Deserialize, Debug)]
enum InstrumentType {
    Airspeed,
}

#[derive(Deserialize, Debug)]
pub struct InstrumentConfig {
    instrument: InstrumentType,
    pub grid_x: f32,
    pub grid_y: f32,
    textures_paths: Vec<String>,
}

impl InstrumentConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let s = read_to_string(path).with_context(|| format!("Cannot load {}", path))?;
        Ok(from_str(&s).with_context(|| format!("Cannot parse {}", path))?)
    }

    pub async fn load_textures(&self) -> Result<Vec<Texture2D>> {
        let mut v = Vec::with_capacity(self.textures_paths.len());
        for p in self.textures_paths.iter() {
            v.push(load_texture(p).await?);
        }
        Ok(v)
    }
}

pub trait Instrument {
    fn render(&self, grid_width: f32, grid_height: f32);
    fn update(&mut self, map: &InstrumentData);
}
