//! Highlight query matches inside a preview line.

use ratatui::text::{Line, Span};

use super::styles::grep_match_style;
use crate::config::Theme;

/// Maximum number of result lines shown in the preview.
const MAX_LINES: usize = 1000;

fn highlight_line(text: &str, query: &str, theme: &Theme) -> Line<'static> {
    let mut spans = Vec::new();
    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();
    let mut start = 0;

    while let Some(idx) = lower_text[start..].find(&lower_query) {
        let match_start = start + idx;
        let match_end = match_start + query.len();

        if match_start > start {
            spans.push(Span::raw(text[start..match_start].to_string()));
        }

        spans.push(Span::styled(
            text[match_start..match_end].to_string(),
            grep_match_style(theme),
        ));

        start = match_end;
    }

    if start < text.len() {
        spans.push(Span::raw(text[start..].to_string()));
    }

    Line::from(spans)
}

/// Highlight raw search output, capped for the preview pane.
pub(crate) fn highlight_lines(lines: &[String], query: &str, theme: &Theme) -> Vec<Line<'static>> {
    lines
        .iter()
        .take(MAX_LINES)
        .map(|line| highlight_line(line, query, theme))
        .collect()
}
