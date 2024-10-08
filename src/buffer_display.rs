use crate::renderer::FrameBuffer;
use crate::terminal::TerminalPixel;
use ratatui::prelude::{Buffer, Rect, StatefulWidget};

/// A widget that renders a buffer
pub struct ImageDisplay<'a> {
    image_buffer: &'a FrameBuffer,
}

impl<'a> ImageDisplay<'a> {
    pub fn new(image_buffer: &'a FrameBuffer) -> Self {
        Self { image_buffer }
    }

    fn draw_pixel(&self, buf: &mut Buffer, x: u16, y: u16, pixel: TerminalPixel) {
        if x >= buf.area().width || y >= buf.area().height {
            return;
        }
        // Draw the pixel on the buffer
        buf.get_mut(x, y)
            .set_char(pixel.character)
            .set_fg(pixel.fg.to_color())
            .set_bg(pixel.bg.to_color());
    }
}

// stores the position and zoom
pub struct ImageDisplayState {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl<'a> StatefulWidget for ImageDisplay<'a> {
    type State = ImageDisplayState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // This function draws the image_buffer to the target buffer
        // using the state to determine the position and zoom
        let image_width = self.image_buffer.width as f64;
        let image_height = self.image_buffer.height as f64;

        // loop over the output pixels and draw the corresponding pixel from the source image
        for y in 0..area.height {
            for x in 0..area.width {
                // calculate the position in the source image
                //  - take into account state.zoom
                //  - assume that when state.x/y is 0, the image is displayed in the center
                let src_x = (x as f64 - state.x) * state.zoom - (area.width as f64 / 2.0)
                    + image_width / 2.0;
                let src_y = (y as f64 - state.y) * state.zoom - (area.height as f64 / 2.0)
                    + image_height / 2.0;

                // get the color from the source image
                if src_x >= image_width || src_y >= image_height || src_x < 0.0 || src_y < 0.0 {
                    self.draw_pixel(buf, x, y, TerminalPixel::default());
                    continue;
                }
                let pixel = self.image_buffer.get_pixel(src_x as usize, src_y as usize);

                // draw the pixel to the target buffer
                self.draw_pixel(buf, x, y, pixel);
            }
        }
    }
}
