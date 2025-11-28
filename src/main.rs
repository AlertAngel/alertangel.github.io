use std::io;
use ratatui::Terminal;
use ratzilla::DomBackend;
use wasm_bindgen::prelude::*;
use web_sys;

mod startup;
mod home;
mod demo;
mod buttons;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Startup,
    Home,
    Demo,
}

fn main() -> io::Result<()> {
    show_startup()?;
    
    let window = web_sys::window().expect("no global window exists");
    
    let closure = Closure::once(move || {
        let _ = show_home();
    });
    
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            2000
        )
        .expect("should register timeout");
    
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
    terminal.clear()?;
    home::HomeScreen::start(&mut terminal)?;
    Ok(())
}

#[wasm_bindgen]
pub fn show_demo_screen() -> Result<(), JsValue> {
    web_sys::console::log_1(&"show_demo_screen called".into());
    
    let backend = DomBackend::new().map_err(|e| {
        let msg = format!("Backend error: {}", e);
        web_sys::console::log_1(&msg.clone().into());
        JsValue::from_str(&msg)
    })?;
    
    web_sys::console::log_1(&"Backend created".into());
    
    let mut terminal = Terminal::new(backend).map_err(|e| {
        let msg = format!("Terminal error: {}", e);
        web_sys::console::log_1(&msg.clone().into());
        JsValue::from_str(&msg)
    })?;
    
    web_sys::console::log_1(&"Terminal created".into());
    
    terminal.clear().map_err(|e| {
        let msg = format!("Clear error: {}", e);
        web_sys::console::log_1(&msg.clone().into());
        JsValue::from_str(&msg)
    })?;
    
    web_sys::console::log_1(&"Terminal cleared".into());
    
    demo::DemoScreen::start(&mut terminal).map_err(|e| {
        let msg = format!("Demo screen error: {}", e);
        web_sys::console::log_1(&msg.clone().into());
        JsValue::from_str(&msg)
    })?;
    
    web_sys::console::log_1(&"Demo screen rendered".into());
    
    Ok(())
}

#[wasm_bindgen]
pub fn show_home_from_demo() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Returning to home screen".into());
    
    let backend = DomBackend::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut terminal = Terminal::new(backend).map_err(|e| JsValue::from_str(&e.to_string()))?;
    terminal.clear().map_err(|e| JsValue::from_str(&e.to_string()))?;
    home::HomeScreen::start(&mut terminal).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(())
}
