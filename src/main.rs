use std::{fs::File, io::BufWriter, path::Path, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("result.png");
    let file = File::create(path)?;
    let bw = BufWriter::new(file);

    let mut encoder = png::Encoder::new(bw, 1, 1);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let data = &[255, 255, 255];

    writer.write_image_data(data)?;

    println!("fin");
    
    Ok(())
}
