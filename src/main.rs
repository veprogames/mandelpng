use std::{error::Error, io};
use mandelpng::render::Scene;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut scene = match io::stdin().read_line(&mut input) {
        Ok(_) => match serde_json::from_str(&input) {
            Ok(scene) => scene,
            Err(_) => Scene::default()
        },
        Err(_) => Scene::default()
    };

    let image = scene.generate_image();

    image.write_to_stdout()?;

    Ok(())
}
