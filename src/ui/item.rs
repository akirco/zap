//! Render one directory entry as a list row.

use std::path::Path;

use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::ListItem,
};

use super::{
    span::{icon_span, label_span},
    styles::{focus_selection_style, selection_style},
};
use crate::config::{Config, Theme};

/// Half-circle caps that round off the selected row's background.
const LEFT_CAP: &str = "\u{e0b6}";
const RIGHT_CAP: &str = "\u{e0b4}";
/// Reserved on unselected rows so names stay in the same column: one cell for
/// the cap and one for the capsule's inner padding.
const ROW_INDENT: &str = "  ";

pub(crate) fn render<'a>(
    path: &Path,
    config: &'a Config,
    theme: &Theme,
    is_active_dir: bool,
    is_selected: bool,
    is_focused: bool,
    width: usize,
) -> ListItem<'a> {
    let spans = vec![
        icon_span(path, config, theme, is_active_dir),
        label_span(path, theme, is_active_dir),
    ];

    let line = if is_selected {
        capsule(spans, theme, width, is_focused)
    } else {
        plain(spans)
    };

    ListItem::new(line)
}

/// Unselected row: indent by the caps' width, no background.
fn plain<'a>(spans: Vec<Span<'a>>) -> Line<'a> {
    let mut line = Vec::with_capacity(spans.len() + 1);
    line.push(Span::raw(ROW_INDENT));
    line.extend(spans);
    Line::from(line)
}

/// Selected row: fill the pane with the selection background and round both
/// ends with caps drawn in the selection colour, so they blend into the
/// background.
fn capsule<'a>(spans: Vec<Span<'a>>, theme: &Theme, width: usize, is_focused: bool) -> Line<'a> {
    let (style, cap) = if is_focused {
        (
            focus_selection_style(theme),
            Style::default().fg(theme.accent),
        )
    } else {
        (selection_style(theme), Style::default().fg(theme.selection))
    };
    let content_width: usize = spans.iter().map(Span::width).sum();
    let padding = width.saturating_sub(content_width + 3);

    let mut line = Vec::with_capacity(spans.len() + 4);
    line.push(Span::styled(LEFT_CAP, cap));
    line.push(Span::styled(" ", style));
    line.extend(
        spans
            .into_iter()
            .map(|span| Span::styled(span.content, style)),
    );
    line.push(Span::styled(" ".repeat(padding), style));
    line.push(Span::styled(RIGHT_CAP, cap));
    Line::from(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Theme;

    #[test]
    fn selection_rows_fill_the_pane_width() {
        let theme = Theme::default();
        let spans = || vec![Span::raw("abc")];

        for width in [10, 40] {
            assert_eq!(capsule(spans(), &theme, width, true).width(), width);
            assert_eq!(capsule(spans(), &theme, width, false).width(), width);
        }

        let focused = capsule(spans(), &theme, 10, true);
        assert_eq!(focused.spans.first().unwrap().content.as_ref(), LEFT_CAP);
        assert_eq!(focused.spans.last().unwrap().content.as_ref(), RIGHT_CAP);
        assert_eq!(focused.spans[0].style.fg, Some(theme.accent));
        assert_eq!(focused.spans[1].style.bg, Some(theme.accent));

        let unfocused = capsule(spans(), &theme, 10, false);
        assert_eq!(unfocused.spans[0].style.fg, Some(theme.selection));
        assert_eq!(unfocused.spans[1].style.bg, Some(theme.selection));
    }

    #[test]
    fn unselected_rows_line_up_with_the_capsule() {
        let theme = Theme::default();
        let plain_line = plain(vec![Span::raw("abc")]);
        let capsule_line = capsule(vec![Span::raw("abc")], &theme, 10, false);

        let plain_prefix = plain_line.spans[0].width();
        let cap_prefix = capsule_line.spans[0].width() + capsule_line.spans[1].width();
        assert_eq!(plain_prefix, cap_prefix);
        assert_eq!(plain_line.spans[1].content.as_ref(), "abc");
    }
}
