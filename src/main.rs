use std::io;
use ratatui::Terminal;
use ratzilla::DomBackend;
use wasm_bindgen::prelude::*;
use web_sys;

// Modules
mod startup;   // This is the startup file
mod home;

fn main() -> io::Result<()> {
    show_startup()?;

    let window = web_sys::window().expect("No global window exists");

    let closure = Closure::once(move || {
        let _ = show_home();
    });

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            2000
        )
            .expect("Should Register TImeout");
    closure.forget();

    Ok(())
}

fn show_startup() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    startup::StartupScreen::start(&mut terminal);

    Ok(())
}

fn show_home() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    home::HomeScreen::start(&mut terminal)?;
    Ok(())
}
