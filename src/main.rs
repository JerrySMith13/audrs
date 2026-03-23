mod app;
mod backend;
mod graph;
mod tui;

use std::time::Duration;

use crossterm::event::{KeyCode, KeyModifiers};
use tui::event::Event;

fn main() -> std::io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new();

    while app.running {
        terminal.draw(|frame| tui::ui::render(&app, frame))?;

        match tui::event::next(Duration::from_millis(16))? {
            Event::Key(key) => match (key.modifiers, key.code) {
                (_, KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    app.quit();
                }
                _ => {}
            },
            _ => {}
        }
    }

    tui::restore()
}
