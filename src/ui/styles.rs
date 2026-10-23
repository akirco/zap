//! Translate a [`Theme`] palette into ratatui styles.

use ratatui::style::{Modifier, Style};

use crate::{config::Theme, utils::FileType};

pub(crate) fn entry_style(theme: &Theme, file_type: &FileType) -> Style {
    let color = match file_type {
        FileType::Directory => theme.accent,
        FileType::Executable => theme.success,
        FileType::Symlink => theme.link,
        FileType::File => theme.text,
    };

    Style::default().fg(color)
}

pub(crate) fn icon_style(theme: &Theme) -> Style {
    Style::default().fg(theme.icon)
}

pub(crate) fn header_style(theme: &Theme) -> Style {
    Style::default()
        .bg(theme.surface)
        .fg(theme.text)
        .add_modifier(Modifier::BOLD)
}

pub(crate) fn footer_normal_style(theme: &Theme) -> Style {
    Style::default().bg(theme.bg).fg(theme.text)
}

pub(crate) fn footer_input_style(theme: &Theme) -> Style {
    Style::default().bg(theme.surface).fg(theme.accent)
}

pub(crate) fn border_style(theme: &Theme) -> Style {
    Style::default().fg(theme.border)
}

pub(crate) fn selection_style(theme: &Theme) -> Style {
    Style::default()
        .bg(theme.selection)
        .fg(theme.accent)
        .add_modifier(Modifier::BOLD)
}

/// Selected row in the focused pane: inverted accent capsule.
pub(crate) fn focus_selection_style(theme: &Theme) -> Style {
    Style::default()
        .bg(theme.accent)
        .fg(theme.selection)
        .add_modifier(Modifier::BOLD)
}

pub(crate) fn grep_match_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.grep_match)
        .add_modifier(Modifier::BOLD)
}
