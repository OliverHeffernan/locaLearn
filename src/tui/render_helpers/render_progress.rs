use ratatui::{
    Frame,
    widgets::{Block, Gauge},
    layout::Rect,
    style::{Color, Style},
};

fn color_for_ratio(ratio: f64) -> Color {
    if ratio < 0.25 {
        Color::Red
    } else if ratio < 0.75 {
        Color::Yellow
    } else {
        Color::Green
    }
}
pub fn render_progress(frame: &mut Frame, area: Rect, ratio: f64) {
    let color = color_for_ratio(ratio);
    frame.render_widget(
        Gauge::default()
            .gauge_style(Style::new().fg(color))
            .label("")
            .block(
                Block::default().style(Style::default()
                    .fg(color)
                    .bg(Color::DarkGray)
                )
            )
            .ratio(ratio),
        area,
    );
}
