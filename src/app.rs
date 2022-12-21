use std::io::Error;

use crossterm::event::{KeyCode, KeyEvent};
use gemini::{request::Any, response, AnyRequest, Request, Url};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

impl App {
    pub fn new() -> App {
        let current_url = match Url::parse("gemini://") {
            Ok(u) => u,
            Err(e) => panic!("{}", e),
        };

        let hello_world_string = String::from("Hello World");

        App {
            current_url,
            hello_world_string,
            mode: States::BrowsingMode,
        }
    }

    pub fn on_event(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Char(c) => {
                //                            app.on_key(&c);
                if c == 'c' {
                    //launch cli mode
                }
            }
/*             KeyCode::Esc => {
                //toggle command mode or view mode
                if self.mode == States::CommandMode {
                    self.mode == States::BrowsingMode;
                } else {
                    self.mode = States::CommandMode;
                }
            } */
/*             KeyCode::Enter => {
                //pass currently hovered object
                self.enter()
            } */
            _ => {}
        }

        Ok(())
    }

    fn set_browsing_mode(&mut self) {
        self.mode = States::BrowsingMode;
    }

    fn set_command_mode(&mut self) {
        self.mode = States::CommandMode;
    }

    pub fn toggle_state(&mut self) {
        self.mode = match self.mode {
            States::BrowsingMode => States::CommandMode,
            States::CommandMode => States::BrowsingMode,
            States::SubmenuMode => States::CommandMode,
        }
    }

    pub fn enter(&mut self) {
        //do nothing for now
    }

    pub fn enter_submenu(&mut self, menu: &Submenu) {}
}

#[derive(Clone)]
pub struct App {
    current_url: Url,
    pub hello_world_string: String,
    pub mode: States,
}

struct Submenu;

#[derive(Copy, Clone, PartialEq)]
pub enum States {
    BrowsingMode,
    CommandMode,
    SubmenuMode,
}

impl ToString for States {
    fn to_string(&self) -> String {
        match self {
            States::BrowsingMode => String::from("Browsing Mode"),
            States::CommandMode => String::from("Command Mode"),
            States::SubmenuMode => String::from("Submenu Mode"),
        }
    }
}
