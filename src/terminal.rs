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
            colors: [[Color::black(); SUBPIXEL_X]; SUBPIXEL_Y],
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.colors[y][x]
    }
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        self.colors[y][x] = color;
    }

    pub fn average_color(&self) -> Color {
        let mut color = Color::black();
        for row in self.colors.iter() {
            for c in row.iter() {
                color += *c;
            }
        }
        color / (SUBPIXEL_X * SUBPIXEL_Y) as f64
    }

    pub fn to_terminal_pixel(&self) -> TerminalPixel {
        // // average the upper and the lower half separately
        // let color_upper =
        //     (self.colors[0][0] + self.colors[0][1] + self.colors[1][0] + self.colors[1][1]) / 4.0;
        // let color_lower =
        //     (self.colors[2][0] + self.colors[2][1] + self.colors[3][0] + self.colors[3][1]) / 4.0;
        // return TerminalPixel::new(color_lower, color_upper, '▄');

        // foreground pixels are part of the first cluster, which is the "darker" one
        let mut is_foreground_pixel: &mut [[bool; SUBPIXEL_X]; SUBPIXEL_Y] =
            &mut [[false; SUBPIXEL_X]; SUBPIXEL_Y];
        let (color_fg, color_bg) =
            foreground_background_detection(&self.colors, is_foreground_pixel);

        //if colors are too similar, just use a space with the average value
        // let similarity_threshold = (0.01f64).powi(2);
        // if (color_fg - color_bg).length_squared() < similarity_threshold {
        //     let color_avg = (color_fg + color_bg) / 2.0;
        //     return TerminalPixel::new(Color::zero(), color_avg, ' ');
        // }

        //let half_block = '▄';
        //let shade_chars = ['█', '▓', '▒', '░', ' '];
        // ▁▂▃▄▅▆▇█
        //let lower_block_increase = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇'];
        //  ▏▎▍▌▋▊▉█
        //let left_block_increase = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        //let debug_map = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

        // reformats the is_foreground_pixel array into a bitmask where the
        // bits correspond to the subpixels in the following order:
        //  1 2
        //  3 4
        //  5 6
        //  7 8
        let foreground_pixel_bitmask = is_foreground_pixel
            .iter()
            .flatten()
            .enumerate()
            .fold(0, |acc, (i, &b)| acc | ((b as usize) << i));

        let braille_char = corresponding_braille_char(foreground_pixel_bitmask as u32);
        // for debugging: visualize the cluster bitmask with braille chars
        // determine the popcount, i.e. the number of set bits in the bitmask
        let popcount = foreground_pixel_bitmask.count_ones();
        let px_color = if popcount > 4 { color_fg } else { color_bg };
        if px_color.brightness() > 0.5 {
            return TerminalPixel::new(Color::black(), px_color, braille_char);
        }

        TerminalPixel::new(Color::white(), px_color, braille_char)

        //return TerminalPixel::new(color_fg, color_bg, braille_char);
    }
}

// basic k-means clustering algorithm with k=2, assigning the 8 subpixels to either foreground or background
fn foreground_background_detection(
    colors: &[[Color; SUBPIXEL_X]; SUBPIXEL_Y],
    is_foreground: &mut [[bool; SUBPIXEL_X]; SUBPIXEL_Y],
) -> (Color, Color) {
    let mut cluster_centers = [Color::default(); 2];
    // initialize cluster centers
    cluster_centers[0] = colors[0][0];
    // find the first color in colors that is not equal to the first cluster center
    let mut found = false;
    for row in colors.iter() {
        for color in row.iter() {
            if *color != cluster_centers[0] {
                cluster_centers[1] = *color;
                found = true;
                break;
            }
        }
    }
    // if no other color was found, return the same color for both clusters
    if !found {
        return (cluster_centers[0], cluster_centers[0]);
    }
    // make sure the first cluster center is the darker one
    if cluster_centers[0].brightness() > cluster_centers[1].brightness() {
        cluster_centers.swap(0, 1);
    }

    for _ in 0..16 {
        let mut new_cluster_centers = [Color::default(); 2];
        let mut new_cluster_counts = [0; 2];
        // assign each color to a cluster
        for (ri, row) in colors.iter().enumerate() {
            for (ci, col) in row.iter().enumerate() {
                let color = *col;
                let dist0 = (color - cluster_centers[0]).length_squared();
                let dist1 = (color - cluster_centers[1]).length_squared();
                if dist0 < dist1 {
                    new_cluster_centers[0] += color;
                    new_cluster_counts[0] += 1;
                    is_foreground[ri][ci] = true;
                } else {
                    new_cluster_centers[1] += color;
                    new_cluster_counts[1] += 1;
                    is_foreground[ri][ci] = false;
                }
            }
        }
        // average the cluster centers
        if new_cluster_counts[0] != 0 {
            cluster_centers[0] = new_cluster_centers[0] / new_cluster_counts[0] as f64;
        }
        if new_cluster_counts[1] != 0 {
            cluster_centers[1] = new_cluster_centers[1] / new_cluster_counts[1] as f64;
        }
    }

    // return the average of the cluster centers
    (cluster_centers[0], cluster_centers[1])
}

