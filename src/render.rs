use num_complex::Complex32;

use crate::{fractal::Mandelbrot, palette::{Color, Palette}};

pub struct Viewport {
    image_width: usize,
    image_height: usize,
    cx: f32,
    cy: f32,
    zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            image_width: 1920,
            image_height: 1080,
            cx: -0.5,
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
        self.image_height
    }

    fn screen_to_world(&self, screen_x: usize, screen_y: usize) -> (f32, f32) {
        let aspect_ratio = self.image_width as f32 / self.image_height as f32;

        (
            remap(
                screen_x as f32,
                0.0,
                self.image_width as f32,
                self.cx - self.zoom * aspect_ratio,
                self.cx + self.zoom * aspect_ratio,
            ),
            remap(
                screen_y as f32,
                0.0,
                self.image_height as f32,
                self.cy - self.zoom,
                self.cy + self.zoom,
            ),
        )
    }

    /// Note: Fails silently if indexing fails :(
    fn try_set_pixel(&self, data: &mut [u8], x: usize, y: usize, rgb: Color) {
        let idx = (x + y * self.image_width) * 3;

        if data.get(idx).is_some() && data.get(idx + 2).is_some() {
            data[idx] = rgb.r();
            data[idx + 1] = rgb.g();
            data[idx + 2] = rgb.b();
        }
    }

    pub fn get_image_data(&self, fractal: Mandelbrot) -> Vec<u8> {
        let mut data = vec![0; self.image_width * self.image_height * 3];

        let palette = Palette::new(vec![
            Color::new(67, 53, 167),
            Color::new(128, 196, 233),
            Color::new(255, 246, 233),
            Color::new(255, 127, 62),
        ]).make_looped();

        assert!(!palette.is_empty());

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let (cx, cy) = self.screen_to_world(x, y);
                let iterations = 2 * fractal.get_iterations(Complex32::new(cx, cy));
                let color = fractal.get_color(iterations);
                self.try_set_pixel(&mut data, x, y, color);
            }
        }

        data.to_vec()
    }
}

fn remap(v: f32, i0: f32, i1: f32, o0: f32, o1: f32) -> f32 {
    let fact = (o1 - o0) / (i1 - i0);
    (v - i0) * fact + o0
}
