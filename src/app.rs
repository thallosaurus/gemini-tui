use std::{
    io::{stdout, Error, Stdout},
    process::Stdio,
    rc::Rc, ops::Add,
};

use crossterm::terminal::ClearType::All;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{KeyCode, KeyEvent},
    execute,
    terminal::Clear,
};
use gemini::{request::Any, response, AnyRequest, Request, Url};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{menus::{url::UrlInputMenu, Submenu, SubmenuStatus}, gemini::GeminiError};

impl App {
    pub fn new() -> App {
        let current_url = match Url::parse("gemini://gemini.circumlunar.space") {
            Ok(u) => u,
            Err(e) => panic!("{}", e),
        };

        // let hello_world_string = String::from("Hello World");

        let text = String::from("20 text/gemini\r\n # Hello World\n\n");
        App {
            current_url,
            should_quit: false,
            display_text: Some(text.clone()),
            // hello_world_string,
            // mode: States::BrowsingMode,
            submenu: None,

            //Length presents Height, Each Element represents the length of each paragraph
            //viewport: vec![text.clone().len()], // submenus: SubmenuManager::new()
            width: text.clone().len(),
            height: 10,
            x: 0,
            y: 0,
        }
    }

    pub fn clear_screen(&mut self) {
        //execute!(stdout(), Clear(All));
        //execute!(Stdout(), Clear);
    }

    fn set_browsing_mode(&mut self) {
        self.submenu = None;
    }

    fn has_submenu(&self) -> bool {
        return self.submenu != None;
    }

    pub fn on_event(&mut self, event: KeyEvent) {
        let menu_copy = self.submenu.clone();

        if self.has_submenu() {
            // Some(mut menu) => {
            if let Some(mut menu) = menu_copy {
                match menu.on_event(event, self) {
                    SubmenuStatus::Open => self.submenu = Some(menu),
                    // (SubmenuStatus::Close, Some(res)) => self.submenu = None,
                    SubmenuStatus::Close => self.submenu = None,
                }
            }
        } else {
            match event.code {
                KeyCode::Char(c) => {
                    //                            app.on_key(&c);
                    match c {
                        'o' => {
                            //open new gemsite
                            // self.enter_submenu();
                            self.submenu = Some(Box::new(UrlInputMenu::default()))
                        }
                        'q' => {
                            self.should_quit = true;
                        }
                        _ => {}
                    }
                }
                KeyCode::Up => {
                    //scroll up
                    if self.y as usize > self.height {
                        self.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.y as usize > 0
                        self.y += 1;
                    }
                }
                _ => {}
            }
        }
        // Ok(())
        // SubmenuStatus::Open
    }

    pub fn get_help_string(&self) -> Paragraph {
        let help = format!("[q] Quit, [o] Open");

        let help_text = Text::from(help);

        let help_pg = Paragraph::new(help_text).block(Block::default());
        help_pg
    }

    pub fn get_main(&mut self, app: &mut App, area: Rect) -> tui::widgets::Paragraph {
        let style = Style::default().fg(Color::White).bg(Color::Black);

        let masterblock = Block::default()
            .style(style)
            .title("Title")
            .borders(Borders::all());
            //mastertext.body_text().
        //let mut mastertext = Vec::new();

        /*for line in text_split.iter().enumerate() {
            if line.0 >= self.y.into() {
                mastertext.push(*line.1);
            }
        }*/

        /*let filtered: Vec<&str> = text_split.iter().enumerate().filter(|f| {
            return f.0 > height_offset.into() && f.0 < height_offset_end.into();
        }).map(|f| return *f.1).collect();*/

        //let built = filtered.join("\n");

        //let built = format!("{}", cop);

        let empty = String::from("");
        let built = self.display_text.as_ref().unwrap_or(&empty);

        let collected: Vec<&str> = self.display_text.as_ref().unwrap().lines().skip(self.y.into()).collect();

        let text = Text::from(String::from(format!("{:?}", collected)));

        let browser_window = Paragraph::new(text).style(style).block(masterblock);

        browser_window
    }

    pub fn update_page(&mut self, content: String) {
        //Length presents Height, Each Element represents the length of each paragraph

        let c = content;

        self.width = calculate_page_width(&c);
        self.height = calculate_page_height(&c);
        self.display_text = Some(c);
    }
}

#[derive(Clone)]
pub struct App {
    pub current_url: Url,
    // pub mode: States,
    pub submenu: Option<Box<dyn Submenu>>,
    pub should_quit: bool,
    pub display_text: Option<String>,
    //pub viewport: Vec<usize>
    pub width: usize,
    pub height: usize,
    x: u16,
    y: u16,
}

#[derive(Copy, Clone, PartialEq)]
pub enum States {
    BrowsingMode,
    SubmenuMode,
}

impl ToString for States {
    fn to_string(&self) -> String {
        match self {
            States::BrowsingMode => String::from("Browsing Mode"),
            // States::CommandMode => String::from("Command Mode"),
            States::SubmenuMode => String::from("Submenu Mode"),
        }
    }
}

fn calculate_page_width(data: &String) -> usize {
    let vport = data
        .split("\r\n")
        .into_iter()
        .map(|f| return f.len())
        .max()
        .unwrap_or(0);

    vport

    //Length presents Height, Each Element represents the length of each paragraph
}

fn calculate_page_height(data: &String) -> usize {
    let vport: Vec<usize> = data
        .split("\r\n")
        .into_iter()
        .map(|f| return f.len())
        .collect();

    vport.len()
}
