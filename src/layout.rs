//! Geometry only: split the frame into header, footer and pane areas.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::Preview;
use crate::app::{App, Focus, InputMode};
use crate::utils::text_width;

/// Areas for one frame, plus the text cursor when an input is active.
pub(crate) struct Areas {
    pub(crate) header: Rect,
    pub(crate) footer: Rect,
    pub(crate) panes: Vec<(Focus, Rect)>,
    pub(crate) cursor: Option<(u16, u16)>,
}

pub(crate) fn compute(area: Rect, app: &App) -> Areas {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let (header, footer) = if app.config.reverse_rows {
        (vertical[2], vertical[0])
    } else {
        (vertical[0], vertical[2])
    };

    let has_parent =
        app.config.show_parent && (!app.parent_entries.is_empty() || app.focus == Focus::Parent);
    let has_preview = !matches!(app.preview_content, Preview::Empty);

    let order = pane_order(has_parent, has_preview, app.config.reverse_columns);
    let constraints = pane_constraints(&order, has_parent, has_preview);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(vertical[1]);

    Areas {
        header,
        footer,
        panes: order.into_iter().zip(chunks.iter().copied()).collect(),
        cursor: input_cursor(app, footer),
    }
}

/// Panes left-to-right (or right-to-left when reversed), skipping hidden ones.
fn pane_order(has_parent: bool, has_preview: bool, reverse: bool) -> Vec<Focus> {
    let mut order = Vec::with_capacity(3);
    if reverse {
        if has_preview {
            order.push(Focus::Preview);
        }
        order.push(Focus::Current);
        if has_parent {
            order.push(Focus::Parent);
        }
    } else {
        if has_parent {
            order.push(Focus::Parent);
        }
        order.push(Focus::Current);
        if has_preview {
            order.push(Focus::Preview);
        }
    }
    order
}

fn pane_constraints(order: &[Focus], has_parent: bool, has_preview: bool) -> Vec<Constraint> {
    let current = match (has_parent, has_preview) {
        (_, true) => 30,
        (true, false) => 80,
        (false, false) => 100,
    };
    order
        .iter()
        .map(|pane| match pane {
            Focus::Parent => Constraint::Percentage(20),
            Focus::Current => Constraint::Percentage(current),
            Focus::Preview => Constraint::Percentage(if has_parent { 50 } else { 70 }),
        })
        .collect()
}

/// Cursor position at the end of the footer input, if one is active.
fn input_cursor(app: &App, footer: Rect) -> Option<(u16, u16)> {
    if app.input_mode == InputMode::Normal {
        return None;
    }
    let x = footer
        .x
        .saturating_add(1)
        .saturating_add(text_width(&app.input_buffer))
        .min(footer.x + footer.width.saturating_sub(1));
    Some((x, footer.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_order_follows_settings() {
        assert_eq!(
            pane_order(true, true, false),
            vec![Focus::Parent, Focus::Current, Focus::Preview]
        );
        assert_eq!(
            pane_order(true, true, true),
            vec![Focus::Preview, Focus::Current, Focus::Parent]
        );
        assert_eq!(
            pane_order(false, true, false),
            vec![Focus::Current, Focus::Preview]
        );
        assert_eq!(
            pane_order(true, false, false),
            vec![Focus::Parent, Focus::Current]
        );
        assert_eq!(pane_order(false, false, false), vec![Focus::Current]);
    }
}
