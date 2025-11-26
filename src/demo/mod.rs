use ratatui::{
    layout::{
        Alignment,
        Constraint,
        Layout
    },
    style::{
        Color,
        Style,
        Stylize
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
    Terminal
};

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
                Constraint::Length(3)
            ])
                .split(area);

            let title = Paragraph::new("AlertAngel - Demo")
                .style(Style::default().fg(Color::LightYellow).bold())
                .alignment(Alignment::Center);

            frame.render_widget(title, layout[0]);

            let content = Paragraph::new(vec![
                "Demo Mode".into(),
                "".into(),
                "You can use this demo to get used to the UI before buying the device".into(),
                "".into(),
                "UI Coming Soon...".into(),
                "Real time sensor data visualization".into(),
                "UI Workflow Demonstration".into()
            ])
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                )
                    .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);

            frame.render_widget(content, layout[1]);

            let footer = Paragraph::new("Press ESC to go back home")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);

            frame.render_widget(footer, layout[2]);
        })?;

        Ok(())
    }
}
