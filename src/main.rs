use std::{error::Error, io::BufWriter};

use num_complex::{Complex32, ComplexFloat};

const W: usize = 1920;
const H: usize = 1080;

const MAX_ITER: i32 = 256;

fn get_iterations(c: Complex32) -> i32 {
    let mut z = c.clone();
    
    for iteration in 1..MAX_ITER {
        z = z.powi(2) + c;

        if z.re.powf(2.0) + z.im.powf(2.0) > 4.0 {
            return iteration;
        }
    }

    MAX_ITER
}

fn remap(v: f32, i0: f32, i1: f32, o0: f32, o1: f32) -> f32 {
    let fact = (o1 - o0) / (i1 - i0);
    (v - i0) * fact + o0
}

fn get_color(iterations: i32) -> u8 {
    if iterations >= MAX_ITER {
        0
    } else {
        (4 * iterations % 256) as u8
    }
}

fn get_image_data() -> Vec<u8> {
    let mut data = [0u8; W * H * 3];

    for y in 0..H {
        for x in 0..W {
            let idx = (x + y * W) * 3;
            let cx: f32 = remap(x as f32, 0.0, W as f32, -3.0, 2.0);
            let cy: f32 = remap(y as f32, 0.0, H as f32, -1.5, 1.5);
            let iterations = 2 * get_iterations(Complex32::new(cx, cy));
            let color = get_color(iterations);
            data[idx] = color;
            data[idx + 1] = color;
            data[idx + 2] = color;
        }
    }

    data.to_vec()
}

fn main() -> Result<(), Box<dyn Error>> {
    let bw = BufWriter::new(std::io::stdout());

    let mut encoder = png::Encoder::new(bw, W as u32, H as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    let data = &get_image_data();

    writer.write_image_data(data)?;
    
    Ok(())
}
