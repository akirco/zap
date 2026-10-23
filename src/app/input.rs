use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use eyre::Result;
use ratatui::DefaultTerminal;

use super::{App, Focus, InputMode};
use crate::event::Event;

impl App {
    /// Handles a key event and updates the state of [`App`].
    pub(crate) async fn handle_key_event(
        &mut self,
        key: KeyEvent,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match self.input_mode {
            InputMode::Normal => self.handle_normal_key(key),
            InputMode::Search => self.handle_search_key(key),
            InputMode::Command => self.handle_command_key(key, terminal).await?,
            InputMode::Grep => self.handle_grep_key(key).await,
        }

        Ok(())
    }

    /// Navigation and mode switching.
    fn handle_normal_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('b') {
            self.config.show_parent = !self.config.show_parent;
            if !self.config.show_parent && self.focus == Focus::Parent {
                self.focus = Focus::Current;
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Down | KeyCode::Char('j') => self.cursor_down(),
            KeyCode::Up | KeyCode::Char('k') => self.cursor_up(),
            KeyCode::Left | KeyCode::Char('h') => {
                if self.focus == Focus::Current {
                    self.go_up();
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.focus == Focus::Current {
                    self.enter_selected();
                }
            }
            KeyCode::Char('r') => self.config.reverse_columns = !self.config.reverse_columns,
            KeyCode::Char('s') => self.config.reverse_rows = !self.config.reverse_rows,
            KeyCode::Tab => self.toggle_focus(),
            KeyCode::Char('/') => {
                self.input_mode = InputMode::Search;
                self.input_buffer.clear();
            }
            KeyCode::Char(':') => {
                self.input_mode = InputMode::Command;
                self.input_buffer.clear();
            }
            KeyCode::Char('?') => {
                self.input_mode = InputMode::Grep;
                self.input_buffer.clear();
            }
            KeyCode::Enter => self.confirm_selection(),
            _ => {}
        }
    }

    /// Filter the current directory by name.
    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.reset_search();
            }
            KeyCode::Enter => self.input_mode = InputMode::Normal,
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
                self.apply_search();
            }
            KeyCode::Backspace => {
                if self.input_buffer.is_empty() {
                    self.input_mode = InputMode::Normal;
                    self.reset_search();
                } else {
                    self.input_buffer.pop();
                    self.apply_search();
                }
            }
            _ => {}
        }
    }

    /// Run a shell command with the TUI suspended.
    async fn handle_command_key(
        &mut self,
        key: KeyEvent,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.input_mode = InputMode::Normal,
            KeyCode::Enter => {
                let cmd = self.input_buffer.clone();
                self.input_mode = InputMode::Normal;
                self.input_buffer.clear();
                self.run_shell_command(&cmd, terminal).await?;
            }
            KeyCode::Char(c) => self.input_buffer.push(c),
            KeyCode::Backspace => {
                if self.input_buffer.is_empty() {
                    self.input_mode = InputMode::Normal;
                } else {
                    self.input_buffer.pop();
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Search file contents and show the result in the preview pane.
    async fn handle_grep_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.update_preview();
            }
            KeyCode::Enter => {
                self.execute_grep().await;
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => self.input_buffer.push(c),
            KeyCode::Backspace => {
                if self.input_buffer.is_empty() {
                    self.input_mode = InputMode::Normal;
                    self.update_preview();
                } else {
                    self.input_buffer.pop();
                }
            }
            _ => {}
        }
    }

    /// Suspends the TUI, runs `cmd` in the user's shell, then resumes the TUI.
    async fn run_shell_command(&mut self, cmd: &str, terminal: &mut DefaultTerminal) -> Result<()> {
        ratatui::restore();

        #[cfg(target_os = "windows")]
        let (shell, arg) = ("cmd", "/C");
        #[cfg(not(target_os = "windows"))]
        let (shell, arg) = ("sh", "-c");

        if let Err(e) = tokio::process::Command::new(shell)
            .arg(arg)
            .arg(cmd)
            .status()
            .await
        {
            println!("excute command failed: {}", e);
        }

        println!("\nPress Enter to return to the file explorer...");
        // Read the Enter from the same event queue as the main loop: the event reader
        // task owns stdin, so a blocking read here would race it for the keypress.
        while !matches!(
            self.events.next().await?,
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            })
        ) {}

        enable_raw_mode()?;
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
        // A fresh Terminal starts with empty buffers, so the next draw repaints
        // everything. Terminal::clear() would query the cursor position instead,
        // and the event reader task swallows that response.
        *terminal =
            ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;

        Ok(())
    }
}
