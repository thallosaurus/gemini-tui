// mod menus;
pub mod url;

use std::io::{Error, stdout};

use crossterm::{event::KeyEvent, execute, cursor::Show};
use tui::{layout::Rect, widgets::{Widget, Block, Paragraph}, backend::Backend, Frame};

use crate::app::App;

use self::url::UrlInputMenu;

pub enum SubmenuStatus {
    Open,
    Close
}

pub trait Submenu: SubmenuClone {
    // type Success;
    // type Error;
    fn menuId(&self) -> u8;
    fn get_help_string(&self) -> Paragraph;
    fn get_main(&mut self, app: &App, area: Rect) -> (Paragraph, Rect);
    fn on_event(&mut self, event: KeyEvent, app: &mut App) -> SubmenuStatus;
    // fn follow_up(&self, app: &App);
    // fn show(&mut self);
    // fn clone_dyn(&self) -> Box<dyn Submenu>;
}

pub trait Loader {
    fn show(&mut self);
}

pub trait SubmenuClone {
    fn clone_box(&self) -> Box<dyn Submenu>;
}

impl<T> SubmenuClone for T
where
    T: 'static + Submenu + Clone,
{
    fn clone_box(&self) -> Box<dyn Submenu> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Submenu> {
    fn clone(&self) -> Box<dyn Submenu> {
        self.clone_box()
    }
}

/* impl Loader for Box<dyn Submenu> {
    fn show(&mut self) {
        // todo!()
        execute!(stdout(), Show);
        self.show()
    }
} */

impl PartialEq for Box<dyn Submenu> {
    fn eq(&self, other: &Self) -> bool {
        self.menuId() == other.menuId() && self.menuId() == other.menuId()
    }
}

