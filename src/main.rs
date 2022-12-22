//mod app;
mod ui;
mod app;
pub mod menus;
// mod cli;

use core::time;
use std::{io, time::{Duration, SystemTime, Instant}, thread, rc::Rc};
use app::{App, States};
// use cli::{draw_cli};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode}};
use menus::Submenu;
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders}, text::Text};
use ui::draw;

fn main() -> Result<(), io::Error>{
    
    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_micros(250);

    let start = Instant::now();
    let duration = Duration::from_secs(5);

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            draw(f, &app.clone());
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.on_event(key);
            }
        }

        if app.should_quit {
            break
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
    //println!("Hello, world!");
}