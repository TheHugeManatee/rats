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

    pub fn gamma_correction(color: &Color) -> Color {
        Color {
            x: color.x.sqrt(),
            y: color.y.sqrt(),
            z: color.z.sqrt(),
        }
    }

    pub fn brightness(&self) -> f64 {
        0.299 * self.x + 0.587 * self.y + 0.114 * self.z
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
    pub fn pink() -> Color {
        Color::new(1.0, 0.0, 1.0)
    }
}
