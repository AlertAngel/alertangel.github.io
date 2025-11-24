use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color,Style, Stylize},
    widgets::Paragraph,
    Terminal,
};

const LOGO: &str = r#"
 █████╗ ██╗     ███████╗██████╗ ████████╗
██╔══██╗██║     ██╔════╝██╔══██╗╚══██╔══╝
███████║██║     █████╗  ██████╔╝   ██║   
██╔══██║██║     ██╔══╝  ██╔══██╗   ██║   
██║  ██║███████╗███████╗██║  ██║   ██║   
╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝   ╚═╝   
 █████╗ ███╗   ██╗ ██████╗ ███████╗██╗     
██╔══██╗████╗  ██║██╔════╝ ██╔════╝██║     
███████║██╔██╗ ██║██║  ███╗█████╗  ██║     
██╔══██║██║╚██╗██║██║   ██║██╔══╝  ██║     
██║  ██║██║ ╚████║╚██████╔╝███████╗███████╗
╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝╚══════╝
"#;

pub struct StartupScreen;

impl  StartupScreen {
    pub fn start<B>(terminal: &mut Terminal<B>)
    where
        B: ratatui::backend::Backend,
    {
        terminal.draw(|frame| {
            let layout = Layout::vertical([
                Constraint::Percentage(40),
                Constraint::Min(30),
                Constraint::Percentage(40),
            ])
                .split(frame.area());

            let logo_widget = Paragraph::new(LOGO)
                .style(Style::default().fg(Color::LightYellow).bold())
                .alignment(Alignment::Center);

            frame.render_widget(logo_widget, layout[1]);
        }).unwrap();
    }
}
