use super::*;

#[derive(Debug, Copy, Clone)]
pub enum Tetroimino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub kind: Tetroimino,
    pub last_x: usize,
    pub x: usize,
    pub last_y: usize,
    pub y: usize,
    pub rotation: u8,
    pub last_rotation: u8,
}

impl Piece {
    // Return a matrix of the Tetroimino at the current rotation
    // The way this is initialized is rotated 90 counter-clockwise
    // from how it will be shown with the usual x,y index.
    pub fn matrix(self) -> Vec<Vec<bool>> {
        match self.kind {
            Tetroimino::O => {
                let matrix = vec![vec![true, true], vec![true, true]];
                matrix
            }
            Tetroimino::T => match self.rotation {
                0 => {
                    vec![
                        vec![false, true, false],
                        vec![false, true, true],
                        vec![false, true, false],
                    ]
                }
                1 => {
                    vec![
                        vec![false, false, false],
                        vec![true, true, true],
                        vec![false, true, false],
                    ]
                }
                2 => {
                    vec![
                        vec![false, true, false],
                        vec![true, true, false],
                        vec![false, true, false],
                    ]
                }
                _ => {
                    vec![
                        vec![false, true, false],
                        vec![true, true, true],
                        vec![false, false, false],
                    ]
                }
            },
            Tetroimino::J => match self.rotation {
                0 => {
                    vec![
                        vec![false, true, false],
                        vec![false, true, false],
                        vec![true, true, false],
                    ]
                }
                1 => {
                    vec![
                        vec![true, false, false],
                        vec![true, true, true],
                        vec![false, false, false],
                    ]
                }
                2 => {
                    vec![
                        vec![false, true, true],
                        vec![false, true, false],
                        vec![false, true, false],
                    ]
                }
                _ => {
                    vec![
                        vec![false, false, false],
                        vec![true, true, true],
                        vec![false, false, true],
                    ]
                }
            },
            Tetroimino::L => match self.rotation {
                0 => {
                    vec![
                        vec![true, true, false],
                        vec![false, true, false],
                        vec![false, true, false],
                    ]
                }
                1 => {
                    vec![
                        vec![false, false, true],
                        vec![true, true, true],
                        vec![false, false, false],
                    ]
                }
                2 => {
                    vec![
                        vec![false, true, false],
                        vec![false, true, false],
                        vec![false, true, true],
                    ]
                }
                _ => {
                    vec![
                        vec![false, false, false],
                        vec![true, true, true],
                        vec![true, false, false],
                    ]
                }
            },
            Tetroimino::S => match self.rotation {
                0 => {
                    vec![
                        vec![false, true, false],
                        vec![true, true, false],
                        vec![true, false, false],
                    ]
                }
                1 => {
                    vec![
                        vec![true, true, false],
                        vec![false, true, true],
                        vec![false, false, false],
                    ]
                }
                2 => {
                    vec![
                        vec![false, false, true],
                        vec![false, true, true],
                        vec![false, true, false],
                    ]
                }
                _ => {
                    vec![
                        vec![false, false, false],
                        vec![true, true, false],
                        vec![false, true, true],
                    ]
                }
            },
            Tetroimino::Z => match self.rotation {
                0 => {
                    vec![
                        vec![true, false, false],
                        vec![true, true, false],
                        vec![false, true, false],
                    ]
                }
                1 => {
                    vec![
                        vec![false, true, true],
                        vec![true, true, false],
                        vec![false, false, false],
                    ]
                }
                2 => {
                    vec![
                        vec![false, true, false],
                        vec![false, true, true],
                        vec![false, false, true],
                    ]
                }
                _ => {
                    vec![
                        vec![false, false, false],
                        vec![true, true, false],
                        vec![false, true, true],
                    ]
                }
            },
            Tetroimino::I => match self.rotation {
                0 => {
                    vec![
                        vec![false, true, false, false],
                        vec![false, true, false, false],
                        vec![false, true, false, false],
                        vec![false, true, false, false],
                    ]
                }
                1 => {
                    vec![
                        vec![false, false, false, false],
                        vec![true, true, true, true],
                        vec![false, false, false, false],
                        vec![false, false, false, false],
                    ]
                }
                2 => {
                    vec![
                        vec![false, true, false, false],
                        vec![false, true, false, false],
                        vec![false, true, false, false],
                        vec![false, true, false, false],
                    ]
                }
                _ => {
                    vec![
                        vec![false, false, false, false],
                        vec![true, true, true, true],
                        vec![false, false, false, false],
                        vec![false, false, false, false],
                    ]
                }
            },
        }
    }

