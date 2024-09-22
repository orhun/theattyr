use theattyr::*;

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let event_handler = event::EventHandler::new(args.tick_rate);
    let result = app::App::new(event_handler, args).run(terminal);
    ratatui::restore();
    result
}
