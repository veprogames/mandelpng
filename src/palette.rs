use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }

    // helper function not done as a trait here
    fn lerp_u8(a: u8, b: u8, weight: f32) -> u8 {
        assert!(weight >= 0.0 && weight <= 1.0);

        if b >= a {
            a + ((b - a) as f32 * weight) as u8
        } else {
            b + ((a - b) as f32 * (1.0 - weight)) as u8
        }
    }

    pub fn lerp(&self, to: &Color, weight: f32) -> Color {
        assert!(weight >= 0.0 && weight <= 1.0);

        Color(
            Self::lerp_u8(self.0, to.0, weight),
            Self::lerp_u8(self.1, to.1, weight),
            Self::lerp_u8(self.2, to.2, weight),
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Palette {
    colors: Vec<Color>,
}

impl Default for Palette {
    fn default() -> Self {
        Palette { colors: vec![] }
    }
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        Palette { colors }
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    pub fn sample(&self, pos: f32) -> Option<Color> {
        assert!(pos >= 0.0 && pos <= 1.0);
        
        if self.is_empty() {
            return None;
        }

        if pos >= 1.0 {
            return self.colors.last()
                .cloned();
        }

        let end_f32 = (self.colors.len() - 1) as f32;

        let idx_f32 = end_f32 * pos;
        let idx = idx_f32 as usize;
        let weight = idx_f32 % 1.0;
        match (self.colors.get(idx), self.colors.get(idx + 1)) {
            (Some(from), Some(to)) => Some(from.lerp(to, weight)),
            (Some(color), None) => Some(color.clone()),
            _ => None
        }
    }

    pub fn make_looped(mut self) -> Self {
        match self.colors.first() {
            Some(color) => {
                self.colors.push(color.clone());
                return self;
            },
            None => self
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BailoutPalette {
    inner: Color,
    outer: Palette,
    escape_radius: f32,
}

impl Default for BailoutPalette {
    fn default() -> Self {
        Self {
            escape_radius: 4.0,
            inner: Color(0, 0, 0),
            outer: Palette::new(vec![
                Color(0, 0, 0),
                Color(255, 255, 255)
            ]).make_looped()
        }
    }
}

impl BailoutPalette {
    pub fn escape_radius(&self) -> f32 {
        self.escape_radius
    }

    pub fn inner(&self) -> Color {
        self.inner
    }

    pub fn outer(&self) -> &Palette {
        &self.outer
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::Color;

    use super::Palette;

    #[test]
    fn test_interpolation_u8() {
        assert_eq!(Color::lerp_u8(100u8, 200u8, 0.75), 175u8);
        assert_eq!(Color::lerp_u8(200u8, 100u8, 0.75), 125u8);
        assert_eq!(Color::lerp_u8(100u8, 200u8, 1.0), 200u8);
        assert_eq!(Color::lerp_u8(200u8, 100u8, 1.0), 100u8);
    }

    #[test]
    fn test_color_lerp() {
        assert_eq!(Color::new(0, 0, 0).lerp(&Color::new(200, 160, 120), 0.5), Color::new(100, 80, 60));
        assert_eq!(Color::new(200, 160, 120).lerp(&Color::new(0, 0, 0), 0.5), Color::new(100, 80, 60));
    }

    #[test]
    fn test_palette_sample() {
        let palette = Palette::new(vec![
            Color::new(200, 0, 0),
            Color::new(0, 200, 0),
            Color::new(0, 0, 200),
            Color::new(0, 200, 200),
            Color::new(200, 200, 200),
        ]).make_looped();

        assert_eq!(palette.colors.len(), 6);

        assert_eq!(palette.sample(0.0), Some(Color::new(200, 0, 0)));
        assert_eq!(palette.sample(0.2), Some(Color::new(0, 200, 0)));
        assert_eq!(palette.sample(0.1), Some(Color::new(100, 100, 0)));
        assert_eq!(palette.sample(0.7), Some(Color::new(100, 200, 200)));
        assert_eq!(palette.sample(1.0), Some(Color::new(200, 0, 0)));

        let palette = Palette::new(vec![Color::new(100, 100, 100)]);
        assert_eq!(palette.sample(0.0), Some(Color::new(100, 100, 100)));
        assert_eq!(palette.sample(0.5), Some(Color::new(100, 100, 100)));
        assert_eq!(palette.sample(1.0), Some(Color::new(100, 100, 100)));

        let palette = Palette::new(vec![]);
        assert_eq!(palette.sample(0.0), None);
        assert_eq!(palette.sample(0.5), None);
        assert_eq!(palette.sample(1.0), None);
    }
}