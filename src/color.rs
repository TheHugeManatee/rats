use crate::maths::Interval;
pub type Color = crate::maths::Vec3;

impl Color {
    pub fn to_color(&self) -> ratatui::prelude::Color {
        let intensity = Interval {
            min: 0.0,
            max: 0.999,
        };
        ratatui::prelude::Color::Rgb(
            (256.0 * intensity.clamp(self.x)) as u8,
            (256.0 * intensity.clamp(self.y)) as u8,
            (256.0 * intensity.clamp(self.z)) as u8,
        )
    }

    pub fn saturated(&self) -> Color {
        Color {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            z: self.z.clamp(0.0, 1.0),
        }
    }
}
