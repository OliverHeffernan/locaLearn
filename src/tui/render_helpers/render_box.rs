use ratatui::{
    Frame,
    widgets::{Borders, Block, Paragraph, Wrap},
    layout::{Rect, Alignment},
    style::{Color, Style},
};
pub fn render_box(frame: &mut Frame, area: Rect, title: &str, text: &str, color: Color) {
    frame.render_widget(
        Paragraph::new(text.to_owned())
            .block(
                Block::default()
                    .title(title.to_owned())
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        area,
    );
}