    pub fn rotate(&mut self, board: &Vec<Vec<bool>>) -> bool {
        let mut rotated_piece = *self;
        rotated_piece.rotation = (rotated_piece.rotation + 1) % 4;
        let matrix = rotated_piece.matrix();
        if collision(board, &matrix, self.x, self.y) {
            return false;
        }
        self.last_rotation = self.rotation;
        self.rotation = (self.rotation + 1) % 4;
        true
    }

    pub fn move_left(&mut self, board: &Vec<Vec<bool>>) -> bool {
        let matrix = self.matrix();
        if collision(board, &matrix, self.x - 1, self.y) {
            return false;
        }
        self.last_x = self.x;
        self.x -= 1;
        self.last_rotation = self.rotation;
        true
    }

    pub fn move_right(&mut self, board: &Vec<Vec<bool>>) -> bool {
        let matrix = self.matrix();
        if collision(board, &matrix, self.x + 1, self.y) {
            return false;
        }
        self.last_x = self.x;
        self.x += 1;
        self.last_rotation = self.rotation;
        true
    }

    // Check to see if a move down of this piece is possible
    pub fn move_down(&mut self, board: &Vec<Vec<bool>>) -> bool {
        let matrix = self.matrix();
        if collision(board, &matrix, self.x, self.y + 1) {
            return false;
        }
        self.last_y = self.y;
        self.y += 1;
        self.last_rotation = self.rotation;
        true
    }

    // Convert a piece to become part of the board
    pub fn settle(&mut self, board: &mut Vec<Vec<bool>>) {
        let matrix = self.matrix();
        for x in 0..matrix.len() {
            for y in 0..matrix[0].len() {
                if matrix[x][y] {
                    board[self.x + x][self.y + y] = true;
                }
            }
        }
    }

    // Move a piece back to the top
    // TODO: make a real random selection of order and use that
    // instead of hard coding the sequence.
    pub fn reset(&mut self) {
        self.kind = match self.kind {
            Tetroimino::O => Tetroimino::J,
            Tetroimino::J => Tetroimino::T,
            Tetroimino::T => Tetroimino::S,
            Tetroimino::S => Tetroimino::Z,
            Tetroimino::Z => Tetroimino::I,
            Tetroimino::I => Tetroimino::L,
            Tetroimino::L => Tetroimino::O,
        };
        self.x = 5;
        self.y = 0;
        self.update();
    }

    // Piece has been moved and printed, set last to current.
    pub fn update(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        self.last_rotation = self.rotation;
    }
}

// Where should this go?  Main?
// Given the current board, a matrix for a piece, and an x,y, see if
// the piece (matrix) has no overlap with the board.
pub fn collision(
    board: &Vec<Vec<bool>>,
    matrix: &Vec<Vec<bool>>,
    piece_x: usize,
    piece_y: usize,
) -> bool {
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            // Only look at the board if our matrix has something
            if !matrix[x][y] {
                continue;
            }
            if x + piece_x >= BOARD_WIDTH || y + piece_y >= BOARD_HEIGHT {
                return true;
            }
            if board[x + piece_x][y + piece_y] {
                return true;
            }
        }
    }
    false
}

#[allow(clippy::needless_range_loop)]
pub fn _print_matrix(m: &Vec<Vec<bool>>, show_counters: bool) {
    if show_counters {
        print!("   ");
        for x in 0..m.len() {
            print!("{:<2}", x % 10);
        }
        println!();
    }
    for y in 0..m[0].len() {
        if show_counters {
            print!("{y:<2}");
        }
        for x in 0..m.len() {
            if m[x][y] {
                print!(" #");
            } else {
                print!(" _");
            }
        }
        println!();
    }
}
