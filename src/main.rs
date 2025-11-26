use std::io;
use ratatui::Terminal;
use ratzilla::DomBackend;
use wasm_bindgen::prelude::*;
use web_sys;

// Modules
mod startup;   // This is the startup file
mod home;
mod demo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Startup,
    Home,
    Demo
}

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

#[wasm_bindgen]
pub fn show_demo_screen() -> Result<(), JsValue> {
    let backend = DomBackend::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut terminal = Terminal::new(backend).map_err(|e| JsValue::from_str(&e.to_string()))?;
    terminal.clear().map_err(|e| JsValue::from_str(&e.to_string()))?;

    demo::DemoScreen::start(&mut terminal).map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
