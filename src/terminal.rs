use crate::color::Color;
use crate::maths::Vec3;

// a terminal pixel is a pixel rendered onto the terminal.
#[derive(Debug, Clone, Copy)]
pub struct TerminalPixel {
    pub fg: Color,
    pub bg: Color,
    pub character: char,
}

impl TerminalPixel {
    pub fn new(fg: Color, bg: Color, character: char) -> Self {
        Self { fg, bg, character }
    }
}
impl Default for TerminalPixel {
    fn default() -> Self {
        Self {
            fg: Color::default(),
            bg: Color::default(),
            character: ' ',
        }
    }
}

// pub struct SubPixel {
//     pub offset: Vec3,
//     pub size: Vec3,
// }

// pub struct SubpixelPattern<'a> {
//     pub subpixels: &'a Vec<SubPixel>,
// }

// // To improve rendering of the pixel beyond using a full block, we subdivide
// // a raytraced pixel into subpixels and try to map the subpixel pattern onto
// // a character that best represents the subpixel pattern.
// pub struct RenderPixel<'a> {
//     pub subpixel_pattern: &'a SubpixelPattern<'a>,
//     pub colors: Vec<Color>,
// }

pub const SUBPIXEL_X: usize = 2;
pub const SUBPIXEL_Y: usize = 4;

#[derive(Default, Debug, Clone, Copy)]
pub struct RenderPixel {
    colors: [[Color; SUBPIXEL_X]; SUBPIXEL_Y],
}

impl RenderPixel {
    pub fn new() -> Self {
        Self {
            colors: [[Color::default(); SUBPIXEL_X]; SUBPIXEL_Y],
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors[y][x]
    }
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        self.colors[y][x] = color;
    }

    pub fn average_color(&self) -> Color {
        let mut color = Color::default();
        for row in self.colors.iter() {
            for c in row.iter() {
                color += *c;
            }
        }
        color / (SUBPIXEL_X * SUBPIXEL_Y) as f64
    }

    pub fn to_terminal_pixel(&self) -> TerminalPixel {
        // average the upper and the lower half separately
        let color_upper =
            (self.colors[0][0] + self.colors[0][1] + self.colors[1][0] + self.colors[1][1]) / 4.0;
        let color_lower =
            (self.colors[2][0] + self.colors[2][1] + self.colors[3][0] + self.colors[3][1]) / 4.0;

        let character = '▄'; // half block

        TerminalPixel::new(color_lower, color_upper, character)
    }
}
