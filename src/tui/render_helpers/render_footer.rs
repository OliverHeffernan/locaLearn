use ratatui::{
    widgets::{Block, Paragraph, Wrap},
    layout::{Rect, Alignment},
    style::{Color, Style},
    Frame,
};

pub fn render_footer(frame: &mut Frame, area: Rect, controls: &str) {
    frame.render_widget(
        Paragraph::new(controls.to_owned())
            .block(Block::default())
            .style(Style::default().bg(Color::DarkGray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        area,
    );
}
