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
    time::{Duration, Instant},
    panic::{set_hook, take_hook},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::Color,
    symbols::Marker,
    terminal::{Frame, Terminal},
    widgets::{
        canvas::{Canvas, Circle, Rectangle},
        Block, Widget,
    },
    prelude::*,
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
    ball: Circle,
    playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
    marker: Marker,
    color_buffer: Vec<Vec<Color>>,
    display_state: ImageDisplayState
}

impl App {
    fn new() -> Self {
        Self {
            x: 30.0,
            y: 30.0,
            ball: Circle {
                x: 20.0,
                y: 20.0,
                radius: 10.0,
                color: Color::Yellow,
            },
            playground: Rect::new(10, 10, 200, 100),
            vx: 1.0,
            vy: 1.0,
            tick_count: 0,
            marker: Marker::Dot,
            color_buffer: vec![vec![Color::Black; 200]; 100], // TODO: figure out how to update the size of the buffer
            display_state: ImageDisplayState {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            }
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

    fn on_tick(&mut self) {
        self.tick_count += 1;
        self.marker = Marker::HalfBlock;
        // // only change marker every 180 ticks (3s) to avoid stroboscopic effect
        // if (self.tick_count % 180) == 0 {

        
        // update the color buffer to a grayscale gradient
        for (_yi, row) in self.color_buffer.iter_mut().enumerate() {
            for (xi, pixel) in row.iter_mut().enumerate() {
                let l = xi.try_into().unwrap();
                
                *pixel = Color::Rgb(l, l, l);
            }
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
        //let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [render_area, menu_area] = horizontal.areas(frame.size());

        frame.render_widget(self.pong_canvas(), menu_area);
        //frame.render_widget(self.boxes_canvas(boxes), boxes);
        //RgbSwatch.render(boxes, frame.buffer_mut())
        //frame.render_widget(self.image_canvas(), boxes)
        
        frame.render_stateful_widget(ImageDisplay::new(self.color_buffer.clone()), render_area, &mut self.display_state);
    }
    
    fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Pong"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball);
                ctx.draw(&Rectangle {
                    x: self.x ,
                    y: self.y,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Green,
                });
            })
            .x_bounds([10.0, 210.0])
            .y_bounds([10.0, 110.0])
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