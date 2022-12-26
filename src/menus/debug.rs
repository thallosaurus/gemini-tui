use tui::{widgets::{Paragraph, Block, Borders}, layout::{Rect, Layout, Direction, Constraint}, style::{Style, Color}, text::Text};

use super::Submenu;

#[derive(Debug, Clone)]
pub struct DebugWindow {
    lines: Vec<String>,
    own_area: Rect
}

impl Submenu for DebugWindow {
    fn menuId(&self) -> u8 {
        2
    }

    fn get_help_string(&self) -> Paragraph {
        Paragraph::new(Text::from(""))
    }

    fn get_main(&mut self, app: &crate::app::App, area: tui::layout::Rect) -> (Paragraph, Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(area);

            let style = Style::default()
            .fg(Color::White)
            .bg(Color::Black);
            
            let block = Block::default()
            //.style(style)
            .borders(Borders::all());
            
            //let prompted = format!("{}_", self.current_url_input.clone());

            let lines = self.lines.join("\n");
            
            let paragraph = Paragraph::new(Text::from(lines)).style(style).block(block);
            self.own_area = chunks[2];

        (paragraph, chunks[2])
    }

    fn on_event(&mut self, event: crossterm::event::KeyEvent, app: &mut crate::app::App) -> super::SubmenuStatus {
        todo!()
    }
}

impl Default for DebugWindow {
    fn default() -> Self {
        Self { lines: vec![], own_area: Rect { x: 0, y: 0, width: 0, height:0 } }
    }
}