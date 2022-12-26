use std::io::{stdout, Stdout};

use crossterm::{
    cursor::{Hide, MoveLeft, MoveTo, Show},
    event::KeyCode,
    execute,
    terminal::disable_raw_mode,
};
use gemini::{request::Gemini, Url};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{app::App, gemini::gemini_request};

use super::{Loader, Submenu, SubmenuStatus};

use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
pub struct UrlInputMenu {
    // area: Rect,
    current_url_input: String,
    cursor_pos: (u16, u16),
    has_loaded: bool,
    own_area: Rect,
    redirect_counter: u8
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

    fn make_request(&mut self, url: &Url, app: &mut App) {
        let req = gemini_request(url);
        match req {
            Ok(data) => match data.header.status.code() {
                gemini::Code::Input => todo!(),
                gemini::Code::SensitiveInput => todo!(),
                gemini::Code::Success => {
                    let as_text = match data.body_text() {
                        Some(body) => body,
                        None => "Error: Empty response".as_ref(),
                    };
                    app.update_page(as_text.to_string());
                }
                gemini::Code::TemporaryRedirect | gemini::Code::PermanentRedirect => {
                    if self.redirect_counter != 10 {
                        self.redirect_counter += 1;



                        match Url::parse(data.header.meta()) {
                            Ok(url_new) => {
                                let data_redir = self.make_request(&url_new, app);
                            },
                            Err(e) => {
                                //Invalid URL
                            }
                        }
                    } else {
                        app.update_page(String::from("Maximum Redirect Limit reached"));
                    }


                },
                gemini::Code::TemporaryFailure => todo!(),
                gemini::Code::ServerUnavailable => todo!(),
                gemini::Code::CGIError => todo!(),
                gemini::Code::ProxyError => todo!(),
                gemini::Code::SlowDown => todo!(),
                gemini::Code::PermanentFailure => todo!(),
                gemini::Code::NotFound => todo!(),
                gemini::Code::Gone => todo!(),
                gemini::Code::ProxyRequestRefused => todo!(),
                gemini::Code::BadRequest => todo!(),
                gemini::Code::ClientCertificateRequired => todo!(),
                gemini::Code::ClientCertificateNotAuthorised => todo!(),
                gemini::Code::CertificateNotValid => todo!(),
            },
            Err(err) => app.update_page(err.error_message),
            //app.update_page(format!("{:?}", req.unwrap()));
            //app.update_page(format!("{}", as));
        }
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
            redirect_counter: 0
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

                //let req = gemini_request(url);

                //let data = self.connection.request(url);

                self.make_request(&url, app);

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

        let chunks_vert = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let style = Style::default().fg(Color::White).bg(Color::Black);

        let block = Block::default()
            //.style(style)
            .borders(Borders::all());

        let prompted = format!("{}_", self.current_url_input.clone());

        let paragraph = Paragraph::new(Text::from(prompted))
            .style(style)
            .block(block);
        self.own_area = chunks_vert[1];

        (paragraph, chunks_vert[1])
    }

    fn menuId(&self) -> u8 {
        1
    }

    /*     fn follow_up(&self, app: &App) {
        todo!()
    } */
}
