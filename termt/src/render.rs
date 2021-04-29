use crossterm::{
    cursor,
    style::{self, Colorize},
    QueueableCommand,
};
use std::error::Error;
use std::io::Stdout;

const BOARD_HEIGHT: u16 = 24;
const BOARD_WIDTH: u16 = 10;

const BOARD_START_X: u16 = 20;
const BOARD_START_Y: u16 = 0;

pub struct Screen {
    board: [[u8; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

impl Default for Screen {
    fn default() -> Screen {
        Screen {
            board: [[0u8; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
        }
    }
}

impl Screen {
    pub fn print_border(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        self.board[0][0] = 1;
        stdout.queue(cursor::MoveTo(2, 0))?;
        stdout.queue(style::PrintStyledContent("q to quit".white()))?;
        for y in BOARD_START_Y..(BOARD_START_Y + BOARD_HEIGHT) {
            stdout.queue(cursor::MoveTo(BOARD_START_X, y))?;
            stdout.queue(style::PrintStyledContent("█".white()))?;
            stdout.queue(cursor::MoveTo(BOARD_START_X + BOARD_WIDTH + 1, y))?;
            stdout.queue(style::PrintStyledContent("█".white()))?;
        }
        for x in BOARD_START_X..(BOARD_START_X + BOARD_WIDTH + 2) {
            stdout.queue(cursor::MoveTo(x, BOARD_START_Y + BOARD_HEIGHT))?;
            stdout.queue(style::PrintStyledContent("█".white()))?;
        }
        Ok(())
    }
}
