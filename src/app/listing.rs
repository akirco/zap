//! Directory listing state: which directory is loaded, which entry is selected,
//! and how the parent listing follows the cursor.

use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use super::{App, Focus, InputMode};
use crate::utils::{load_dir, selection_index};

impl App {
    /// Entry under the cursor in the current listing.
    pub(crate) fn selected_entry(&self) -> Option<&Path> {
        self.current_state
            .selected()
            .and_then(|idx| self.entries.get(idx))
            .map(PathBuf::as_path)
    }

    /// Confirm the entry under the cursor: remember it for the caller and quit.
    pub(crate) fn confirm_selection(&mut self) {
        if let Some(path) = self.selected_entry().map(Path::to_path_buf) {
            self.selected_path = Some(path);
        }
        self.quit();
    }

    /// Load `path` as the current directory.
    ///
    /// `select` places the cursor on that entry of the new listing, which keeps
    /// the cursor on the directory we came from when going up a level. Opening
    /// a file selects the file itself.
    pub(crate) fn change_dir(&mut self, path: &Path, select: Option<&Path>) {
        let path = canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let target_dir = if path.is_file() {
            path.parent().unwrap_or(&path).to_path_buf()
        } else {
            path.clone()
        };

        self.current_dir = target_dir;
        self.all_entries = load_dir(&self.current_dir);
        self.entries = self.all_entries.clone();

        let select = select.or(path.is_file().then_some(path.as_path()));
        let idx = selection_index(&self.entries, select);
        self.current_state.select(if self.entries.is_empty() {
            None
        } else {
            Some(idx)
        });

        self.update_parent_state();

        self.focus = Focus::Current;
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.update_preview();
    }

    fn update_parent_state(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.parent_entries = load_dir(parent);
            let p_idx = self
                .parent_entries
                .iter()
                .position(|p| p == &self.current_dir)
                .unwrap_or(0);
            self.parent_state.select(Some(p_idx));
        } else {
            self.parent_entries.clear();
            self.parent_state.select(None);
        }
    }

    /// Follow the cursor in the parent pane into the current pane.
    pub(crate) fn sync_from_parent_move(&mut self) {
        if let Some(idx) = self.parent_state.selected()
            && let Some(path) = self.parent_entries.get(idx).cloned()
        {
            if path.is_dir() {
                self.current_dir = path;
                self.all_entries = load_dir(&self.current_dir);
                self.entries = self.all_entries.clone();
                self.current_state.select(if self.entries.is_empty() {
                    None
                } else {
                    Some(0)
                });
            } else {
                self.current_dir = path;
                self.all_entries.clear();
                self.entries.clear();
                self.current_state.select(None);
            }
            self.update_preview();
        }
    }
}
