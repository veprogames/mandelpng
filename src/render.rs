use num_complex::Complex32;
use serde::{Deserialize, Serialize};

use crate::{fractal::Mandelbrot, palette::{Color, Palette}};

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
}

#[derive(Serialize, Deserialize)]
pub struct Viewport {
    image: Image,
    cx: f32,
    cy: f32,
    zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            image: Image::new(1920, 1080),
            cx: -0.5,
            cy: 0.0,
            zoom: 2.0,
        }
    }
}

impl Viewport {
    pub fn image_width(&self) -> usize {
        self.image.width
    }

    pub fn image_height(&self) -> usize {
        self.image.height
    }

    fn screen_to_world(&self, screen_x: usize, screen_y: usize) -> (f32, f32) {
        let (width_f32, height_f32) = (self.image.width as f32, self.image.height as f32);
        let aspect_ratio = width_f32 / height_f32;

        (
            remap(
                screen_x as f32,
                0.0,
                width_f32,
                self.cx - self.zoom * aspect_ratio,
                self.cx + self.zoom * aspect_ratio,
            ),
            remap(
                screen_y as f32,
                0.0,
                height_f32,
                self.cy - self.zoom,
                self.cy + self.zoom,
            ),
        )
    }

    pub fn generate_image(&mut self, fractal: &Mandelbrot) -> &[u8] {
        let palette = Palette::new(vec![
            Color::new(67, 53, 167),
            Color::new(128, 196, 233),
            Color::new(255, 246, 233),
            Color::new(255, 127, 62),
        ]).make_looped();

        assert!(!palette.is_empty());

        for y in 0..self.image.height {
            for x in 0..self.image.width {
                let (cx, cy) = self.screen_to_world(x, y);
                let iterations = 2 * fractal.get_iterations(Complex32::new(cx, cy));
                let color = fractal.get_color(iterations);
                self.image.try_set_pixel(x, y, color);
            }
        }

        &self.image.data
    }
}

fn remap(v: f32, i0: f32, i1: f32, o0: f32, o1: f32) -> f32 {
    let fact = (o1 - o0) / (i1 - i0);
    (v - i0) * fact + o0
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

    pub fn get_image_data(&mut self) -> &[u8] {
        self.viewport.generate_image(&self.fractal)
    }
}