#[rustfmt::skip]
#[allow(clippy::identity_op)]
fn corresponding_braille_char(bitmask: u32) -> char {
    // the braille unicode block starts at 0x2800
    let braille_start = '⠀';
    // but the dots are not arranged in the right order to directly map to the bitmask
    // the braille dots are arranged like this
    //  1 4
    //  2 5
    //  3 6
    //  7 8
    // so we need to rearrange our bits to match the braille order:   
    //  1 2
    //  3 4
    //  5 6
    //  7 8
    let braille_offset = 
          (bitmask & 0b0000_0001) << 0
        | (bitmask & 0b0000_0010) << 2
        | (bitmask & 0b0000_0100) >> 1
        | (bitmask & 0b0000_1000) << 1
        | (bitmask & 0b0001_0000) >> 2
        | (bitmask & 0b0010_0000) << 0
        | (bitmask & 0b0100_0000) << 0
        | (bitmask & 0b1000_0000) << 0;
    // return the corresponding braille character
    std::char::from_u32(braille_start as u32 + braille_offset).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmask_to_braille() {
        // Single braille dots: ⠁⠈⠂⠐⠄⠠⡀⢀
        assert_eq!(corresponding_braille_char(0b0000_0000), '⠀');

        assert_eq!(corresponding_braille_char(0b0000_0001), '⠁');
        assert_eq!(corresponding_braille_char(0b0000_0010), '⠈');
        assert_eq!(corresponding_braille_char(0b0000_0100), '⠂');
        assert_eq!(corresponding_braille_char(0b0000_1000), '⠐');
        assert_eq!(corresponding_braille_char(0b0001_0000), '⠄');
        assert_eq!(corresponding_braille_char(0b0010_0000), '⠠');
        assert_eq!(corresponding_braille_char(0b0100_0000), '⡀');
        assert_eq!(corresponding_braille_char(0b1000_0000), '⢀');

        assert_eq!(corresponding_braille_char(0b0001_0101), '⠇');
        assert_eq!(corresponding_braille_char(0b1111_0000), '⣤');
        assert_eq!(corresponding_braille_char(0b1111_1111), '⣿');
    }

    #[test]
    fn kmeans_cluster_left_right() {
        let mut render_pixel = RenderPixel::new();
        render_pixel.set_color(0, 0, Color::black());
        render_pixel.set_color(1, 0, Color::white());
        render_pixel.set_color(0, 1, Color::black());
        render_pixel.set_color(1, 1, Color::white());
        render_pixel.set_color(0, 2, Color::black());
        render_pixel.set_color(1, 2, Color::white());
        render_pixel.set_color(0, 3, Color::black());
        render_pixel.set_color(1, 3, Color::white());

        let mut first_cluster = [[false; SUBPIXEL_X]; SUBPIXEL_Y];
        let (color_first, color_second) =
            foreground_background_detection(&render_pixel.colors, &mut first_cluster);

        assert_eq!(color_first, Color::black());
        assert_eq!(color_second, Color::white());
    }

    #[test]
    fn top_left_subpixel_to_braille() {
        let mut render_pixel = RenderPixel::new();
        render_pixel.set_color(0, 0, Color::white());
        render_pixel.set_color(1, 0, Color::black());
        render_pixel.set_color(0, 1, Color::black());
        render_pixel.set_color(1, 1, Color::black());
        render_pixel.set_color(0, 2, Color::black());
        render_pixel.set_color(1, 2, Color::black());
        render_pixel.set_color(0, 3, Color::black());
        render_pixel.set_color(1, 3, Color::black());

        let terminal_pixel = render_pixel.to_terminal_pixel();
        assert_eq!(terminal_pixel.character, '⣾');
    }
    #[test]
    fn top_right_subpixel_to_braille() {
        let mut render_pixel = RenderPixel::new();
        render_pixel.set_color(0, 0, Color::white());
        render_pixel.set_color(1, 0, Color::black());
        render_pixel.set_color(0, 1, Color::white());
        render_pixel.set_color(1, 1, Color::white());
        render_pixel.set_color(0, 2, Color::white());
        render_pixel.set_color(1, 2, Color::white());
        render_pixel.set_color(0, 3, Color::white());
        render_pixel.set_color(1, 3, Color::white());

        let terminal_pixel = render_pixel.to_terminal_pixel();
        assert_eq!(terminal_pixel.character, '⠈');
    }
    #[test]
    fn bottom_left_subpixel_to_braille() {
        let mut render_pixel = RenderPixel::new();
        render_pixel.set_color(0, 0, Color::white());
        render_pixel.set_color(1, 0, Color::white());
        render_pixel.set_color(0, 1, Color::white());
        render_pixel.set_color(1, 1, Color::white());
        render_pixel.set_color(0, 2, Color::white());
        render_pixel.set_color(1, 2, Color::white());
        render_pixel.set_color(0, 3, Color::black());
        render_pixel.set_color(1, 3, Color::white());

        let terminal_pixel = render_pixel.to_terminal_pixel();
        assert_eq!(terminal_pixel.character, '⡀');
    }
}
