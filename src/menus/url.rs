use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{Hide, MoveLeft, MoveTo, Show},
    event::KeyCode,
    execute,
    terminal::disable_raw_mode,
};
use gemini::{Url, request::Gemini};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget}, style::{Color, Style},
};

use crate::{app::App, gemini::{gemini_request}};

use super::{Loader, Submenu, SubmenuStatus};

use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
pub struct UrlInputMenu {
    // area: Rect,
    current_url_input: String,
    cursor_pos: (u16, u16),
    has_loaded: bool,
    own_area: Rect,
    //connection: Connection
}

impl UrlInputMenu {
    fn update_cursor(&mut self, pos: (u16, u16)) {
        if !self.has_loaded {
            execute!(stdout(), Show);

            self.has_loaded = true;
        }

        //if self.cursor_pos != pos {
            execute!(stdout(), MoveTo(pos.0, pos.1));
            self.cursor_pos = pos;
        //}
    }
}

impl Default for UrlInputMenu {
    fn default() -> Self {
        //let conn: Connection = Connection::default();
        Self {
            current_url_input: String::from("gemini://gemini.circumlunar.space"),
            cursor_pos: (0, 0),
            has_loaded: false,
            own_area: Rect::new(0, 0, 0, 0),
            //connection: conn
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
        let status = match event.code {
            KeyCode::Enter => {
                // app.
                let url = match Url::parse(&self.current_url_input) {
                    Ok(u) => u,
                    Err(e) => panic!("{}", e),
                };
                
                //let data = self.connection.request(url);
                
                match gemini_request(url) {
                    Ok(data) => {
                        let as_text = data.body_text().unwrap();
                        app.update_page(as_text.to_string());
                    }
                    Err(err) => app.update_page(err.error_message),
                }

                SubmenuStatus::Close
            }
            KeyCode::Char(c) => {
                self.current_url_input.push(c);
                SubmenuStatus::Open
            }
            KeyCode::Backspace => {
                self.current_url_input.pop();
                SubmenuStatus::Open
            }
            KeyCode::Esc => {
                execute!(stdout(), Hide);
                self.has_loaded = false;
                SubmenuStatus::Close
            }
            _ => {
                // todo!();
                SubmenuStatus::Open
                // Ok(())
            }
        };

        let x = self.own_area.x + self.current_url_input.width() as u16 + self.own_area.x;
        let y = self.own_area.y + 1;
        self.update_cursor((x, y));

        return status;
    }

    fn get_main(&mut self, app: &App, area: Rect) -> (Paragraph, Rect) {
        // todo!()
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

        self.own_area = chunks[1];

        let style = Style::default()
            .fg(Color::White)
            .bg(Color::Black);

        let block = Block::default()
        //.style(style)
        .borders(Borders::all());

        let prompted = format!("{}_", self.current_url_input.clone());

        let paragraph = Paragraph::new(Text::from(prompted)).style(style).block(block);

        (paragraph, chunks[1])
    }

    fn menuId(&self) -> u8 {
        1
    }

    /*     fn follow_up(&self, app: &App) {
        todo!()
    } */
}
