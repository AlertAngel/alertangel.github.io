use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use std::io;

pub struct DemoScreen;

impl DemoScreen {
    pub fn start<B>(terminal: &mut Terminal<B>) -> io::Result<()>
    where
        B: ratatui::backend::Backend,
    {
        terminal.draw(|frame| {
            let area = frame.area();

            let layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

            // Title
            let title = Paragraph::new("AlertAngel - Demo")
                .style(Style::default().fg(Color::LightYellow).bold())
                .alignment(Alignment::Center);
            frame.render_widget(title, layout[0]);

            // Demo content placeholder
            let content = Paragraph::new(vec![
                "🎮 Demo Mode".into(),
                "".into(),
                "This is where the interactive demo will be displayed.".into(),
                "".into(),
                "Features coming soon:".into(),
                "• Real-time sensor data visualization".into(),
                "• Alert simulation".into(),
                "• UI workflow demonstration".into(),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
            
            frame.render_widget(content, layout[1]);

            // Footer
            let footer = Paragraph::new("Press ESC to go back to home")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);
            frame.render_widget(footer, layout[2]);
        })?;

        // Set up ESC key handler
        Self::setup_key_handler();

        Ok(())
    }

    fn setup_key_handler() {
        use web_sys::window;
        
        let window = window().expect("no global window");
        let document = window.document().expect("no document");
        
        console::log_1(&"ESC key handler setup for demo screen".into());
        
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key();
            
            console::log_1(&format!("Key pressed: {}", key).into());
            
            // Check for ESC key
            if key == "Escape" {
                console::log_1(&"ESC pressed, going back to home".into());
                event.prevent_default();
                
                // Navigate back to home screen
                let _ = crate::show_home_from_demo();
            }
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");
        
        closure.forget();
    }
}
