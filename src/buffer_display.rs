use ratatui::prelude::*;

/// A widget that renders a buffer
pub struct ImageDisplay {
    image_buffer: Vec<Vec<Color>>,
}

impl ImageDisplay {
    // Implement the new() method
    pub fn new(image_buffer: Vec<Vec<Color>>) -> Self {
        ImageDisplay { image_buffer }
    }
}


// stores the position and zoom
pub struct ImageDisplayState {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl StatefulWidget for ImageDisplay {
    type State = ImageDisplayState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Calculate the number of image pixels to display based on the zoom level
        let display_width = (area.width as f64 * state.zoom).round() as usize;
        let display_height = (area.height as f64 * state.zoom).round() as usize;

        // Calculate the starting point in the image based on the current position
        let start_x = state.x as usize;
        let start_y = state.y as usize;

        // Calculate the step size for iterating over the image pixels
        let step_x = if display_width > 0 { self.image_buffer[0].len() as f64 / display_width as f64 } else { 0.0 };
        let step_y = if display_height > 0 { self.image_buffer.len() as f64 / display_height as f64 } else { 0.0 };

        for y in 0..area.height as usize {
            for x in 0..area.width as usize {
                // Determine the corresponding pixel in the image
                let image_x = (start_x as f64 + x as f64 * step_x).round() as usize;
                let image_y = (start_y as f64 + y as f64 * step_y).round() as usize;

                if image_y < self.image_buffer.len() && image_x < self.image_buffer[image_y].len() {
                    let color = self.image_buffer[image_y][image_x];
                    let buffer_x = area.left() + x as u16;
                    let buffer_y = area.top() + y as u16;

                    // Draw the pixel on the buffer
                    buf.get_mut(buffer_x, buffer_y)
                        .set_char(' ')
                        .set_fg(color)
                        .set_bg(color);
                }
            }
        }
    }
}