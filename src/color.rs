use crate::maths::Vec3;

type Color = ratatui::prelude::Color;
type color = crate::maths::Vec3;

#[allow(dead_code)]
impl color {
    pub fn to_color(&self) -> Color {
        Color::Rgb(
            (self.x * 255.99) as u8,
            (self.y * 255.99) as u8,
            (self.z * 255.99) as u8,
        )
    }

    pub fn saturated(&self) -> color {
        color {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            z: self.z.clamp(0.0, 1.0),
        }
    }
}
