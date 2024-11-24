use std::{fs::File, io::BufWriter, path::Path, error::Error};

const W: usize = 640;
const H: usize = 360;

fn get_image_data() -> Vec<u8> {
    let mut data = [0u8; W * H * 3];

    for y in 0..H {
        for x in 0..W {
            let idx = (x + y * W) * 3;
            data[idx] = 255;
            data[idx + 1] = 255;
            data[idx + 2] = 255;
        }
    }

    data.to_vec()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("result.png");
    let file = File::create(path)?;
    let bw = BufWriter::new(file);

    let mut encoder = png::Encoder::new(bw, W as u32, H as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let data = &get_image_data();

    writer.write_image_data(data)?;

    println!("fin");
    
    Ok(())
}
