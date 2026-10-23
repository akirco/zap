mod app;
mod cli;
mod config;
mod event;
mod layout;
mod ui;
mod utils;

use eyre::Result;

use crate::app::App;
use crate::cli::parse_cli;
use crate::config::Config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = parse_cli();
    let config = Config::load_or_default(cli.config.as_deref());
    let mut app = App::new(cli.path, config);

    let mut terminal = ratatui::try_init()?;
    let result = app.run(&mut terminal).await;
    ratatui::restore();
    result?;

    if let Some(selected) = app.selected_path {
        println!("{}", selected.display());
    }
    Ok(())
}
