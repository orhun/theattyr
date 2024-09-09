pub mod animation;
pub mod app;
pub mod event;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let event_handler = event::EventHandler::new(250);
    let result = app::App::new(event_handler).run(terminal);
    ratatui::restore();
    result
}
