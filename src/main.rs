//mod app;
mod ui;
mod app;
mod cli;

use core::time;
use std::{io, time::{Duration, SystemTime, Instant}, thread};
use app::{App, States};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode}};
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
            draw(f, app.clone());
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                
                if let KeyCode::Char(c) = key.code {
                    if c == 'q' && app.mode == States::BrowsingMode {
                        break
                    }
                }
                app.on_event(key);
                /*match key.code {
                    KeyCode::Char(c) => {
                        match c {
                            'q' => break,
                            _ => app.on_key(&c)
                        }
//                            app.on_key(&c);
                    },
                    KeyCode::Esc => {
                        //toggle command mode or view mode
                        app.toggle_state()
                    },
                    KeyCode::Enter => {

                        //pass currently hovered object
                        app.enter()
                    }
                    _ => {}
                }*/
            }
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