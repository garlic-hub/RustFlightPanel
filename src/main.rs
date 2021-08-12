mod instruments;

use anyhow::Result;
use macroquad::prelude::*;
use serde_json::json;

use crate::instruments::instrument::get_instruments;
use crate::instruments::instrument::InstrumentData;

const GRID_SIZE_WIDTH: f32 = 300.;
const GRID_SIZE_HEIGHT: f32 = 300.;

#[macroquad::main("Flight Panel")]
async fn main() -> Result<()> {
    let mut rot = 0.;
    let mut map = InstrumentData::new();
    map.insert("rot".to_string(), json!(rot));

    let mut v = get_instruments().await?;

    loop {
        clear_background(WHITE);

        for i in v.iter_mut() {
            i.update(&map);
            i.render(GRID_SIZE_WIDTH, GRID_SIZE_HEIGHT);
        }
        rot += 0.03;
        map.insert("rot".to_string(), json!(rot));

        next_frame().await
    }
}
