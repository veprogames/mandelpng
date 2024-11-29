use crate::palette::Color;

pub fn remap(v: f32, i0: f32, i1: f32, o0: f32, o1: f32) -> f32 {
    let fact = (o1 - o0) / (i1 - i0);
    (v - i0) * fact + o0
}

// FIXME: this should be done through a Trait, but idk how
pub fn average_color(colors: &[Color]) -> Color {
    let mut sum = (0.0f32, 0.0f32, 0.0f32);
    for col in colors {
        sum.0 += col.r() as f32;
        sum.1 += col.g() as f32;
        sum.2 += col.b() as f32;
    }
    let len_f32 = colors.len() as f32;
    Color::new((sum.0 / len_f32) as u8, (sum.1 / len_f32) as u8, (sum.2 / len_f32) as u8)
}