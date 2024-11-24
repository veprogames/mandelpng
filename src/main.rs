use std::{fs::File, io::BufWriter, path::Path};

fn main() {
    let path = Path::new("result.png");
    let file = File::create(path)
        .expect("File created");
    let bw = BufWriter::new(file);

    let mut encoder = png::Encoder::new(bw, 1, 1);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()
        .expect("Add PNG header to Buffer");

    let data = &[255, 255, 255];

    writer.write_image_data(data)
        .expect("Write Image data to buffer");

    println!("fin");
}
