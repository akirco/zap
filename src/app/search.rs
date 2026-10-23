//! Name filtering and content search driven by the footer input line.

use ratatui::text::Line;

use super::{App, Preview};
use crate::utils::file_name;
use std::path::Path;

impl App {
    /// Filter the current directory by the typed name.
    pub(crate) fn apply_search(&mut self) {
        let query = self.input_buffer.to_lowercase();
        self.entries = self
            .all_entries
            .iter()
            .filter(|p| file_name(p).to_lowercase().contains(&query))
            .cloned()
            .collect();
        self.current_state.select(if self.entries.is_empty() {
            None
        } else {
            Some(0)
        });
        self.update_preview();
    }

    /// Drop the search filter and restore the full listing.
    pub(crate) fn reset_search(&mut self) {
        self.entries = self.all_entries.clone();
        self.current_state.select(if self.entries.is_empty() {
            None
        } else {
            Some(0)
        });
        self.update_preview();
    }

    /// Search file contents and show the result in the preview pane.
    pub(crate) async fn execute_grep(&mut self) {
        let query = self.input_buffer.clone();
        if query.is_empty() {
            self.update_preview();
            return;
        }

        let target_path = self
            .selected_entry()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| self.current_dir.clone());

        let lines = match crate::utils::search_content(&query, &target_path).await {
            Ok(raw) if raw.is_empty() => {
                vec![Line::from(self.config.strings.grep_empty.clone())]
            }
            Ok(raw) => crate::ui::highlight_lines(&raw, &query, &self.theme),
            Err(e) => vec![Line::from(format!("执行失败: {e}"))],
        };
        let max_scroll = lines.len().saturating_sub(1) as u16;
        self.preview_content = Preview::Grep { lines, max_scroll };
        self.preview_scroll = 0;
    }
}
