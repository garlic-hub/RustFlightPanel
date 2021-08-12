use anyhow::{Context, Result};
use macroquad::texture::{load_texture, Texture2D};
use serde::Deserialize;
use serde_json::Value;
use serde_yaml::from_str;

use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

use crate::instruments::airspeed::Airspeed;
use crate::instruments::attitude_indicator::AttitudeIndicator;

pub type InstrumentData = HashMap<String, Value>;

enum InstrumentWrapper {
    Disabled(String),
    Enabled(Box<dyn Instrument>),
}

#[derive(Deserialize, Debug)]
enum InstrumentType {
    Airspeed,
    AttitudeIndicator,
}

#[derive(Deserialize, Debug)]
pub struct InstrumentConfig {
    enabled: bool,
    instrument: InstrumentType,
    pub grid_x: f32,
    pub grid_y: f32,
    textures_paths: Vec<String>,
}

impl InstrumentConfig {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_string = path.as_ref().display().to_string();
        let s = read_to_string(path).with_context(|| format!("Cannot load {}", path_string))?;
        Ok(from_str(&s).with_context(|| format!("Cannot parse {}", path_string))?)
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

async fn create_instrument(ic: InstrumentConfig) -> Result<Box<dyn Instrument>> {
    match ic.instrument {
        InstrumentType::Airspeed => Ok(Box::new(Airspeed::new(ic).await?)),
        InstrumentType::AttitudeIndicator => Ok(Box::new(AttitudeIndicator::new(ic).await?)),
    }
}

async fn get_instrument<P: AsRef<Path>>(path: P) -> Result<InstrumentWrapper> {
    let ic = InstrumentConfig::from_file(path)?;

    if !ic.enabled {
        return Ok(InstrumentWrapper::Disabled(format!(
            "Instrument {:?} disabled. Skipping",
            ic.instrument
        )));
    }

    Ok(InstrumentWrapper::Enabled(create_instrument(ic).await?))
}

pub async fn get_instruments() -> Result<Vec<Box<dyn Instrument>>> {
    let mut v = Vec::new();

    for p in read_dir("config")? {
        match get_instrument(p?.path()).await? {
            InstrumentWrapper::Disabled(s) => println!("{}", s),
            InstrumentWrapper::Enabled(i) => v.push(i),
        }
    }

    Ok(v)
}
