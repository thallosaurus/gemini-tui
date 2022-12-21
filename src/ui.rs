use tui::{backend::Backend, Frame, widgets::{Paragraph, Block, Borders}, text::Text, layout::{Direction, Constraint, Layout}};

use crate::app::{App, States};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(0)
    .constraints(
        [
            Constraint::Percentage(90),
            Constraint::Percentage(10)
            ].as_ref()
    ).split(f.size());

    let block = Block::default()
        .title("Title")
        .borders(Borders::all());

    //let text = Text::from(app.hello_world_string);

    let text_string = format!("Current Mode: {}", app.mode.to_string());
    let text = Text::from(text_string);

    let browser_window = Paragraph::new(text)
        .block(block);

    f.render_widget(browser_window, chunks[0]);

    let prompt_prefix = match app.mode {
        States::BrowsingMode => String::from("B"),
        States::CommandMode => String::from(">"),
        States::SubmenuMode => String::from("S"),
    };

    let prompt = format!("{} ", prompt_prefix);

    let prompt_text = Text::from(prompt);

    let prompt_pg = Paragraph::new(prompt_text)
        .block(Block::default());

    f.render_widget(prompt_pg, chunks[1])
}