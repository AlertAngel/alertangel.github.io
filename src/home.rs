use ratatui::{
    Terminal, layout::{
        Alignment, 
        Constraint,
        Layout,
        Margin
    }, style::{
        Color,
        Style,
        Stylize
    }, text::{
        Line,
        Span
    }, widgets::{
        Block,
        Borders,
        Paragraph
    }
};
use wasm_bindgen::prelude::wasm_bindgen;

use std::io;

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
        terminal.draw(|frame| {
            let area = frame.area();

            // Main Layout
            let main_layout = Layout::vertical([
                Constraint::Length(8) ,  // Title
                Constraint::Min(10),     // Button Area
                Constraint::Length(2),   // Footer
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
                Constraint::Length(5),      // Blog BUtton 
                Constraint::Length(1),      // Spacing
                Constraint::Length(5),      // Contact BUtton 
                Constraint::Min(0),         // Remaining Space
            ])
                .split(main_layout[1].inner(Margin {
                    horizontal: 20,
                    vertical: 2,
                }));

            // Demo Button
            let demo_button = Self::create_clickable_button(
                "Demo",
                "Get a feel of the UI and Workflow before buying the device",
                Color::Cyan,
                ButtonState::Normal,
                "demo-btn"
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

            // Blog Button
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

        Ok(())
    }

    fn create_clickable_button<'a>(
        title: &'a str,
        tooltip: &'a str,
        color: Color,
        state: ButtonState,
        id: &str,
    ) -> Paragraph<'a> {
        if id == "demo-btn" {
            setup_button_click(id);
        }

        Self::create_button(title, tooltip, color, state)
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn setup_button_click(button_id: &str) {
    use web_sys::window;

    if button_id == "demo-btn" {
        log("Demo Button Ready for Clicks");
    }
}
