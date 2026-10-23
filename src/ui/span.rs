//! Spans making up one list row: the type icon and the styled name.

use std::path::Path;

use ratatui::text::Span;

use super::styles::{entry_style, icon_style, selection_style};
use crate::{
    config::{Config, Theme},
    utils::{FileType, file_name, file_type, icon_for},
};

pub(super) fn icon_span<'a>(
    path: &Path,
    config: &'a Config,
    theme: &Theme,
    is_active_dir: bool,
) -> Span<'a> {
    let file_type = file_type(path);
    let icon = icon_for(config, &file_type, path);
    let style = if is_active_dir {
        selection_style(theme)
    } else {
        icon_style(theme)
    };
    Span::styled(icon, style)
}

pub(super) fn label_span<'a>(path: &Path, theme: &Theme, is_active_dir: bool) -> Span<'a> {
    let file_type = file_type(path);
    let name = file_name(path);
    let suffix = if is_active_dir || !matches!(file_type, FileType::Directory) {
        ""
    } else {
        "/"
    };
    let style = if is_active_dir {
        selection_style(theme)
    } else {
        entry_style(theme, &file_type)
    };
    Span::styled(format!(" {}{}", name, suffix), style)
}
