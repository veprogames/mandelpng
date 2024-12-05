use num_complex::Complex32;
use serde::{Serialize, Deserialize};

use crate::palette::{BailoutPalette, Color};

#[derive(Serialize, Deserialize, Clone)]
pub enum MandelbrotMode {
    Normal,
    Julia(Complex32),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mandelbrot {
    z0: Complex32,
    max_iterations: u32,
    mode: MandelbrotMode,
    bailout: BailoutPalette
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Mandelbrot {
            mode: MandelbrotMode::Normal,
            z0: Complex32::ZERO,
            max_iterations: 128,
            bailout: BailoutPalette::default(),
        }
    }
}

impl Mandelbrot {
    pub fn get_iterations(&self, c: Complex32) -> u32 {
        let mut z = match self.mode {
            MandelbrotMode::Normal => self.z0,
            MandelbrotMode::Julia(_) => self.z0.powi(2) + c,
        };

        let c = match self.mode {
            MandelbrotMode::Normal => c,
            MandelbrotMode::Julia(jc) => jc,
        };
    
        for iteration in 0..self.max_iterations {
            z = z.powi(2) + c;
    
            if z.re.powf(2.0) + z.im.powf(2.0) > self.bailout.escape_radius() {
                return iteration;
            }
        }
    
        self.max_iterations
    }

    pub fn get_color(&self, iterations: u32) -> Color {
        if iterations >= self.max_iterations {
            self.bailout.inner()
        } else {
            let pos = (iterations as f32 / 32.0) % 1.0;
            self.bailout.outer().sample(pos)
                .unwrap_or(Color::new(255, 255, 255))
        }
    }
}