use ratatui::{
    Terminal, 
    layout::{
        Alignment, 
        Constraint,
        Layout,
        Margin,
        Rect,
    }, 
    style::{
        Color,
        Style,
        Stylize
    }, 
    text::{
        Line,
        Span
    }, 
    widgets::{
        Block,
        Borders,
        Paragraph
    }
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use std::io;
use std::cell::RefCell;

pub struct HomeScreen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Normal,
    Hovered
}

// Global state to track button states
thread_local! {
    static DEMO_HOVERED: RefCell<bool> = RefCell::new(false);
    static BLOG_HOVERED: RefCell<bool> = RefCell::new(false);
    static CONTACT_HOVERED: RefCell<bool> = RefCell::new(false);
    static LISTENERS_ATTACHED: RefCell<bool> = RefCell::new(false);
}

impl HomeScreen {
    pub fn start<B>(terminal: &mut Terminal<B>) -> io::Result<()> 
        where 
            B: ratatui::backend::Backend,
    {
        terminal.draw(|frame| {
            let area = frame.area();

            // Main Layout
            let main_layout = Layout::vertical([
                Constraint::Length(6),       // Title
                Constraint::Min(20),         // Button Area
                Constraint::Length(3),       // Footer
            ])
                .split(area);

            // Title
            let title = Paragraph::new("AlertAngel")
                .style(Style::default().fg(Color::LightYellow).bold())
                .alignment(Alignment::Center);

            frame.render_widget(title, main_layout[0]);

            // Button Layout 
            let button_layout = Layout::vertical([
                Constraint::Length(5),      // Demo Button
                Constraint::Length(1),      // Spacing
                Constraint::Length(5),      // Blog Button 
                Constraint::Length(1),      // Spacing
                Constraint::Length(5),      // Contact Button 
                Constraint::Min(0),         // Remaining Space
            ])
                .split(main_layout[1].inner(Margin {
                    horizontal: 20,
                    vertical: 2,
                }));

            // Demo Button
            let demo_state = DEMO_HOVERED.with(|h| if *h.borrow() { ButtonState::Hovered } else { ButtonState::Normal });
            let demo_button = Self::create_button(
                "Demo", 
                "Get a feel of the UI and workflow before buying the device",
                Color::Cyan,
                demo_state
            );
            frame.render_widget(demo_button, button_layout[0]);

            // Blog Button
            let blog_state = BLOG_HOVERED.with(|h| if *h.borrow() { ButtonState::Hovered } else { ButtonState::Normal });
            let blog_button = Self::create_button(
                "Blog",
                "Coming Soon",
                Color::Magenta,
                blog_state,
            );
            frame.render_widget(blog_button, button_layout[2]);

            // Contact Button
            let contact_state = CONTACT_HOVERED.with(|h| if *h.borrow() { ButtonState::Hovered } else { ButtonState::Normal });
            let contact_button = Self::create_button(
                "Contact",
                "Email : lorem@ipsummail.com",
                Color::Green,
                contact_state,
            );
            frame.render_widget(contact_button, button_layout[4]);

            // Footer 
            let footer = Paragraph::new("Use keyboard shortcuts or click on buttons to navigate")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);

            frame.render_widget(footer, main_layout[2]);

            // Store button areas for event handlers
            // We need to capture these for the event handlers
            LISTENERS_ATTACHED.with(|attached| {
                if !*attached.borrow() {
                    // Set up event handlers with the button areas
                    Self::setup_event_handlers(
                        button_layout[0],
                        button_layout[2],
                        button_layout[4]
                    );
                    *attached.borrow_mut() = true;
                }
            });
        })?;

