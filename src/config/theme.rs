//! Named colour themes.
//!
//! A theme is a small semantic palette; every widget derives its style from
//! it (see `styles`). Built-in themes are listed in [`builtin`]; users can add
//! their own under `[[themes]]` in the config file and select one with
//! `theme = "name"`.

use std::collections::HashMap;
use std::sync::OnceLock;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Colour palette used by every widget.
///
/// Missing fields fall back to [`Theme::default`] (the terminal palette), so a
/// user theme only needs to declare the colours it changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub(crate) struct Theme {
    pub(crate) name: String,
    /// Footer background.
    pub(crate) bg: Color,
    /// Header and input-footer background.
    pub(crate) surface: Color,
    /// Normal text and file names.
    pub(crate) text: Color,
    /// Directories, icons and selected-row text, and the focused row background.
    pub(crate) accent: Color,
    /// Pane borders.
    pub(crate) border: Color,
    /// Unfocused selected-row background; also the text colour on the focused row.
    pub(crate) selection: Color,
    /// Executables.
    pub(crate) success: Color,
    /// Symlinks.
    pub(crate) link: Color,
    /// Icons.
    pub(crate) icon: Color,
    /// Matched text in content-search results.
    pub(crate) grep_match: Color,
}

impl Default for Theme {
    /// The terminal's own 16-colour palette.
    ///
    /// Slots are picked for the cwal/foot scheme, where the ANSI slot names do
    /// not follow the usual hues: 1 is a dark blue (surface/selection), 13 a
    /// light blue (accent), 12/3/11 the warm colours and 14 the icon blue.
    fn default() -> Self {
        Self {
            name: "terminal".to_string(),
            bg: Color::Reset,
            surface: Color::Indexed(1),
            text: Color::Indexed(15),
            accent: Color::Indexed(13),
            border: Color::Indexed(2),
            selection: Color::Indexed(1),
            success: Color::Indexed(12),
            link: Color::Indexed(3),
            icon: Color::Indexed(14),
            grep_match: Color::Indexed(11),
        }
    }
}

impl Theme {
    /// Look up a built-in theme by name.
    pub(crate) fn builtin(name: &str) -> Option<Theme> {
        builtin().get(name).cloned()
    }

    /// Names of all built-in themes.
    pub(crate) fn builtin_names() -> Vec<&'static str> {
        let mut names: Vec<&'static str> = builtin().keys().map(String::as_str).collect();
        names.sort_unstable();
        names
    }
}

