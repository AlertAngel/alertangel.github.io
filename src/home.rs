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
use std::rc::Rc;

pub struct HomeScreen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Normal,
    Hovered
}

impl HomeScreen {
    pub fn start<B>(terminal: &mut Terminal<B>) -> io::Result<()> 
        where 
            B: ratatui::backend::Backend,
    {
        // Use Rc<RefCell<>> to capture button areas from inside the closure
        let demo_area = Rc::new(RefCell::new(Rect::default()));
        let blog_area = Rc::new(RefCell::new(Rect::default()));
        let contact_area = Rc::new(RefCell::new(Rect::default()));

        let demo_area_clone = demo_area.clone();
        let blog_area_clone = blog_area.clone();
        let contact_area_clone = contact_area.clone();

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

            // Store areas for click detection
            *demo_area_clone.borrow_mut() = button_layout[0];
            *blog_area_clone.borrow_mut() = button_layout[2];
            *contact_area_clone.borrow_mut() = button_layout[4];

            // Log the button areas for debugging
            console::log_1(&format!("Demo area: x={}, y={}, w={}, h={}", 
                button_layout[0].x, button_layout[0].y, 
                button_layout[0].width, button_layout[0].height).into());

            // Demo Button
            let demo_button = Self::create_button(
                "Demo", 
                "Get a feel of the UI and workflow before buying the device",
                Color::Cyan,
                ButtonState::Normal
            );

            frame.render_widget(demo_button, button_layout[0]);

            // Blog Button
            let blog_button = Self::create_button(
                "Blog",
                "Coming Soon",
                Color::Magenta,
                ButtonState::Normal,
            );

            frame.render_widget(blog_button, button_layout[2]);

            // Contact Button
            let contact_button = Self::create_button(
                "Contact",
                "Email : lorem@ipsummail.com",
                Color::Green,
                ButtonState::Normal,
            );

            frame.render_widget(contact_button, button_layout[4]);

            // Footer 
            let footer = Paragraph::new("Use keyboard shortcuts or click on buttons to navigate")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);

            frame.render_widget(footer, main_layout[2]);
        })?;

        // Set up click handlers after rendering with captured areas
        Self::setup_click_handlers(
            *demo_area.borrow(),
            *blog_area.borrow(),
            *contact_area.borrow()
        );

        Ok(())
    }

    fn setup_click_handlers(demo_area: Rect, blog_area: Rect, contact_area: Rect) {
        use web_sys::window;
        
        console::log_1(&"Demo Button Ready for Clicks".into());
        
        let window = window().expect("no global window");
        let document = window.document().expect("no document");

        let window_for_closure = window.clone();
        
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.client_x();
            let y = event.client_y();
            
            let char_width = 10;
            let char_height = 20;
            
            let col = (x / char_width) as u16;
            let row = (y / char_height) as u16;
            
            console::log_1(&format!("Click at Col : {} , Row: {}", col, row).into());
            
            // Check if click is in Demo button area
            if col >= demo_area.x && col < demo_area.x + demo_area.width
                && row >= demo_area.y && row < demo_area.y + demo_area.height {
                console::log_1(&"Demo Button Clicked".into());
                
                let window_clone = window_for_closure.clone();
                let closure = Closure::once(Box::new(move || {
                    let _ = crate::show_demo_screen();
                }) as Box<dyn FnOnce()>);
                
                window_clone.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    10
                ).expect("setTimeout failed");
                
                closure.forget();
            }
            
            // Check Blog button
            if col >= blog_area.x && col < blog_area.x + blog_area.width
                && row >= blog_area.y && row < blog_area.y + blog_area.height {
                console::log_1(&"Blog Button Clicked (Coming Soon)".into());
            }
            
            // Check Contact button
            if col >= contact_area.x && col < contact_area.x + contact_area.width
                && row >= contact_area.y && row < contact_area.y + contact_area.height {
                console::log_1(&"Contact Button Clicked".into());
            }
        }) as Box<dyn FnMut(_)>);
        
        document.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .expect("failed to add click listener");
        
        closure.forget();
    }

    fn create_button<'a>(
        title: &'a str,
        tooltip: &'a str,
        color: Color,
        state: ButtonState
    ) -> Paragraph<'a> {
        let (bg_color, border_color) = match state {
            ButtonState::Normal => (Color::Reset, color),
            ButtonState::Hovered => (color, Color::White),
        };

        let content = vec![
            Line::from(vec![
                Span::styled(
                    format!("  {}  ", title),
                    Style::default()
                        .fg(Color::White)
                        .bold()
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    tooltip,
                    Style::default().fg(Color::Gray).italic()
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
