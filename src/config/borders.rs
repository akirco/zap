//! Per-pane border configuration.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct BordersConfig {
    pub(crate) parent: PaneBorderConfig,
    pub(crate) current: PaneBorderConfig,
    pub(crate) preview: PaneBorderConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct PaneBorderConfig {
    /// Which sides are drawn: `none`, `all`, `top`, `bottom`, `left`, `right`,
    /// `horizontal` or `vertical`.
    pub(crate) sides: String,
    /// Border glyph set: `plain`, `rounded`, `double` or `thick`.
    pub(crate) kind: String,
}

impl Default for BordersConfig {
    fn default() -> Self {
        Self {
            parent: PaneBorderConfig {
                sides: "none".to_string(),
                kind: String::new(),
            },
            current: PaneBorderConfig {
                sides: "vertical".to_string(),
                kind: String::new(),
            },
            preview: PaneBorderConfig {
                sides: "none".to_string(),
                kind: String::new(),
            },
        }
    }
}
