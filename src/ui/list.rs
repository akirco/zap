//! Parent and current listings, plus the shared list renderer.

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem},
};

use super::{block::pane_block, item};
use crate::app::Preview;
use crate::app::{App, Focus};

pub(crate) fn render_parent(f: &mut Frame, area: Rect, app: &mut App) {
    render_list(f, area, app, Focus::Parent);
}

pub(crate) fn render_current(f: &mut Frame, area: Rect, app: &mut App) {
    render_list(f, area, app, Focus::Current);
}

/// Shared renderer for the parent listing, current listing and directory preview.
///
/// `pane` selects the data source; `Focus::Preview` renders the directory being
/// previewed.
pub(super) fn render_list(f: &mut Frame, area: Rect, app: &mut App, pane: Focus) {
    let App {
        config,
        theme,
        focus,
        current_dir,
        parent_entries,
        entries,
        preview_content,
        parent_state,
        current_state,
        preview_state,
        ..
    } = app;

    let (list_entries, state, title, border, active_dir) = match pane {
        Focus::Parent => (
            &*parent_entries,
            parent_state,
            &config.titles.parent,
            &config.borders.parent,
            Some(current_dir.as_path()),
        ),
        Focus::Current => (
            &*entries,
            current_state,
            &config.titles.current,
            &config.borders.current,
            None,
        ),
        Focus::Preview => {
            let Preview::Dir(children) = &*preview_content else {
                return;
            };
            (
                children,
                preview_state,
                &config.titles.preview,
                &config.borders.preview,
                None,
            )
        }
    };

    let is_focused = *focus == pane;
    let block = pane_block(title, theme, border);
    let width = block.inner(area).width as usize;
    let selected = state.selected().filter(|&idx| idx < list_entries.len());
    let items: Vec<ListItem> = list_entries
        .iter()
        .enumerate()
        .map(|(idx, path)| {
            let is_active = active_dir.is_some_and(|dir| dir == path);
            item::render(
                path,
                config,
                theme,
                is_active,
                selected == Some(idx),
                is_focused,
                width,
            )
        })
        .collect();

    let list = List::new(items).block(block);

    f.render_stateful_widget(list, area, state);
}
