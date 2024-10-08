//! # [rats] Terminal ray tracing with ratatui
//!
//! This is a simple ray tracer in a terminal.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui

// for a lib in progress, dead code is useless
#![allow(dead_code)]

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
    widgets::{block::Title, Block, Borders, Gauge, Padding, Row, Table, Widget},
};

fn main() -> Result<()> {
    init_panic_hook();
    let terminal = &mut init_tui()?;
    App::new().run(terminal)?;
    restore_tui()?;
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
mod camera;
mod color;
mod geometry;
mod materials;
mod maths;
mod random;
mod renderer;
mod scene;
mod terminal;
use buffer_display::{ImageDisplay, ImageDisplayState};
use color_eyre::Result;
use renderer::Renderer;

struct App {
    tick_count: u64,
    marker: Marker,
    renderer: Renderer,
    display_state: ImageDisplayState,
    show_side_panel: bool,
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
            tick_count: 0,
            marker: Marker::Dot,
            renderer: Renderer::new(192, 72),
            display_state: ImageDisplayState {
                x: 0.0,
                y: 0.0,
                zoom: 1.0,
            },
            show_side_panel: true,
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
                        KeyCode::Char(' ') => self.show_side_panel = !self.show_side_panel,
                        KeyCode::Char('j') => self.display_state.zoom += 0.1,
                        KeyCode::Char('k') => self.display_state.zoom -= 0.1,
                        KeyCode::Char('r') => {
                            self.display_state.zoom = 1.0;
                            self.display_state.x = 0.0;
                            self.display_state.y = 0.0;
                        }
                        KeyCode::Right => self.display_state.x += 1.0,
                        KeyCode::Left => self.display_state.x -= 1.0,
                        KeyCode::Up => self.display_state.y -= 1.0,
                        KeyCode::Down => self.display_state.y += 1.0,
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

        self.renderer.render_step();
    }

    fn ui(&mut self, frame: &mut Frame) {
        if !self.show_side_panel {
            frame.render_stateful_widget(
                ImageDisplay::new(self.renderer.get_color_buffer()),
                frame.size(),
                &mut self.display_state,
            );
            return;
        }

        let horizontal =
            Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)]);
        let [render_area, menu_area] = horizontal.areas(frame.size());

        self.render_side_panel(menu_area, frame.buffer_mut());
        frame.render_stateful_widget(
            ImageDisplay::new(self.renderer.get_color_buffer()),
            render_area,
            &mut self.display_state,
        );
    }
    // Updated function based on the comments
    fn render_side_panel(&self, area: Rect, buf: &mut Buffer) {
        // Calculate and display the rendering resolution
        let resolution = format!(
            "{}x{}",
            self.renderer.get_color_buffer().width,
            self.renderer.get_color_buffer().height
        );
        let objects_count = format!("{}", self.renderer.get_scene_object_count());
        let render_duration = format!("{:.2?}", self.renderer.get_render_duration());

        // Create a table widget
        let widths = [
            Constraint::Length(area.width / 2),
            Constraint::Length(area.width / 2),
        ];
        let rows = [
            Row::new(vec!["Resolution", &resolution]),
            Row::new(vec!["Objects", &objects_count]),
            Row::new(vec!["Render Duration", &render_duration]),
        ];
        Widget::render(
            Table::new(rows, widths).header(Row::new(vec!["Metric", "Value"])),
            area,
            buf,
        );

        // Calculate and display the current progress gauge
        let progress = self.renderer.get_progress_percentage();
        let label = format!("{:.1}%", progress * 100.0);

        //let title = "Progress".to_string();
        //buf.set_string(area.left(), area.top(), title, Style::default());
        Gauge::default()
            .block(title_block("Progress"))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(progress)
            .label(label)
            .render(
                Rect::new(area.left(), area.bottom() - 4, area.width, 4),
                buf,
            );
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
