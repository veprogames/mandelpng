use std::{error::Error, fmt::Display, io::{self, BufWriter}, thread};

use num_complex::Complex32;
use serde::{Deserialize, Serialize};

use crate::{fractal::Mandelbrot, palette::Color, utils::{average_color, remap}};

#[derive(Debug)]
pub enum ImageError{
    Creation{ got_len: usize, expected: usize },
    Threading,
}

impl Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Creation { got_len, expected } => 
                write!(f, "ImageCreationError (got data len {}, expected {}", got_len, expected),
            Self::Threading => write!(f, "Threading Error"),
        }
    }
}

impl Error for ImageError {
    fn description(&self) -> &str {
        "Failed to create Image"
    }
}

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

    pub fn from(width: usize, height: usize, data: Vec<u8>) -> Result<Self, ImageError> {
        if width * height * Self::channel_count() != data.len() {
            Err(ImageError::Creation {
                got_len: data.len(),
                expected: width * height * Self::channel_count(),
            })
        } else {
            Ok(Self {
                width,
                height,
                data
            })
        }
    }

    fn channel_count() -> usize {
        3
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

struct RenderTask {
    viewport: Viewport,
    fractal: Mandelbrot,
    ymin: usize,
    ymax: usize,
}

impl RenderTask {
    fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let (width_f32, height_f32) = (self.viewport.image_width as f32, self.viewport.image_height as f32);
        let (cx, cy) = (self.viewport.cx, self.viewport.cy);
        let zoom = self.viewport.zoom;
        let aspect_ratio = width_f32 / height_f32;

        (
            remap(
                screen_x,
                0.0,
                width_f32,
                cx - zoom * aspect_ratio,
                cy + zoom * aspect_ratio,
            ),
            remap(
                screen_y,
                0.0,
                height_f32,
                cy - zoom,
                cy + zoom,
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

    pub fn run(&self) -> Vec<u8> {
        let RenderTask{ymin, ymax, ..} = self;
        let image_width = self.viewport.image_width;
        
        let mut data = vec![];

        for y in *ymin..=*ymax {
            for x in 0..image_width {
                let color = self.calculate_pixel(x, y, self.viewport.super_sampling, &self.fractal);
                data.append(&mut vec![color.r(), color.g(), color.b()]);
            }
        }

        data
    } 
}

#[derive(Serialize, Deserialize, Clone)]
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

    fn create_tasks(&self) -> Vec<RenderTask> {
        let h = self.viewport.image_height;
        let step = h / 64;
        let mut tasks = vec![];
        
        for y in (0..h).step_by(step) {
            tasks.push(RenderTask {
                viewport: self.viewport.clone(),
                fractal: self.fractal.clone(),
                ymin: y,
                ymax: (y + step - 1).min(h - 1),
            });
        }

        tasks
    }

    pub fn generate_image(&self) -> Result<Image, ImageError> {
        let mut data: Vec<u8> = Vec::with_capacity(self.viewport.image_width * self.viewport.image_height * Image::channel_count());

        let mut handles = vec![];

        for task in self.create_tasks() {
            handles.push(thread::spawn(move || {    
                task.run()
            }));
        }

        for h in handles {
            data.append(&mut h.join().map_err(|_| ImageError::Threading)?);
        }

        Image::from(self.viewport.image_width, self.viewport.image_height, data)
    }
}