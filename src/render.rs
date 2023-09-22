use crossterm::{
    cursor,
    style::{self, Colorize},
    QueueableCommand,
};
use std::error::Error;
use std::io::Stdout;

use super::*;

pub fn print_msg(stdout: &mut Stdout, msg: &str) {
    stdout.queue(cursor::MoveTo(20, 7)).unwrap();
    stdout
        .queue(style::PrintStyledContent(msg.white()))
        .unwrap();
}

#[allow(clippy::needless_range_loop)]
pub fn print_board(
    stdout: &mut Stdout,
    board: &Vec<Vec<bool>>,
    game_over: bool,
    print_empty: bool,
) -> Result<(), Box<dyn Error>> {
    stdout.queue(cursor::MoveTo(20, 1))?;
    stdout.queue(style::PrintStyledContent("a Left".white()))?;
    stdout.queue(cursor::MoveTo(20, 2))?;
    stdout.queue(style::PrintStyledContent("d Right".white()))?;
    stdout.queue(cursor::MoveTo(20, 3))?;
    stdout.queue(style::PrintStyledContent("s Down".white()))?;
    stdout.queue(cursor::MoveTo(20, 4))?;
    stdout.queue(style::PrintStyledContent("w Rotate".white()))?;
    stdout.queue(cursor::MoveTo(20, 6))?;
    stdout.queue(style::PrintStyledContent("q to quit".white()))?;

    if game_over {
        stdout.queue(cursor::MoveTo(20, 8))?;
        stdout.queue(style::PrintStyledContent("GAME OVER".white()))?;
    }

    for y in 0..board[0].len() {
        for x in 0..board.len() {
            stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
            if board[x][y] {
                stdout.queue(style::PrintStyledContent("b".white()))?;
            } else if print_empty {
                stdout.queue(style::PrintStyledContent(" ".white()))?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::needless_range_loop)]
pub fn print_piece(
    stdout: &mut Stdout,
    p: &Piece
) -> Result<(), Box<dyn Error>> {
    let matrix = p.matrix();
    // If we moved, then erase our old location
    if p.x != p.last_x || p.y != p.last_y {
        for y in 0..matrix[0].len() {
            for x in 0..matrix.len() {
                if matrix[x][y] {
                    stdout.queue(cursor::MoveTo(
                        (p.last_x + x) as u16,
                        (p.last_y + y) as u16)
                    )?;
                    stdout.queue(style::PrintStyledContent(" ".black()))?;
                }
            }
        }
    }
    if p.rotation != p.last_rotation {
        //let mut pl = *p.clone();
        let mut pl = *p;
        pl.rotation = p.last_rotation;
        let lr_matrix = pl.matrix();
        for y in 0..lr_matrix[0].len() {
            for x in 0..lr_matrix.len() {
                if lr_matrix[x][y] {
                    stdout.queue(cursor::MoveTo(
                        (p.last_x + x) as u16,
                        (p.last_y + y) as u16)
                    )?;
                    stdout.queue(style::PrintStyledContent(" ".black()))?;
                }
            }
        }
    }

    // Print our tet at the new location.
    for y in 0..matrix[0].len() {
        for x in 0..matrix.len() {
            if matrix[x][y] {
                stdout.queue(cursor::MoveTo(
                    (p.x + x) as u16,
                    (p.y + y) as u16)
                )?;
                stdout.queue(style::PrintStyledContent("#".white()))?;
            }
        }
    }

    Ok(())
}
