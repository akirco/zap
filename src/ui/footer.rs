//! Bottom bar: selected entry info, or the active input line.

use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use super::styles::{footer_input_style, footer_normal_style};
use crate::app::{App, InputMode};
use crate::utils::file_summary;

pub(crate) fn render(f: &mut Frame, area: Rect, app: &App) {
    let (text, style) = match app.input_mode {
        InputMode::Normal => {
            let info = app
                .selected_entry()
                .map(file_summary)
                .unwrap_or_else(|| app.config.strings.no_selection.clone());
            (info, footer_normal_style(&app.theme))
        }
        InputMode::Search => (
            format!("/{}", app.input_buffer),
            footer_input_style(&app.theme),
        ),
        InputMode::Command => (
            format!(":{}", app.input_buffer),
            footer_input_style(&app.theme),
        ),
        InputMode::Grep => (
            format!("?{}{}", app.input_buffer, app.config.strings.grep_suffix),
            footer_input_style(&app.theme),
        ),
    };

    f.render_widget(Paragraph::new(Line::from(text)).style(style), area);
}
