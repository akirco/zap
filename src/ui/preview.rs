//! Preview pane: directory listing, file contents, grep results or an empty box.

use ratatui::{
    Frame,
    layout::Rect,
    text::Text,
    widgets::{Paragraph, Wrap},
};

use super::{block::pane_block, list::render_list};
use crate::app::Preview;
use crate::app::{App, Focus};

pub(crate) fn render_preview(f: &mut Frame, area: Rect, app: &mut App) {
    let title = if matches!(app.preview_content, Preview::Grep { .. }) {
        app.config.titles.grep.as_str()
    } else {
        app.config.titles.preview.as_str()
    };
    let block = pane_block(title, &app.theme, &app.config.borders.preview);

    match &app.preview_content {
        Preview::Dir(_) => render_list(f, area, app, Focus::Preview),
        Preview::File(preview) => {
            let text = Paragraph::new(Text::from(preview.text.as_str()))
                .block(block)
                .wrap(Wrap { trim: true })
                .scroll((app.preview_scroll, 0));
            f.render_widget(text, area);
        }
        Preview::Grep { lines, .. } => {
            let text = Paragraph::new(lines.clone())
                .block(block)
                .wrap(Wrap { trim: true })
                .scroll((app.preview_scroll, 0));
            f.render_widget(text, area);
        }
        Preview::Empty => f.render_widget(block, area),
    }
}
