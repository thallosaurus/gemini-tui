use std::{
    io::{stdout, Error, Stdout},
    rc::Rc,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{KeyCode, KeyEvent},
    execute,
};
use gemini::{request::Any, response, AnyRequest, Request, Url};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::menus::{url::UrlInputMenu, Submenu, SubmenuStatus};

impl App {
    pub fn new() -> App {
        let current_url = match Url::parse("gemini://") {
            Ok(u) => u,
            Err(e) => panic!("{}", e),
        };

        // let hello_world_string = String::from("Hello World");

        App {
            current_url,
            should_quit: false,
            // hello_world_string,
            // mode: States::BrowsingMode,
            submenu: None, // submenus: SubmenuManager::new()
        }
    }

    fn set_browsing_mode(&mut self) {
        self.submenu = None;
    }

    pub fn enter(&mut self) {
        //do nothing for now
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
                    SubmenuStatus::Open => {
                        // menu.follow_up(self);
                        self.submenu = Some(menu)
                    }
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

    pub fn get_main(&self, app: &App, area: Rect) -> tui::widgets::Paragraph {
        // todo!()

        let block = Block::default().title("Title").borders(Borders::all());

        //let text = Text::from(app.hello_world_string);

        let text_string = "Hello from trait-world!";
        let text = Text::from(text_string);

        let browser_window = Paragraph::new(text).block(block);

        browser_window
    }

}

// impl Submenu for App {

#[derive(Clone)]
pub struct App {
    current_url: Url,
    // pub mode: States,
    pub submenu: Option<Box<dyn Submenu>>,
    pub should_quit: bool,
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
