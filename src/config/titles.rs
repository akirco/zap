//! Pane titles.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub(crate) struct TitlesConfig {
    pub(crate) parent: String,
    pub(crate) current: String,
    pub(crate) preview: String,
    pub(crate) grep: String,
}
