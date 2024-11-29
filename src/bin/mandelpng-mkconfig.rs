use std::error::Error;

use mandelpng::render::Scene;

extern crate mandelpng;

fn main() -> Result<(), Box<dyn Error>>{
    let json = serde_json::to_string_pretty(&Scene::default())?;
    print!("{}", json);
    Ok(())
}