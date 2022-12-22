use std::io::{Stdout, stdout};

use crossterm::{event::KeyCode, execute, cursor::{MoveTo, Show, Hide}};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Paragraph, Widget, Borders},
};

use crate::app::App;

use super::{Submenu, SubmenuStatus, Loader};

use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
pub struct UrlInputMenu {
    // area: Rect,
    current_url_input: String,
    cursor_pos: (u16, u16),
    has_loaded: bool
}

impl UrlInputMenu {
    fn update_cursor(&mut self, pos: (u16, u16)) {
        if !self.has_loaded {
            execute!(stdout(), Show);
            self.has_loaded = true;
        }
        
        if self.cursor_pos != pos {
            execute!(stdout(), MoveTo(pos.0, pos.1));
            self.cursor_pos = pos;
        }
    }
}

impl Default for UrlInputMenu {
    fn default() -> Self {
        Self {
            current_url_input: String::new(),
            cursor_pos: (0, 0),
            has_loaded: false
        }
    }
}

impl Submenu for UrlInputMenu {
    fn get_help_string(&self) -> Paragraph {
        // todo!()
        let help = format!("[ESC] Back, [Enter] Open");

        let help_text = Text::from(help);

        let help_pg = Paragraph::new(help_text).block(Block::default());
        help_pg
    }

    fn on_event(&mut self, event: crossterm::event::KeyEvent, app: &mut App) -> SubmenuStatus {
        
        return match event.code {
            KeyCode::Enter => {
                // app.
                SubmenuStatus::Close
            },
            KeyCode::Char(c) => {

                self.current_url_input.push(c);
                SubmenuStatus::Open
            },
            KeyCode::Backspace => {
                self.current_url_input.pop();
                SubmenuStatus::Open
            }
            KeyCode::Esc => {
                execute!(stdout(), Hide);
                self.has_loaded = false;
                SubmenuStatus::Close
            },
            _ => {
                // todo!();
                SubmenuStatus::Open
                // Ok(())
            }
        }
    }

    fn get_main(&mut self, app: &App, area: Rect) -> (Paragraph, Rect) {
        // todo!()
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)].as_ref())
            .split(area);

        let block = Block::default().borders(Borders::all());

        let paragraph = Paragraph::new(Text::from(self.current_url_input.clone())).block(block);

        // InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering

            // chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                // chunks[1].y + 1,

            let x = chunks[1].x + self.current_url_input.width() as u16 + 1;
            let y = chunks[1].y + 1;

            self.update_cursor((x, y));


        (paragraph, chunks[1])
    }

    fn menuId(&self) -> u8 {
        1
    }

/*     fn follow_up(&self, app: &App) {
        todo!()
    } */
}
