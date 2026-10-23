//! Application state and lifecycle: the event loop plus the behaviour modules
//! (input, listing, navigation, preview, search) under `app/`.

mod input;
mod listing;
mod navigation;
mod preview;
mod search;

use std::path::PathBuf;

use eyre::Result;
use ratatui::{DefaultTerminal, text::Line, widgets::ListState};

use crate::config::{Config, Theme};
use crate::event::{Event, EventHandler};
use crate::utils::PreviewText;

#[derive(Debug, PartialEq)]
pub(crate) enum Focus {
    Parent,
    Current,
    Preview,
}

pub(crate) enum Preview {
    Dir(Vec<PathBuf>),
    File(PreviewText),
    Grep {
        lines: Vec<Line<'static>>,
        max_scroll: u16,
    },
    Empty,
}

#[derive(PartialEq)]
pub(crate) enum InputMode {
    Normal,
    Search,
    Command,
    Grep,
}

pub(crate) struct App {
    pub(crate) config: Config,
    pub(crate) theme: Theme,
    pub(crate) current_dir: PathBuf,
    pub(crate) parent_entries: Vec<PathBuf>,
    pub(crate) parent_state: ListState,
    pub(crate) all_entries: Vec<PathBuf>,
    pub(crate) entries: Vec<PathBuf>,
    pub(crate) current_state: ListState,
    pub(crate) preview_content: Preview,
    pub(crate) preview_state: ListState,
    pub(crate) preview_scroll: u16,
    pub(crate) focus: Focus,
    pub(crate) input_mode: InputMode,
    pub(crate) input_buffer: String,
    pub(crate) should_quit: bool,
    pub(crate) selected_path: Option<PathBuf>,
    pub(crate) events: EventHandler,
}

impl App {
    pub(crate) fn new(start_path: PathBuf, config: Config) -> App {
        let theme = config.resolve_theme();
        let mut app = App {
            config,
            theme,
            current_dir: PathBuf::new(),
            parent_entries: Vec::new(),
            parent_state: ListState::default(),
            all_entries: Vec::new(),
            entries: Vec::new(),
            current_state: ListState::default(),
            preview_content: Preview::Empty,
            preview_state: ListState::default(),
            preview_scroll: 0,
            focus: Focus::Current,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            should_quit: false,
            selected_path: None,
            events: EventHandler::new(),
        };
        app.change_dir(&start_path, None);
        app
    }

    /// Run the application's main loop. Redraws only when an event arrives.
    pub(crate) async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|f| crate::ui::render(f, self))?;
            match self.events.next().await? {
                Event::Key(key) => self.handle_key_event(key, terminal).await?,
                // Ratatui autoresizes on the next draw, so nothing to do here.
                Event::Resize => {}
            }
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    pub(crate) fn quit(&mut self) {
        self.should_quit = true;
    }
}
