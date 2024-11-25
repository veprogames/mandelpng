use std::{error::Error, io::BufWriter};

use render::Viewport;

mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let viewport = Viewport::default();

    let bw = BufWriter::new(std::io::stdout());

    let mut encoder = png::Encoder::new(
        bw,
        viewport.image_width() as u32,
        viewport.image_height() as u32,
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let data = &viewport.get_image_data();

    writer.write_image_data(data)?;

    Ok(())
}
