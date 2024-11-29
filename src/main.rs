use std::{error::Error, io::{self, BufWriter}};

use render::Scene;

pub mod render;
mod fractal;
mod palette;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut scene = match io::stdin().read_line(&mut input) {
        Ok(_) => match serde_json::from_str(&input) {
            Ok(scene) => scene,
            Err(_) => Scene::default()
        },
        Err(_) => Scene::default()
    };

    let bw = BufWriter::new(std::io::stdout());

    let mut encoder = png::Encoder::new(
        bw,
        scene.viewport().image_width() as u32,
        scene.viewport().image_height() as u32,
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let data = scene.get_image_data();

    writer.write_image_data(data)?;

    Ok(())
}
