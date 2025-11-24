use std::io;
use ratatui::Terminal;
use ratzilla::DomBackend;

mod startup;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    startup::StartupScreen::start(&mut terminal);
    Ok(())
}
