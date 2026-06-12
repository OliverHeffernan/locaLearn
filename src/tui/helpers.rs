use ratatui::layout::{Constraint, Layout, Rect};

pub fn study_layout(area: Rect) -> [Rect; 4] {
    Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Min(8),
        Constraint::Length(1),
    ])
    .areas(area)
}

pub fn progress_ratio(total: usize, remaining: usize) -> f64 {
    (total != 0)
        .then(|| (total.saturating_sub(remaining)) as f64 / total as f64)
        .unwrap_or(1.0)
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [_, vertical, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(area);
    let [_, horizontal, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(vertical);
    horizontal
}
