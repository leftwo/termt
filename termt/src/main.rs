use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    cursor, event,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::error::Error;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

mod render;

// Inspired (and some initial code) from https://github.com/CleanCut/invaders

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal
    let mut stdout = io::stdout();
    let mut x = 1;
    let mut last_x = 0;
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        thread::sleep(Duration::from_millis(1000));
        render::print_border(&mut stdout)?;
        stdout.queue(cursor::MoveTo(x, 5))?;
        stdout.queue(style::PrintStyledContent("█".magenta()))?;
        stdout.queue(cursor::MoveTo(last_x, 5))?;
        stdout.queue(style::PrintStyledContent(" ".magenta()))?;
        stdout.flush()?;
        last_x = x;
        x += 1;
        if x > 100 {
            x = 0;
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
