use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::Block,
};

use crate::app::App;

pub fn render(_app: &App, frame: &mut Frame) {
    let [tracks, mixer] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .areas(frame.area());

    let [timeline, transport] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .areas(tracks);

    frame.render_widget(Block::bordered().title("Timeline"), timeline);
    frame.render_widget(Block::bordered().title("Mixer"), mixer);
    frame.render_widget(Block::bordered().title("Transport"), transport);
}
