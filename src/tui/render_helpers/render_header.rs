use ratatui::{
    layout::{Rect, Alignment},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render_header(frame: &mut Frame, area: Rect, mode: &str, stats: impl Into<String>) {
    frame.render_widget(
        Paragraph::new(format!("{mode} | {}", stats.into()))
            .block(Block::default())
            .alignment(Alignment::Center),
        area,
    );
}
