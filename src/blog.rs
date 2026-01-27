use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap, List, ListItem, ListState},
    Terminal,
};
use std::io;
use std::cell::RefCell;
use include_dir::{include_dir, Dir};
use pulldown_cmark::{Parser, Event, Tag, TagEnd};
use wasm_bindgen::prelude::*;
use web_sys::console;

// 1. EMBED THE POSTS DIRECTORY
static POSTS_DIR: Dir = include_dir!("posts");

// 2. STATE MANAGEMENT
#[derive(Clone, Copy, PartialEq)]
enum BlogView {
    Index,          // Showing the list of all posts
    Reading(usize), // Reading a specific post (by index)
}

thread_local! {
    static CURRENT_VIEW: RefCell<BlogView> = RefCell::new(BlogView::Index);
    static LIST_STATE: RefCell<ListState> = RefCell::new(ListState::default());
    // New flag to ensure we only attach the event listener once
    static HANDLERS_INITIALIZED: RefCell<bool> = RefCell::new(false);
}

pub struct BlogScreen;

impl BlogScreen {
    pub fn start<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();

            // Check which view we are in
            let view = CURRENT_VIEW.with(|v| *v.borrow());

            match view {
                BlogView::Index => Self::render_index(frame, area),
                BlogView::Reading(index) => Self::render_post(frame, area, index),
            }
        })?;

        // Setup persistent handlers (only runs once)
        Self::setup_handlers();

        Ok(())
    }

    // --- RENDER FUNCTIONS ---

    fn render_index(frame: &mut ratatui::Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // List
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new("AlertAngel Blog Directory")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(header, chunks[0]);

        // Get file names for the list
        let items: Vec<ListItem> = POSTS_DIR
            .files()
            .map(|file| {
                let name = file.path().file_stem().unwrap().to_str().unwrap();
                let display_name = name.replace("_", " ").to_uppercase();
                ListItem::new(Line::from(vec![
                    Span::styled(" 📝 ", Style::default()),
                    Span::styled(display_name, Style::default().fg(Color::Cyan)),
                ]))
            })
            .collect();

        // List Widget
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Select a Post "))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Green))
            .highlight_symbol(">> ");

        frame.render_widget(list, chunks[1]);

        // Nav Hint
        let footer = Paragraph::new("← Click 'Back' or use standard navigation")
             .alignment(Alignment::Center)
             .style(Style::default().fg(Color::Gray));
        frame.render_widget(footer, chunks[2]);
    }

    fn render_post(frame: &mut ratatui::Frame, area: Rect, index: usize) {
        let files: Vec<_> = POSTS_DIR.files().collect();
        // Safety check
        if index >= files.len() { 
             let _ = Self::go_to_index();
             return; 
        }

        let file = files[index];
        let content = file.contents_utf8().unwrap_or("Error reading file");

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Nav Bar
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Nav Bar
        let nav_text = vec![
            Span::styled(" ← Back to Index ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled(format!(" Post {}/{} ", index + 1, files.len()), Style::default().fg(Color::DarkGray)),
        ];
        
        let nav = Paragraph::new(Line::from(nav_text))
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(nav, chunks[0]);

        // Parse and Render Markdown
        let styled_content = parse_markdown(content);
        let content_widget = Paragraph::new(styled_content)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::NONE).padding(ratatui::widgets::Padding::new(2, 2, 1, 1)));

        frame.render_widget(content_widget, chunks[1]);
    }

    // --- LOGIC FUNCTIONS ---

    fn go_to_index() {
        CURRENT_VIEW.with(|v| *v.borrow_mut() = BlogView::Index);
        let _ = crate::show_blog_screen(); // Re-render
    }

    fn go_to_post(index: usize) {
        CURRENT_VIEW.with(|v| *v.borrow_mut() = BlogView::Reading(index));
        let _ = crate::show_blog_screen(); // Re-render
    }

    // --- HANDLER SETUP ---

    fn setup_handlers() {
        // 1. Check initialization to prevent duplicate listeners
        let already_initialized = HANDLERS_INITIALIZED.with(|initialized| {
            let was_init = *initialized.borrow();
            if !was_init {
                *initialized.borrow_mut() = true;
            }
            was_init
        });

        if already_initialized {
            return;
        }

        console::log_1(&"Initializing Blog Event Handlers (One-time setup)".into());

        use web_sys::window;
        let window = window().expect("no global window");
        let document = window.document().expect("no document");

        // 2. Use FnMut so the closure can be called multiple times without crashing
        let click_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let y = event.client_y() as i32;
            let _x = event.client_x() as i32;

            // 3. Read the CURRENT view dynamically inside the click handler
            let current_view = CURRENT_VIEW.with(|v| *v.borrow());

            match current_view {
                BlogView::Index => {
                    // HIT TESTING FOR INDEX
                    // Header is approx 3 rows * 20px = 60px
                    if y < 60 {
                        console::log_1(&"Header clicked, going home".into());
                        // Important: Reset initialized state if we leave, so we re-attach when we come back?
                        // Actually, since the WASM module state persists, we don't strictly need to reset 
                        // unless we want to clean up. For now, let's just navigate.
                        let _ = crate::show_home_from_blog();
                    } else if y > 60 {
                        // List Area
                        let list_start_y = 80;
                        let item_height = 20; 
                        let clicked_index = ((y - list_start_y) / item_height) as usize;
                        
                        if clicked_index < POSTS_DIR.files().count() {
                            console::log_1(&format!("Clicked post index {}", clicked_index).into());
                            Self::go_to_post(clicked_index);
                        }
                    }
                },
                BlogView::Reading(current_index) => {
                    // HIT TESTING FOR POST
                    if y < 60 {
                        // Clicked "Back to Index"
                         Self::go_to_index();
                    } else {
                        // Clicked body - advance to next post
                        let total_files = POSTS_DIR.files().count();
                        if current_index + 1 < total_files {
                            Self::go_to_post(current_index + 1);
                        } else {
                            Self::go_to_index();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);

        document.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
            .expect("failed to add click listener");
        
        // Leak the closure so it stays valid for the lifetime of the page
        click_closure.forget();
    }
}

// Markdown Parser (Unchanged)
fn parse_markdown(markdown: &str) -> Text<'static> {
    let parser = Parser::new(markdown);
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_style = Style::default().fg(Color::Gray);

    for event in parser {
        match event {
            Event::Text(text) => {
                current_line.push(Span::styled(text.to_string(), current_style));
            }
            Event::Start(Tag::Heading { level, .. }) => {
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
                lines.push(Line::from(""));
                current_style = match level {
                    pulldown_cmark::HeadingLevel::H1 => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    _ => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                };
            }
            Event::End(TagEnd::Heading(_)) => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
                current_style = Style::default().fg(Color::Gray);
                lines.push(Line::from(""));
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
                lines.push(Line::from(""));
            }
            Event::Start(Tag::Emphasis) => { current_style = current_style.add_modifier(Modifier::ITALIC); }
            Event::End(TagEnd::Emphasis) => { current_style = current_style.remove_modifier(Modifier::ITALIC); }
            Event::Start(Tag::Strong) => { current_style = current_style.add_modifier(Modifier::BOLD); }
            Event::End(TagEnd::Strong) => { current_style = current_style.remove_modifier(Modifier::BOLD); }
            Event::Start(Tag::Item) => { current_line.push(Span::styled(" • ", Style::default().fg(Color::Green))); }
            Event::End(TagEnd::Item) => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
            }
            _ => {}
        }
    }
    if !current_line.is_empty() { lines.push(Line::from(current_line)); }
    Text::from(lines)
}
