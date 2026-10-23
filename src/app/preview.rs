//! Preview pane state: what the selected entry shows, and how it scrolls.

use std::path::Path;

use super::{App, Preview};
use crate::utils::{load_dir, read_file_preview};

impl App {
    /// Refresh the preview for the current selection.
    pub(crate) fn update_preview(&mut self) {
        self.preview_scroll = 0;
        self.preview_state.select(Some(0));

        if self.entries.is_empty() && self.current_dir.is_file() {
            self.preview_content = Preview::File(read_file_preview(&self.current_dir));
            return;
        }

        if let Some(path) = self.selected_entry().map(Path::to_path_buf) {
            if path.is_dir() {
                let children = load_dir(&path);
                self.preview_state
                    .select(if children.is_empty() { None } else { Some(0) });
                self.preview_content = Preview::Dir(children);
            } else if path.is_file() {
                self.preview_content = Preview::File(read_file_preview(&path));
            } else {
                self.preview_content = Preview::Empty;
            }
        } else {
            self.preview_content = Preview::Empty;
        }
    }

    /// Scroll the file/grep preview by one line, clamped to `max_scroll`.
    pub(crate) fn scroll_preview(&mut self, max_scroll: u16, delta: i32) {
        if delta > 0 {
            self.preview_scroll = self.preview_scroll.saturating_add(1).min(max_scroll);
        } else {
            self.preview_scroll = self.preview_scroll.saturating_sub(1);
        }
    }
}
