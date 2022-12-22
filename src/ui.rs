use std::io::stdout;

use crossterm::{cursor::MoveTo, execute};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::{app::{App, States}, menus::Submenu};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.size());

        let data: (Paragraph, Paragraph);

        draw_widget(f, app.get_main(app, f.size()), chunks[0]);
        draw_widget(f, app.get_help_string(), chunks[1]);

        match app.submenu.clone() {
            None => {
                // data = (app.get_main(app, chunks[0]), app.get_help_string());

            },
            Some(mut menu) => {
                let menu_main = menu.get_main(app, chunks[0]);
                draw_widget(f, menu_main.0, menu_main.1);
                draw_widget(f, menu.get_help_string(), chunks[1]);
            },
        }


        // draw_widget(f, data.1, f.size());
}

fn draw_widget<B: Backend>(f: &mut Frame<B>, w: Paragraph, area: Rect) {
    f.render_widget(w, area);
}

/* fn draw_browser<B: Backend>(f: &mut Frame<B>, w: Paragraph, area: Rect) {


    f.render_widget(w, area);
} */

fn draw_help<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {

/*     let help_prefix = match app.mode {
        States::BrowsingMode => String::from("B"),
        // States::CommandMode => String::from(">"),
        States::SubmenuMode => String::from("S"),
    }; */

/*     let help = format!("[q] Quit, [o] Open");

    let help_text = Text::from(help);

    let help_pg = Paragraph::new(help_text).block(Block::default()); */

    // f.render_widget(help_pg, area);
}