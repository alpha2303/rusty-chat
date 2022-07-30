use std::error;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]

pub struct App {
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);
        frame.render_widget(
            Paragraph::new("Rust Chat")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Center),
            frame.size(),
        )
    }
}
