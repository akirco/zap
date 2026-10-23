use ratatui::widgets::{Block, BorderType, Borders};

use super::styles::border_style;
use crate::config::{PaneBorderConfig, Theme};

pub(crate) fn pane_block<'a>(
    title: &'a str,
    theme: &Theme,
    border_cfg: &PaneBorderConfig,
) -> Block<'a> {
    let mut block = Block::default()
        .borders(parse_borders(&border_cfg.sides))
        .border_type(parse_border_type(&border_cfg.kind))
        .border_style(border_style(theme));

    if !title.trim().is_empty() {
        block = block.title(title);
    }

    block
}

fn parse_borders(sides: &str) -> Borders {
    match sides.to_lowercase().as_str() {
        "none" => Borders::NONE,
        "top" => Borders::TOP,
        "bottom" => Borders::BOTTOM,
        "left" => Borders::LEFT,
        "right" => Borders::RIGHT,
        "horizontal" | "top_bottom" => Borders::TOP | Borders::BOTTOM,
        "vertical" | "left_right" => Borders::LEFT | Borders::RIGHT,
        _ => Borders::ALL,
    }
}

fn parse_border_type(kind: &str) -> BorderType {
    match kind.to_lowercase().as_str() {
        "rounded" => BorderType::Rounded,
        "double" => BorderType::Double,
        "thick" => BorderType::Thick,
        _ => BorderType::Plain,
    }
}
