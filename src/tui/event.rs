use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
}

pub fn next(tick_rate: Duration) -> std::io::Result<Event> {
    if event::poll(tick_rate)? {
        match event::read()? {
            CrosstermEvent::Key(key) => Ok(Event::Key(key)),
            CrosstermEvent::Mouse(mouse) => Ok(Event::Mouse(mouse)),
            CrosstermEvent::Resize(w, h) => Ok(Event::Resize(w, h)),
            _ => Ok(Event::Tick),
        }
    } else {
        Ok(Event::Tick)
    }
}
