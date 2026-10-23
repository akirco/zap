//! Fixed strings shown in the UI.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct StringsConfig {
    /// Prefix prepended to the current path in the header.
    pub(crate) path_prefix: String,
    /// Footer text when nothing is selected.
    pub(crate) no_selection: String,
    /// Preview text when a content search found nothing.
    pub(crate) grep_empty: String,
    /// Suffix appended to the grep input in the footer.
    pub(crate) grep_suffix: String,
}

impl Default for StringsConfig {
    fn default() -> Self {
        Self {
            path_prefix: String::new(),
            no_selection: "No item selected".to_string(),
            grep_empty: "No matches found".to_string(),
            grep_suffix: String::new(),
        }
    }
}
