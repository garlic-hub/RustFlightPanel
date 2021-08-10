mod instruments;

use anyhow::Result;
use macroquad::prelude::*;

use crate::instruments::airspeed::Airspeed;
use crate::instruments::instrument::Instrument;
use crate::instruments::instrument::InstrumentData;
use serde_json::json;

const GRID_SIZE_WIDTH: f32 = 300.;
const GRID_SIZE_HEIGHT: f32 = 300.;

#[macroquad::main("Flight Panel")]
async fn main() -> Result<()> {
    let a = Airspeed::create("config/airspeed.yaml").await?;
    let mut rot = 0.;
    let mut map = InstrumentData::new();
    map.insert("rot".to_string(), json!(rot));

    let mut v = Vec::<Box<dyn Instrument>>::new();
    v.push(Box::new(a));

    loop {
        clear_background(WHITE);

        for i in v.iter_mut() {
            i.update(&map);
            i.render(GRID_SIZE_WIDTH, GRID_SIZE_HEIGHT);
        }
        rot += 0.03;
        map.insert("rot".to_string(), json!(rot));
        println!("{}", get_fps());

        next_frame().await
    }
}
