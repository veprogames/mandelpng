use std::{error::Error, io::{self, BufWriter}};

use num_complex::Complex32;
use serde::{Deserialize, Serialize};

use crate::{fractal::Mandelbrot, palette::Color, utils::{average_color, remap}};

#[derive(Serialize, Deserialize)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            data: vec![],
        }
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0u8; width * height * Image::channel_count()],
        }
    }

    fn channel_count() -> usize {
        3
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) * Image::channel_count()
    }

    fn has_pixel(&self, x: usize, y: usize) -> bool{
        let idx = self.get_index(x, y);
        let last_channel = idx + Image::channel_count() - 1;
        return self.data.get(idx).is_some() &&
            self.data.get(last_channel).is_some()
    }

    /// Note: Fails silently if indexing fails :(
    fn try_set_pixel(&mut self, x: usize, y: usize, rgb: Color) {
        let idx = self.get_index(x, y);

        if self.has_pixel(x, y) {
            self.data[idx] = rgb.r();
            self.data[idx + 1] = rgb.g();
            self.data[idx + 2] = rgb.b();
        }
    }

    pub fn write_to_stdout(&self) -> Result<(), Box<dyn Error>>{
        let bw = BufWriter::new(io::stdout());

        let mut encoder = png::Encoder::new(
            bw,
            self.width as u32,
            self.height as u32,
        );
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
    
        let mut writer = encoder.write_header()?;
    
        let data = &self.data;
    
        writer.write_image_data(data)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Viewport {
    image_width: usize,
    image_height: usize,
    super_sampling: u32,
    cx: f32,
    cy: f32,
    zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            image_width: 1920,
            image_height: 1080,
            super_sampling: 2,
            cx: 0.0,
            cy: 0.0,
            zoom: 2.0,
        }
    }
}

impl Viewport {
    pub fn image_width(&self) -> usize {
        self.image_width
    }

    pub fn image_height(&self) -> usize {
        self.image_width
    }

    fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let (width_f32, height_f32) = (self.image_width as f32, self.image_height as f32);
        let aspect_ratio = width_f32 / height_f32;

        (
            remap(
                screen_x,
                0.0,
                width_f32,
                self.cx - self.zoom * aspect_ratio,
                self.cx + self.zoom * aspect_ratio,
            ),
            remap(
                screen_y,
                0.0,
                height_f32,
                self.cy - self.zoom,
                self.cy + self.zoom,
            ),
        )
    }

    fn calculate_pixel(&self, x: usize, y: usize, super_sampling: u32, fractal: &Mandelbrot) -> Color {
        let mut colors: Vec<Color> = vec![];
        for sx in 0..super_sampling {
            for sy in 0..super_sampling {
                let screen_x = x as f32 + sx as f32 / super_sampling as f32;
                let screen_y = y as f32 + sy as f32 / super_sampling as f32;
                let (cx, cy) = self.screen_to_world(screen_x, screen_y);
                let iterations = 2 * fractal.get_iterations(Complex32::new(cx, cy));
                let color= fractal.get_color(iterations);
                colors.push(color);
            }
        }
        average_color(&colors)
    }

    pub fn generate_image(&mut self, fractal: &Mandelbrot) -> Image {
        let mut image = Image::new(self.image_width, self.image_height);

        for y in 0..image.height {
            for x in 0..image.width {
                let color = self.calculate_pixel(x, y, self.super_sampling, fractal);
                image.try_set_pixel(x, y, color);
            }
        }

        image
    }
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    fractal: Mandelbrot,
    viewport: Viewport,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            fractal: Mandelbrot::default(),
            viewport: Viewport::default(),
        }
    }
}

impl Scene {
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn generate_image(&mut self) -> Image {
        self.viewport.generate_image(&self.fractal)
    }
}