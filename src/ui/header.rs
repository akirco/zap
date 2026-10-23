//! Top bar: the current directory, shortened with `~`.

use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

use super::styles::header_style;
use crate::app::App;
use crate::utils::display_path;

pub(crate) fn render(f: &mut Frame, area: Rect, app: &App) {
    let path = display_path(&app.current_dir);
    let line = Line::from(format!("{}{}", app.config.strings.path_prefix, path))
        .style(header_style(&app.theme));
    f.render_widget(Paragraph::new(line), area);
}
