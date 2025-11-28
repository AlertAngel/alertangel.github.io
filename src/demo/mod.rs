use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
    Terminal,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use std::io;
use std::cell::RefCell;
use crate::buttons::ArrowKeys;

// Global state to persist across re-renders
thread_local! {
    static LAST_KEY: RefCell<Option<ArrowKeys>> = RefCell::new(None);
    static KEY_HISTORY: RefCell<Vec<ArrowKeys>> = RefCell::new(Vec::new());
    static LISTENER_ATTACHED: RefCell<bool> = RefCell::new(false);
}

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

            // Demo content with key display
            let last_key_display = LAST_KEY.with(|k| {
                if let Some(key) = *k.borrow() {
                    format!("{} ({})", key.as_symbol(), key.as_name())
                } else {
                    "None".to_string()
                }
            });

            let history_display = KEY_HISTORY.with(|h| {
                let history = h.borrow();
                if history.is_empty() {
                    "No keys pressed yet".to_string()
                } else {
                    history.iter()
                        .rev()
                        .take(10)
                        .map(|k| k.as_symbol())
                        .collect::<Vec<_>>()
                        .join(" ")
                }
            });

            let content_lines = vec![
                Line::from(vec![
                    Span::styled("🎮 Demo Mode", Style::default().fg(Color::Cyan).bold())
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Press Arrow Keys to interact", Style::default().fg(Color::White))
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Last Key Pressed: ", Style::default().fg(Color::Gray)),
                    Span::styled(last_key_display, Style::default().fg(Color::Green).bold())
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Key History: ", Style::default().fg(Color::Gray)),
                ]),
                Line::from(vec![
                    Span::styled(history_display, Style::default().fg(Color::Magenta))
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Arrow Key Controls:", Style::default().fg(Color::Yellow))
                ]),
                Line::from(vec![
                    Span::styled("  ↑ Up  |  ↓ Down  |  ← Left  |  → Right", Style::default().fg(Color::White))
                ]),
            ];

            let content = Paragraph::new(content_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                )
                .alignment(Alignment::Center);
            
            frame.render_widget(content, layout[1]);

            // Footer
            let footer = Paragraph::new("Press ESC to go back to home")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);
            frame.render_widget(footer, layout[2]);
        })?;

        // Only set up key handler once
        LISTENER_ATTACHED.with(|attached| {
            if !*attached.borrow() {
                Self::setup_key_handler();
                *attached.borrow_mut() = true;
                console::log_1(&"Event listener attached".into());
            }
        });

        Ok(())
    }

    fn setup_key_handler() {
        use web_sys::window;
        
        let window = window().expect("no global window");
        let document = window.document().expect("no document");
        
        console::log_1(&"Arrow key handler setup for demo screen".into());
        
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key();
            
            console::log_1(&format!("Key pressed: {}", key).into());
            
            // Check for ESC key
            if key == "Escape" {
                console::log_1(&"ESC pressed, going back to home".into());
                event.prevent_default();
                
                // Reset listener flag so home can set up its own
                LISTENER_ATTACHED.with(|attached| {
                    *attached.borrow_mut() = false;
                });
                
                let _ = crate::show_home_from_demo();
                return;
            }
            
            // Check for arrow keys
            if let Some(arrow_key) = ArrowKeys::from_key_string(&key) {
                console::log_1(&format!("Arrow key detected: {:?}", arrow_key).into());
                event.prevent_default();
                
                // Update global state
                LAST_KEY.with(|k| {
                    *k.borrow_mut() = Some(arrow_key);
                });
                
                KEY_HISTORY.with(|h| {
                    h.borrow_mut().push(arrow_key);
                });
                
                // Re-render the demo screen with updated state
                let _ = crate::show_demo_screen();
            }
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");
        
        closure.forget();
    }
}