fn builtin() -> &'static HashMap<String, Theme> {
    static THEMES: OnceLock<HashMap<String, Theme>> = OnceLock::new();
    THEMES.get_or_init(|| {
        [
            Theme::default(),
            Theme {
                name: "dracula".to_string(),
                bg: Color::Rgb(0x28, 0x2a, 0x36),
                surface: Color::Rgb(0x44, 0x47, 0x5a),
                text: Color::Rgb(0xf8, 0xf8, 0xf2),
                accent: Color::Rgb(0xbd, 0x93, 0xf9),
                border: Color::Rgb(0x62, 0x72, 0xa4),
                selection: Color::Rgb(0x44, 0x47, 0x5a),
                success: Color::Rgb(0x50, 0xfa, 0x7b),
                link: Color::Rgb(0xff, 0x79, 0xc6),
                icon: Color::Rgb(0x8b, 0xe9, 0xfd),
                grep_match: Color::Rgb(0xff, 0x55, 0x55),
            },
            Theme {
                name: "nord".to_string(),
                bg: Color::Rgb(0x2e, 0x34, 0x40),
                surface: Color::Rgb(0x3b, 0x42, 0x52),
                text: Color::Rgb(0xec, 0xef, 0xf4),
                accent: Color::Rgb(0x88, 0xc0, 0xd0),
                border: Color::Rgb(0x61, 0x6e, 0x88),
                selection: Color::Rgb(0x3b, 0x42, 0x52),
                success: Color::Rgb(0xa3, 0xbe, 0x8c),
                link: Color::Rgb(0xb4, 0x8e, 0xad),
                icon: Color::Rgb(0x88, 0xc0, 0xd0),
                grep_match: Color::Rgb(0xbf, 0x61, 0x6a),
            },
            Theme {
                name: "gruvbox".to_string(),
                bg: Color::Rgb(0x28, 0x28, 0x28),
                surface: Color::Rgb(0x3c, 0x38, 0x36),
                text: Color::Rgb(0xeb, 0xdb, 0xb2),
                accent: Color::Rgb(0xd6, 0x5d, 0x0e),
                border: Color::Rgb(0x92, 0x83, 0x74),
                selection: Color::Rgb(0x3c, 0x38, 0x36),
                success: Color::Rgb(0xb8, 0xbb, 0x26),
                link: Color::Rgb(0xd3, 0x86, 0x9b),
                icon: Color::Rgb(0xfa, 0xbd, 0x2f),
                grep_match: Color::Rgb(0xfb, 0x49, 0x34),
            },
            Theme {
                name: "catppuccin".to_string(),
                bg: Color::Rgb(0x1e, 0x1e, 0x2e),
                surface: Color::Rgb(0x31, 0x32, 0x44),
                text: Color::Rgb(0xcd, 0xd6, 0xf4),
                accent: Color::Rgb(0xcb, 0xa6, 0xf7),
                border: Color::Rgb(0x6c, 0x70, 0x86),
                selection: Color::Rgb(0x45, 0x47, 0x5a),
                success: Color::Rgb(0xa6, 0xe3, 0xa1),
                link: Color::Rgb(0xf5, 0xc2, 0xe7),
                icon: Color::Rgb(0x89, 0xdc, 0xeb),
                grep_match: Color::Rgb(0xf3, 0x8b, 0xa8),
            },
            Theme {
                name: "tokyo-night".to_string(),
                bg: Color::Rgb(0x1a, 0x1b, 0x26),
                surface: Color::Rgb(0x24, 0x28, 0x3b),
                text: Color::Rgb(0xc0, 0xca, 0xf5),
                accent: Color::Rgb(0x7a, 0xa2, 0xf7),
                border: Color::Rgb(0x56, 0x5f, 0x89),
                selection: Color::Rgb(0x2f, 0x35, 0x49),
                success: Color::Rgb(0x9e, 0xce, 0x6a),
                link: Color::Rgb(0xbb, 0x9a, 0xf7),
                icon: Color::Rgb(0x7d, 0xcf, 0xff),
                grep_match: Color::Rgb(0xf7, 0x76, 0x8e),
            },
            Theme {
                name: "solarized-light".to_string(),
                bg: Color::Rgb(0xfd, 0xf6, 0xe3),
                surface: Color::Rgb(0xee, 0xe8, 0xd5),
                text: Color::Rgb(0x65, 0x7b, 0x83),
                accent: Color::Rgb(0x26, 0x8b, 0xd2),
                border: Color::Rgb(0x93, 0xa1, 0xa1),
                selection: Color::Rgb(0xee, 0xe8, 0xd5),
                success: Color::Rgb(0x85, 0x99, 0x00),
                link: Color::Rgb(0xd3, 0x36, 0x82),
                icon: Color::Rgb(0x2a, 0xa1, 0x98),
                grep_match: Color::Rgb(0xdc, 0x32, 0x2f),
            },
            Theme {
                name: "github-light".to_string(),
                bg: Color::Rgb(0xff, 0xff, 0xff),
                surface: Color::Rgb(0xf6, 0xf8, 0xfa),
                text: Color::Rgb(0x24, 0x29, 0x2f),
                accent: Color::Rgb(0x09, 0x69, 0xda),
                border: Color::Rgb(0xd0, 0xd7, 0xde),
                selection: Color::Rgb(0xf6, 0xf8, 0xfa),
                success: Color::Rgb(0x1a, 0x7f, 0x37),
                link: Color::Rgb(0x82, 0x50, 0xdf),
                icon: Color::Rgb(0x57, 0x60, 0x6a),
                grep_match: Color::Rgb(0xcf, 0x22, 0x2e),
            },
        ]
        .into_iter()
        .map(|theme| (theme.name.clone(), theme))
        .collect()
    })
}
