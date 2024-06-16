//! # [Ratatui] Canvas example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::{
    io::{self, stdout, Stdout},
    panic::{set_hook, take_hook},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::Color,
    symbols::Marker,
    terminal::{Frame, Terminal},
    widgets::{block::Title, Block, Borders, Gauge, Padding, Paragraph, Widget},
};

fn main() -> Result<()> {
    init_panic_hook();
    let terminal = &mut init_tui()?;
    App::new().run(terminal)?;
    restore_tui();
    Ok(())
}

pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

mod buffer_display;
use buffer_display::{ImageDisplay, ImageDisplayState};
use color_eyre::Result;

struct App {
    x: f64,
    y: f64,
    tick_count: u64,
    marker: Marker,
    color_buffer: Vec<Vec<Color>>,
    display_state: ImageDisplayState,
    next_line_to_process: usize,
    progress_percentage: f64,
}

fn title_block(title: &str) -> Block {
    let title = Title::from(title).alignment(Alignment::Center);
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(Color::White)
}

impl App {
    fn new() -> Self {
        Self {
            x: 30.0,
            y: 30.0,
            tick_count: 0,
            marker: Marker::Dot,
            color_buffer: vec![vec![Color::Black; 200]; 100], // TODO: figure out how to update the size of the buffer
            display_state: ImageDisplayState {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            },
            next_line_to_process: 0,
            progress_percentage: 0f64,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        loop {
            let _ = terminal.draw(|frame| self.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => self.display_state.zoom += 0.1,
                        KeyCode::Up | KeyCode::Char('k') => self.display_state.zoom -= 0.1,
                        KeyCode::Right | KeyCode::Char('l') => self.x += 1.0,
                        KeyCode::Left | KeyCode::Char('h') => self.x -= 1.0,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn render_line(&mut self, line_index: usize) {
        let mut row = self.color_buffer.get_mut(line_index).unwrap();
        for (xi, pixel) in row.iter_mut().enumerate() {
            let l = xi.try_into().unwrap();

            *pixel = Color::Rgb(l, l, l);
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        self.marker = Marker::HalfBlock;

        // update the color buffer to a grayscale gradient
        if self.next_line_to_process < self.color_buffer.len() {
            self.render_line(self.next_line_to_process);
            self.next_line_to_process += 1;
            self.progress_percentage =
                self.next_line_to_process as f64 / self.color_buffer.len() as f64;
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
        let [render_area, menu_area] = horizontal.areas(frame.size());

        self.render_side_panel(menu_area, frame.buffer_mut());
        frame.render_stateful_widget(
            ImageDisplay::new(self.color_buffer.clone()),
            render_area,
            &mut self.display_state,
        );
    }

    fn render_side_panel(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Progress");
        let label = format!("{:.1}%", self.progress_percentage * 100.0);
        Gauge::default()
            .block(title)
            .gauge_style(Color::Green)
            .ratio(self.progress_percentage)
            .label(label)
            .render(area, buf);
    }
}

fn init_tui() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
