use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    cursor, event,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::error::Error;
use std::io;
use std::io::Write;
use std::time::{Duration, Instant};

mod render;

// Inspired (and some initial code) from https://github.com/CleanCut/invaders

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal
    let mut stdout = io::stdout();
    let mut y = 0;
    let mut last_y = 0;
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut instant = Instant::now();
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

        // Time to move down yet?
        let refresh_time = Duration::from_secs(1);
        if instant.elapsed() > refresh_time {
            render::print_border(&mut stdout)?;
            stdout.queue(cursor::MoveTo(15, last_y))?;
            stdout.queue(style::PrintStyledContent(" ".magenta()))?;
            stdout.queue(cursor::MoveTo(15, y))?;
            stdout.queue(style::PrintStyledContent("█".magenta()))?;
            stdout.flush()?;
            last_y = y;
            y += 1;
            if y > 24 {
                y = 0;
            }
            instant = Instant::now();
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
