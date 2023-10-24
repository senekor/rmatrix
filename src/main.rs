mod app;
mod event;
mod matrix;
mod tui;
mod update;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let term_size = terminal.size()?;

    let events = EventHandler::new();
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Create an application.
    let mut app = App::new(term_size.width, term_size.height);

    // Start the main loop.
    while !app.should_quit() {
        // Render the user interface.
        tui.draw(|frame| frame.render_widget(app.get_matrix(), frame.size()))?;
        // Handle events.
        match events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(w, h) => app.resize(w, h),
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
