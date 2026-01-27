use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;
use pulldown_cmark::{Parser, Event, Tag, TagEnd}; // Added TagEnd here
use wasm_bindgen::prelude::*;
use web_sys::console;

// Prepare your markdown content here. 
const SAMPLE_BLOG: &str = r#"
# Welcome to My Rust Blog

This blog is rendered entirely in a terminal emulator inside your browser!

## Why Ratzilla?
Because standard HTML is boring. I wanted to build something that feels like a retro CLI but runs on the modern web.

### Features
* **Fast**: It's Rust + WASM.
* **Cool**: It looks like a hacker's dashboard.
* **Markdown**: This text is parsing standard Markdown syntax.

Check back later for more updates on my embedded systems projects!
"#;

pub struct BlogScreen;

impl BlogScreen {
    pub fn start<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();

            // Split screen into Header (Back button) and Content
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header/Nav
                    Constraint::Min(0),    // Blog Content
                ])
                .split(area);

            // 1. Navigation Bar
            let nav = Paragraph::new("← Back to Home (Click here)")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Left)
                .block(Block::default().borders(Borders::BOTTOM));
            
            frame.render_widget(nav, chunks[0]);

            // 2. Parse Markdown to Ratatui Text
            let styled_content = parse_markdown(SAMPLE_BLOG);
            
            // 3. Render Blog Content
            let content_widget = Paragraph::new(styled_content)
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::NONE).padding(ratatui::widgets::Padding::new(2, 2, 1, 1)));

            frame.render_widget(content_widget, chunks[1]);
            
            // Setup click handler
            Self::setup_blog_handlers(chunks[0]);

        })?;

        Ok(())
    }

    fn setup_blog_handlers(back_button_area: Rect) {
        use web_sys::window;
        
        let window = window().expect("no global window");
        let document = window.document().expect("no document");
        
        let click_closure = Closure::once(Box::new(move |event: web_sys::MouseEvent| {
            let y = event.client_y() as u16;
            let x = event.client_x() as u16;
            
            // Basic hit detection for the back button area
            // We approximate char height/width or just use raw pixels if we aren't translating coords perfectly yet.
            // For now, let's stick to the heuristic that the top bar is roughly 60px high.
            // If you implement proper coordinate translation like in home.rs, use `back_button_area` values.
            
            // Using back_button_area to silence warning, though real pixel translation requires the char logic from home.rs
            let _ = back_button_area; 

            if y < 60 && x < 300 {
                console::log_1(&"Navigating back to Home...".into());
                let _ = crate::show_home_from_blog();
            } else {
                // If they click the body, re-render to keep it alive (optional)
                 let _ = crate::show_blog_screen();
            }
        }) as Box<dyn FnOnce(web_sys::MouseEvent)>);

        document.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
            .expect("failed to add click listener");
        click_closure.forget();
    }
}

// Fixed Markdown Parser for pulldown-cmark 0.12
fn parse_markdown(markdown: &str) -> Text<'static> {
    let parser = Parser::new(markdown);
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_style = Style::default().fg(Color::Gray);

    for event in parser {
        match event {
            // Text events are the same
            Event::Text(text) => {
                current_line.push(Span::styled(text.to_string(), current_style));
            }
            
            // FIX 1: Tag::Heading is now a struct variant
            Event::Start(Tag::Heading { level, .. }) => {
                // Flush previous line if any
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
                
                // Add spacing before header
                lines.push(Line::from(""));

                current_style = match level {
                    pulldown_cmark::HeadingLevel::H1 => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    pulldown_cmark::HeadingLevel::H2 => Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
                    _ => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                };
            }
            
            // FIX 2: Event::End now uses TagEnd enum, not Tag
            Event::End(TagEnd::Heading(_)) => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
                current_style = Style::default().fg(Color::Gray);
                lines.push(Line::from("")); // Spacing after header
            }
            
            Event::Start(Tag::Paragraph) => {
                // Ensure we start on a new line
            }
            
            // FIX 3: TagEnd for Paragraph
            Event::End(TagEnd::Paragraph) => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
                lines.push(Line::from("")); // Paragraph spacing
            }
            
            Event::Start(Tag::Emphasis) => {
                current_style = current_style.add_modifier(Modifier::ITALIC);
            }
            
            // FIX 4: TagEnd for Emphasis
            Event::End(TagEnd::Emphasis) => {
                current_style = current_style.remove_modifier(Modifier::ITALIC);
            }
            
            Event::Start(Tag::Strong) => {
                current_style = current_style.add_modifier(Modifier::BOLD);
            }
            
            // FIX 5: TagEnd for Strong
            Event::End(TagEnd::Strong) => {
                current_style = current_style.remove_modifier(Modifier::BOLD);
            }
            
            Event::Start(Tag::List(_)) => {
               // lines.push(Line::from(""));
            }
            
            Event::Start(Tag::Item) => {
                current_line.push(Span::styled(" • ", Style::default().fg(Color::Green)));
            }
            
            // FIX 6: TagEnd for Item
            Event::End(TagEnd::Item) => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
            }
            
            _ => {}
        }
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    Text::from(lines)
}
