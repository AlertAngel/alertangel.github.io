use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction,Layout, Alignment},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use std::io;
use pulldown_cmark::{Parser, Event, Tag};
use wasm_bindgen::prelude::*;
use web_sys::console;

// Markdown content here
// Sample placeholder blog
const SAMPLE_BLOG: &str = r#"
# AlertAngel Blog

This is a Sample Blog for the AlertAngel Device

## Ratzilla

`ratzilla` is cool.

```rust
fn main() {
    println!("Hello World");
}
```

End of Sample Blog
"#;

pub struct BlogScreen;

impl BlogScreen {
    pub fn start<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();

            // Split the screen into Header (Back Button) and Content
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),      // Header/Nav
                    Constraint::Min(0),         // Blog Content
                ])
                .split(area);

            // Navigation Bar 
            let nav = Paragraph::new("<- Back to Home (Click Here)")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Left)
                .block(Block::default().borders(Borders::BOTTOM));

            frame.render_widget(nav, chunks[0]);

            // Parse Markdown to ratatui text
            let styled_content = parse_markdown(SAMPLE_BLOG);

            // Render Blog
            let content_widget = Paragraph::new(styled_content)
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::NONE).padding(ratatui::widgets::Padding::new(2, 2, 1, 1)));

            frame.render_widget(content_widget, chunks[1]);

            // Click Handler for Back button
            // Area for click handler
            Self::setup_blog_handlers(chunks[0]);
        })?;

        Ok(())
    }

    fn setup_blog_handlers(back_button_area: ratatui::layout::Rect) {
        use web_sys::window;

        let window = window().expect("No Global Window");
        let document = window.document().expect("No Document");

        // For just checks if the top area is clicked
        // TODO: Make it more robust using the state management used in home.rs
        let click_closure = Closure::once(Box::new(move |event: web_sys::MouseEvent| {
            let y = event.client_y();

            // Very rough estimation 
            if y < 60 {
                console::log_1(&"Navigating back to Home..".into());
                let _ = crate::show_home_from_blog();
            } else {
                // If body is clicked, the listener should be re-attached to keep the blog active
                // For now, this is a one-shot listener
                let _ = crate::show_blog_screen();
            }
        }) as Box<dyn FnOnce(web_sys::MouseEvent)>);

        // We use the "Once" here to keep it simple. Ideally we want a persistent listener
        document.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
            .expect("Failed to add event listener");

        click_closure.forget();
    }
}

// Simple markdown parser for ratatui 
fn parse_markdown(markdown: &str)-> Text<'static>  {
    let parser = Parser::new(markdown);
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_style = Style::default().fg(Color::Gray);

    for event in parser {
        match event {
            Event::Text(text) => {
                current_line.push(Span::styled(text.to_string(), current_style));
            }
            Event::Start(Tag::Heading(level, _, _)) => {
                // Flush previous line if any 
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
            }
        }
    }
}
