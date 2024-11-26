use num_complex::Complex32;

pub enum MandelbrotMode {
    Normal,
    Julia(Complex32),
}

pub struct Mandelbrot {
    z0: Complex32,
    max_iterations: u32,
    mode: MandelbrotMode,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        Mandelbrot {
            mode: MandelbrotMode::Normal,
            z0: Complex32::ZERO,
            max_iterations: 128,
        }
    }
}

impl Mandelbrot {
    pub fn get_iterations(&self, c: Complex32) -> u32 {
        let mut z = self.z0.powi(2) + c;

        let c = match self.mode {
            MandelbrotMode::Normal => c,
            MandelbrotMode::Julia(jc) => jc,
        };
    
        for iteration in 1..self.max_iterations {
            z = z.powi(2) + c;
    
            if z.re.powf(2.0) + z.im.powf(2.0) > 4.0 {
                return iteration;
            }
        }
    
        self.max_iterations
    }
}