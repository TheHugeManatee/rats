use ratatui::prelude::Color;

pub struct Renderer {
    color_buffer: Vec<Vec<Color>>,
    next_line_to_process: usize,
}

impl Renderer {
    fn get_color_buffer_size(&self) -> (usize, usize) {
        let width = self.color_buffer.len();
        let height = self.color_buffer[0].len();
        (width, height)
    }

    pub fn new(width: usize, height: usize) -> Renderer {
        let color_buffer = vec![vec![Color::default(); width]; height];
        Renderer {
            color_buffer,
            next_line_to_process: 0,
        }
    }

    pub fn get_color_buffer(&self) -> &Vec<Vec<Color>> {
        &self.color_buffer
    }

    pub fn get_progress_percentage(&self) -> f64 {
        let (width, height) = self.get_color_buffer_size();
        let total_pixels = width * height;
        let processed_pixels = self.next_line_to_process * width;
        processed_pixels as f64 / total_pixels as f64
    }

    pub fn render_step(&mut self) {
        // render line-by line, but only for a maximum of 10ms
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_millis(15) {
            if self.next_line_to_process < self.color_buffer.len() {
                self.render_line(self.next_line_to_process);
                self.next_line_to_process += 1;
            } else {
                break;
            }
        }
    }

    fn render_line(&mut self, line_index: usize) {
        let row = self.color_buffer.get_mut(line_index).unwrap();
        for (xi, pixel) in row.iter_mut().enumerate() {
            let r = xi as u8;
            let g = line_index as u8;

            *pixel = Color::Rgb(r, g, 0);
        }
    }
}
