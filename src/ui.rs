//! ratatui widgets and the per-frame composition.

mod block;
mod footer;
mod header;
mod highlight;
mod item;
mod list;
mod preview;
mod span;
mod styles;

pub(crate) use highlight::highlight_lines;

use ratatui::Frame;

use crate::app::{App, Focus};
use crate::layout;

/// Draw one frame.
pub(crate) fn render(f: &mut Frame, app: &mut App) {
    let areas = layout::compute(f.area(), app);

    header::render(f, areas.header, app);
    footer::render(f, areas.footer, app);

    if let Some(position) = areas.cursor {
        f.set_cursor_position(position);
    }

    for (pane, area) in areas.panes {
        match pane {
            Focus::Parent => list::render_parent(f, area, app),
            Focus::Current => list::render_current(f, area, app),
            Focus::Preview => preview::render_preview(f, area, app),
        }
    }
}
