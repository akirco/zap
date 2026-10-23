//! Cursor movement and directory changes driven by the navigation keys.

use super::Preview;
use super::{App, Focus};
use crate::utils::compute_new_index;
use std::path::Path;

impl App {
    pub(crate) fn cursor_down(&mut self) {
        self.navigate_list(1);
    }

    pub(crate) fn cursor_up(&mut self) {
        self.navigate_list(-1);
    }

    fn navigate_list(&mut self, delta: i32) {
        let cycle = self.config.list_cycle;

        match self.focus {
            Focus::Parent => {
                if self.parent_entries.is_empty() {
                    return;
                }
                let len = self.parent_entries.len();
                let new_index = compute_new_index(self.parent_state.selected(), len, delta, cycle);
                self.parent_state.select(Some(new_index));
                self.sync_from_parent_move();
            }
            Focus::Current => {
                if self.entries.is_empty() {
                    return;
                }
                let len = self.entries.len();
                let new_index = compute_new_index(self.current_state.selected(), len, delta, cycle);
                self.current_state.select(Some(new_index));
                self.update_preview();
            }
            Focus::Preview => match &self.preview_content {
                Preview::Dir(children) => {
                    if children.is_empty() {
                        return;
                    }
                    let len = children.len();
                    let new_index =
                        compute_new_index(self.preview_state.selected(), len, delta, cycle);
                    self.preview_state.select(Some(new_index));
                }
                Preview::File(preview) => self.scroll_preview(preview.max_scroll, delta),
                Preview::Grep { max_scroll, .. } => self.scroll_preview(*max_scroll, delta),
                Preview::Empty => {}
            },
        }
    }

    pub(crate) fn go_up(&mut self) {
        if let Some(parent) = self.current_dir.parent().map(|p| p.to_path_buf()) {
            let current = self.current_dir.clone();
            self.change_dir(&parent, Some(&current));
        }
    }

    pub(crate) fn enter_selected(&mut self) {
        let path = self.selected_entry().map(Path::to_path_buf);
        if let Some(path) = path
            && path.is_dir()
        {
            self.change_dir(&path, None);
        }
    }

    pub(crate) fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Preview,
            Focus::Preview => {
                if self.config.show_parent {
                    Focus::Parent
                } else {
                    Focus::Current
                }
            }
            Focus::Parent => Focus::Current,
        };
    }
}
