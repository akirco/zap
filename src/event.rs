use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use eyre::{OptionExt, Result};
use futures_util::StreamExt;
use tokio::sync::mpsc;

/// Representation of all possible events.
#[derive(Clone, Debug)]
pub(crate) enum Event {
    /// Crossterm key events.
    Key(KeyEvent),
    /// Terminal resize. The dimensions are not needed: ratatui autoresizes on draw.
    Resize,
}

/// Terminal event handler.
pub(crate) struct EventHandler {
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    /// Constructs a new [`EventHandler`] and spawns a task that emits events.
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        tokio::spawn(EventTask::new(sender).run());
        Self { receiver }
    }

    /// Receives the next event.
    ///
    /// # Errors
    ///
    /// Returns an error if the event task's sender is dropped, which can only
    /// happen if the task itself died.
    pub(crate) async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
    }
}

/// Forwards terminal events to the event channel.
struct EventTask {
    sender: mpsc::UnboundedSender<Event>,
}

impl EventTask {
    fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    async fn run(self) {
        let mut reader = crossterm::event::EventStream::new();
        loop {
            match reader.next().await {
                Some(Ok(CrosstermEvent::Key(key))) => self.send(Event::Key(key)),
                Some(Ok(CrosstermEvent::Resize(_, _))) => self.send(Event::Resize),
                // Other terminal events (mouse, focus, paste) are ignored.
                Some(Ok(_)) | Some(Err(_)) => continue,
                // Input stream closed: drop the sender so the app exits.
                None => break,
            }
        }
    }

    fn send(&self, event: Event) {
        // Ignore the error: a dropped receiver means the app is shutting down.
        let _ = self.sender.send(event);
    }
}
