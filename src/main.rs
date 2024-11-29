use std::{error::Error, io::{stdin, Read}};
use mandelpng::render::Scene;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    stdin().read_to_string(&mut input)?;
    let mut scene: Scene = serde_json::from_str(&input)?;

    let image = scene.generate_image();

    image.write_to_stdout()?;

    Ok(())
}
