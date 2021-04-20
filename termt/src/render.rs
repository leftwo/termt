use crossterm::{
    cursor,
    style::{self, Colorize},
    QueueableCommand,
};
use std::error::Error;
use std::io::Stdout;

pub fn print_border(stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
    stdout.queue(cursor::MoveTo(2, 1))?;
    stdout.queue(style::PrintStyledContent("q to quit".white()))?;
    Ok(())
}
