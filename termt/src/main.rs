use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use std::error::Error;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

const VX_BOARD_START: usize = 2;
const VX_BOARD_END: usize = 12;
const VY_BOARD_END: usize = 16;
const BOARD_HEIGHT: usize = 18;
const BOARD_WIDTH: usize = 14;

mod piece;
mod render;
use piece::{Piece, Tetroimino};

mod test;

// Inspired (and some initial code) from https://github.com/CleanCut/invaders

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal
    let mut stdout = io::stdout();
    let mut board = new_empty_board();

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut p = Piece {
        kind: Tetroimino::O,
        x: 6,
        last_x: 6,
        y: 0,
        last_y: 0,
        rotation: 0,
        last_rotation: 0,
    };

    let mut game_over = false;
    let mut pause = false;
    let mut instant = Instant::now();
    render::print_board(&mut stdout, &board, game_over, true)?;
    // Game loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('d') => {
                        p.move_right(&board);
                    }
                    KeyCode::Char('a') => {
                        p.move_left(&board);
                    }
                    KeyCode::Char('s') => {
                        p.move_down(&board);
                    }
                    KeyCode::Char('w') => {
                        p.rotate(&board);
                    }
                    KeyCode::Char('p') => {
                        pause = true;
                    }
                    _ => {}
                }
            }
        }

        render::print_board(&mut stdout, &board, game_over, false)?;

        if pause {
            render::print_board(&mut stdout, &board, game_over, true)?;
            sleep(Duration::from_secs(1));
            continue;
        }
        if game_over {
            // Nothing else to do but wait for input
            // from the user.
            render::print_board(&mut stdout, &board, game_over, true)?;
            continue;
        }

        render::print_piece(&mut stdout, &p)?;
        p.update();

        // Time to move down yet?
        let refresh_time = Duration::from_secs(1);
        if instant.elapsed() > refresh_time {
            // If move down can't move down, then piece becomes board.
            if !p.move_down(&board) {
                // Can't move down any more, so, new piece
                p.settle(&mut board); // Transition to perm.

                let tets = tetris_check(&board);

                if tets.is_empty() {
                    // Check for a full row
                    if p.y == 0 {
                        // Game over
                        game_over = true;
                    }
                } else {
                    // We made a tet.
                    tetris_clear(&mut board, &tets);
                    render::print_board(&mut stdout, &board, false, true)?;
                    sleep(Duration::from_secs(2));
                    tetris_shift(&mut board, &tets);
                    render::print_board(&mut stdout, &board, false, true)?;
                    sleep(Duration::from_secs(1));
                }
                p.reset();
            } else {
                render::print_msg(&mut stdout, "down ok");
            }
            //render::print_board(&mut stdout, &board)?;
            render::print_piece(&mut stdout, &p)?;
            instant = Instant::now();
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

// Build a new empty board matrix
#[allow(clippy::needless_range_loop)]
fn new_empty_board() -> Vec<Vec<bool>> {
    let mut board = vec![vec![false; BOARD_HEIGHT]; BOARD_WIDTH];

    // Build the border around the playing field
    for y in 0..BOARD_HEIGHT {
        for x in 0..VX_BOARD_START {
            board[x][y] = true;
        }
        for x in VX_BOARD_END..BOARD_WIDTH {
            board[x][y] = true;
        }
    }
    for x in 0..BOARD_WIDTH {
        for y in VY_BOARD_END..BOARD_HEIGHT {
            board[x][y] = true;
        }
    }

    board
}

// Walk the rows of a board looking for one where every x is set.
#[allow(clippy::needless_range_loop)]
fn tetris_check(board: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut tets = Vec::new();
    // Check ever row except the last two, those we don't consider.
    for y in 0..board[0].len() - 2 {
        let mut count = 0;
        for x in 0..board.len() {
            if board[x][y] {
                count += 1;
            }
        }
        if count == board.len() {
            tets.push(y);
        }
    }
    tets
}
// Erase all contents for the row in a board based on values in the
// tets vec.
fn tetris_clear(board: &mut Vec<Vec<bool>>, tets: &[usize]) {
    // Check every row except the last, that one we don't consider.
    for y in tets.iter() {
        for x in 2..board.len() - 2 {
            board[x][*y] = false;
        }
    }
}

// Shift down all active board spots, replacing whatever is in the
// tets vec rows with the rows above them.  No checking for actual
// contents is done here, so it's assumed you are sending us the
// correct data.
#[allow(clippy::needless_range_loop)]
fn tetris_shift(board: &mut Vec<Vec<bool>>, tets: &[usize]) {
    assert!(!tets.is_empty());
    let mut dest_row: usize = *tets.last().unwrap();
    let mut copy_from_row = dest_row - 1;

    loop {
        while tets.contains(&copy_from_row) {
            if copy_from_row == 0 {
                break;
            }
            copy_from_row -= 1;
        }
        for x in 0..board.len() {
            board[x][dest_row] = board[x][copy_from_row];
        }
        dest_row -= 1;
        if copy_from_row == 0 {
            break;
        }
        copy_from_row -= 1;
    }
    // Any remaining rows at the top should be empty.
    while dest_row != 0 {
        for x in 2..board.len() - 2 {
            board[x][dest_row] = false;
        }
        dest_row -= 1;
    }
    // The top row will always be empty.
    for x in 2..board.len() - 2 {
        board[x][0] = false;
    }
}