        Ok(())
    }

    fn setup_event_handlers(demo_area: Rect, blog_area: Rect, contact_area: Rect) {
        use web_sys::window;
        
        let window = window().expect("no global window");
        let document = window.document().expect("no document");
        
        console::log_1(&"Setting up home screen event handlers".into());

        // Click handler
        let window_for_click = window.clone();
        let click_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let (col, row) = Self::get_terminal_coords(event.client_x(), event.client_y());
            
            console::log_1(&format!("Click at Col: {}, Row: {}", col, row).into());
            
            if Self::is_in_area(col, row, demo_area) {
                console::log_1(&"Demo Button Clicked".into());
                
                // Reset listener flag for demo screen
                LISTENERS_ATTACHED.with(|attached| {
                    *attached.borrow_mut() = false;
                });
                
                let window_clone = window_for_click.clone();
                let closure = Closure::once(Box::new(move || {
                    let _ = crate::show_demo_screen();
                }) as Box<dyn FnOnce()>);
                
                window_clone.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    10
                ).expect("setTimeout failed");
                
                closure.forget();
            }
            
            if Self::is_in_area(col, row, blog_area) {
                console::log_1(&"Blog Button Clicked (Coming Soon)".into());
            }
            
            if Self::is_in_area(col, row, contact_area) {
                console::log_1(&"Contact Button Clicked".into());
            }
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
            .expect("failed to add click listener");
        click_closure.forget();

        // Mousemove handler for hover effects
        let document_clone = document.clone();
        let mousemove_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let (col, row) = Self::get_terminal_coords(event.client_x(), event.client_y());
            
            let demo_hover = Self::is_in_area(col, row, demo_area);
            let blog_hover = Self::is_in_area(col, row, blog_area);
            let contact_hover = Self::is_in_area(col, row, contact_area);
            
            let any_hover = demo_hover || blog_hover || contact_hover;
            
            // Update cursor style - cast body to HtmlElement
            if let Some(body) = document_clone.body() {
                if let Some(html_body) = body.dyn_ref::<web_sys::HtmlElement>() {
                    let cursor_style = if any_hover { "pointer" } else { "default" };
                    let _ = html_body.style().set_property("cursor", cursor_style);
                }
            }
            
            // Update hover states
            let mut needs_redraw = false;
            
            DEMO_HOVERED.with(|h| {
                if *h.borrow() != demo_hover {
                    *h.borrow_mut() = demo_hover;
                    needs_redraw = true;
                }
            });
            
            BLOG_HOVERED.with(|h| {
                if *h.borrow() != blog_hover {
                    *h.borrow_mut() = blog_hover;
                    needs_redraw = true;
                }
            });
            
            CONTACT_HOVERED.with(|h| {
                if *h.borrow() != contact_hover {
                    *h.borrow_mut() = contact_hover;
                    needs_redraw = true;
                }
            });
            
            // Redraw if hover state changed
            if needs_redraw {
                let backend = ratzilla::DomBackend::new().unwrap();
                let mut terminal = ratatui::Terminal::new(backend).unwrap();
                let _ = Self::start(&mut terminal);
            }
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref())
            .expect("failed to add mousemove listener");
        mousemove_closure.forget();
    }

    fn get_terminal_coords(x: i32, y: i32) -> (u16, u16) {
        const CHAR_WIDTH: i32 = 10;
        const CHAR_HEIGHT: i32 = 20;
        
        let col = (x / CHAR_WIDTH) as u16;
        let row = (y / CHAR_HEIGHT) as u16;
        
        (col, row)
    }

    fn is_in_area(col: u16, row: u16, area: Rect) -> bool {
        col >= area.x && col < area.x + area.width
            && row >= area.y && row < area.y + area.height
    }

    fn create_button<'a>(
        title: &'a str,
        tooltip: &'a str,
        color: Color,
        state: ButtonState
    ) -> Paragraph<'a> {
        let (bg_color, border_color, text_color) = match state {
            ButtonState::Normal => (Color::Reset, color, Color::White),
            ButtonState::Hovered => (color, Color::White, Color::Black),
        };

        let content = vec![
            Line::from(vec![
                Span::styled(
                    format!("  {}  ", title),
                    Style::default()
                        .fg(text_color)
                        .bold()
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    tooltip,
                    Style::default()
                        .fg(if state == ButtonState::Hovered { Color::DarkGray } else { Color::Gray })
                        .italic()
                ),
            ]),
        ];

        Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().bg(bg_color))
            )
            .alignment(Alignment::Center)
    }
}
