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

pub struct SubPixel {
    pub offset: Vec3,
    pub size: Vec3,
}

pub struct SubpixelPattern<'a> {
    pub subpixels: &'a Vec<SubPixel>,
}

// To improve rendering of the pixel beyond using a full block, we subdivide
// a raytraced pixel into subpixels and try to map the subpixel pattern onto
// a character that best represents the subpixel pattern.
pub struct RenderPixel<'a> {
    pub subpixel_pattern: &'a SubpixelPattern<'a>,
    pub colors: Vec<Color>,
}
