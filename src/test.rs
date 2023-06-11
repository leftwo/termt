#[cfg(test)]
use super::*;

#[cfg(test)]
mod test {
    use super::*;
    use crate::piece::collision;
    use piece::{Piece, Tetroimino};

    // The collision method does the hard work here, all we really
    // need to verify is that it does the correct thing and we can
    // send any shape to it
    #[test]
    fn o_board_collision() {
        // Test that the O will see the walls and floor of the
        // basic new board correctly.

        let board = new_empty_board();

        let p = Piece {
            kind: Tetroimino::O,
            x: 0,
            last_x: 0,
            y: 0,
            last_y: 0,
            rotation: 0,
            last_rotation: 0,
        };

        let mat = p.matrix();

        // Top row
        assert!(collision(&board, &mat, 0, 0));
        assert!(collision(&board, &mat, 1, 0));
        assert!(!collision(&board, &mat, 2, 0));
        assert!(!collision(&board, &mat, BOARD_WIDTH - 4, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, 0));

        // Last valid row
        assert!(collision(&board, &mat, 0, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, 1, BOARD_HEIGHT - 4));
        assert!(!collision(&board, &mat, 2, BOARD_HEIGHT - 4));
        assert!(!collision(&board, &mat, BOARD_WIDTH - 4, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, BOARD_HEIGHT - 4));

        // First invalid row
        assert!(collision(&board, &mat, 0, BOARD_HEIGHT - 3));
        assert!(collision(&board, &mat, 1, BOARD_HEIGHT - 3));
        assert!(collision(&board, &mat, 2, BOARD_HEIGHT - 3));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, BOARD_HEIGHT - 3));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, BOARD_HEIGHT - 3));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, BOARD_HEIGHT - 3));
    }

    #[test]
    fn o_object_collision() {
        // Test that the O will see an occupied point in the middle of
        // an otherwise empty board.

        let mut board = new_empty_board();

        board[5][5] = true;

        let p = Piece {
            kind: Tetroimino::O,
            x: 0,
            last_x: 0,
            y: 0,
            last_y: 0,
            rotation: 0,
            last_rotation: 0,
        };

        let mat = p.matrix();

        // Just above
        assert!(!collision(&board, &mat, 3, 3));
        assert!(!collision(&board, &mat, 4, 3));
        assert!(!collision(&board, &mat, 5, 3));
        assert!(!collision(&board, &mat, 6, 3));

        // The bottom of our piece is a collision
        assert!(!collision(&board, &mat, 3, 4));
        assert!(collision(&board, &mat, 4, 4));
        assert!(collision(&board, &mat, 5, 4));
        assert!(!collision(&board, &mat, 6, 4));

        // The top of our piece is the collision
        assert!(!collision(&board, &mat, 3, 5));
        assert!(collision(&board, &mat, 4, 5));
        assert!(collision(&board, &mat, 5, 5));
        assert!(!collision(&board, &mat, 6, 5));

        // Just below
        assert!(!collision(&board, &mat, 3, 6));
        assert!(!collision(&board, &mat, 4, 6));
        assert!(!collision(&board, &mat, 5, 6));
        assert!(!collision(&board, &mat, 6, 6));
    }

    #[test]
    fn t_rotation_0_board_collision() {
        // Test that the T at rotation 0 will see the walls and floor of the
        // basic new board correctly.
        // ___
        // ###
        // _#_

        let board = new_empty_board();

        let p = Piece {
            kind: Tetroimino::T,
            x: 0,
            last_x: 0,
            y: 0,
            last_y: 0,
            rotation: 0,
            last_rotation: 0,
        };

        let mat = p.matrix();
        piece::_print_matrix(&board, true);
        piece::_print_matrix(&mat, false);

        // Top row left side
        assert!(collision(&board, &mat, 0, 0));
        assert!(collision(&board, &mat, 1, 0));
        assert!(!collision(&board, &mat, 2, 0));
        // Top row right side
        assert!(!collision(&board, &mat, BOARD_WIDTH - 5, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 4, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, 0));

        // Last valid bottom row left side
        assert!(collision(&board, &mat, 0, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, 1, BOARD_HEIGHT - 5));
        assert!(!collision(&board, &mat, 2, BOARD_HEIGHT - 5));
        // Last valid bottom row right side
        assert!(!collision(&board, &mat, BOARD_WIDTH - 5, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 4, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, BOARD_HEIGHT - 5));

        // First invalid row left side
        assert!(collision(&board, &mat, 0, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, 1, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, 2, BOARD_HEIGHT - 4));
        // First invalid row right side
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, BOARD_HEIGHT - 4));
    }

    #[test]
    fn t_rotation_1_board_collision() {
        // Test that the T at rotation 1 will see the walls and floor of the
        // basic new board correctly.
        // _#_
        // _##
        // _#_

        let board = new_empty_board();

        let p = Piece {
            kind: Tetroimino::T,
            x: 0,
            last_x: 0,
            y: 0,
            last_y: 0,
            rotation: 1,
            last_rotation: 1,
        };

        let mat = p.matrix();
        piece::_print_matrix(&board, true);
        piece::_print_matrix(&mat, false);

        // Top row
        assert!(collision(&board, &mat, 0, 0));
        assert!(!collision(&board, &mat, 1, 0));
        assert!(!collision(&board, &mat, 2, 0));
        assert!(!collision(&board, &mat, BOARD_WIDTH - 5, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 4, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, 0));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, 0));

        // Last valid row
        assert!(collision(&board, &mat, 0, BOARD_HEIGHT - 5));
        assert!(!collision(&board, &mat, 1, BOARD_HEIGHT - 5));
        assert!(!collision(&board, &mat, 2, BOARD_HEIGHT - 5));
        assert!(!collision(&board, &mat, BOARD_WIDTH - 5, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 4, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, BOARD_HEIGHT - 5));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, BOARD_HEIGHT - 5));

        // First invalid row
        assert!(collision(&board, &mat, 0, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, 1, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, 2, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 3, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 2, BOARD_HEIGHT - 4));
        assert!(collision(&board, &mat, BOARD_WIDTH - 1, BOARD_HEIGHT - 4));
    }

    #[test]
    fn display() {
        let board = new_empty_board();
        piece::_print_matrix(&board, true);

        let mut p = Piece {
            kind: Tetroimino::S,
            x: 0,
            last_x: 0,
            y: 0,
            last_y: 0,
            rotation: 2,
            last_rotation: 2,
        };

        for r in 0..4 {
            p.rotation = r;
            let mat = p.matrix();
            piece::_print_matrix(&mat, false);
            println!();
        }
    }

    #[test]
    fn tetris_none() {
        // An empty board has no tetris.

        let board = new_empty_board();
        let ts = tetris_check(&board);
        assert!(ts.is_empty());
    }

    #[test]
    fn tetris_not_quite() {
        // Some, but not enough

        let mut board = new_empty_board();
        for x in 3..board.len() {
            board[x][3] = true;
        }

        let ts = tetris_check(&board);
        assert!(ts.is_empty());
    }

    #[test]
    fn tetris_also_not_quite() {
        // Some, but not enough

        let mut board = new_empty_board();
        for x in 2..board.len() - 3 {
            board[x][5] = true;
        }

        let ts = tetris_check(&board);
        assert!(ts.is_empty());
    }

    #[test]
    fn tetris_one() {
        // Make a new board and make one line a tetris.
        // Do this in a loop for every valid row.
        let just_for_y_board = new_empty_board();
        let y_max = just_for_y_board[0].len() - 2;
        for y in 0..y_max {
            let mut board = new_empty_board();
            for x in 0..board.len() {
                board[x][y] = true;
            }

            let ts = tetris_check(&board);
            assert_eq!(ts, vec![y]);
        }
    }

    #[test]
    fn tetris_two() {
        // Make a new board and make two lines a tetris.
        // Do this in a loop as we walk from 0 to len - 3
        let just_for_y_board = new_empty_board();
        let y_max = just_for_y_board[0].len() - 2;

        for y in 1..y_max {
            let mut board = new_empty_board();
            for x in 0..board.len() {
                board[x][y - 1] = true;
                board[x][y] = true;
            }

            piece::_print_matrix(&board, true);
            let ts = tetris_check(&board);
            assert_eq!(ts, vec![y - 1, y]);
        }
    }
    #[test]
    fn tetris_gap_two() {
        // Make a new board and make two lines a tetris, with
        // a gap of a line between them.
        let just_for_y_board = new_empty_board();
        let y_max = just_for_y_board[0].len() - 4;

        for y in 0..y_max {
            let mut board = new_empty_board();
            for x in 0..board.len() {
                board[x][y] = true;
                board[x][y + 2] = true;
            }

            let ts = tetris_check(&board);
            assert_eq!(ts, vec![y, y + 2]);
        }
    }

    #[test]
    fn tetris_gap_three() {
        // Make a new board and make three lines a tetris, with
        // a gap of a line between them.
        let just_for_y_board = new_empty_board();
        let y_max = just_for_y_board[0].len() - 6;

        for y in 0..y_max {
            let mut board = new_empty_board();
            for x in 0..board.len() {
                board[x][y] = true;
                board[x][y + 2] = true;
                board[x][y + 4] = true;
            }

            let ts = tetris_check(&board);
            assert_eq!(ts, vec![y, y + 2, y + 4]);
        }
    }

    #[test]
    fn tetris_four() {
        // Make a new board and make the last four lines a tetris

        let mut board = new_empty_board();
        let y_max = board[0].len() - 2;
        for x in 0..board.len() {
            board[x][y_max - 4] = true;
            board[x][y_max - 3] = true;
            board[x][y_max - 2] = true;
            board[x][y_max - 1] = true;
        }

        let ts = tetris_check(&board);
        assert_eq!(ts, vec![y_max - 4, y_max - 3, y_max - 2, y_max - 1]);
    }

    #[test]
    fn tetris_move_one() {
        // Make a new board and mark the last line a tetris
        // Call shift, then verify everything moved down

        let mut board = new_empty_board();
        let y_max = board[0].len() - 3;

        // Make the tetris
        let ts = vec![y_max];

        board[5][y_max - 3] = true;
        board[7][y_max - 3] = true;
        for x in 4..board.len() - 4 {
            board[x][y_max - 2] = true;
        }
        for x in 3..board.len() - 3 {
            board[x][y_max - 1] = true;
        }
        piece::_print_matrix(&board, true);
        tetris_shift(&mut board, &ts);
        piece::_print_matrix(&board, true);

        // Nothing should be in this range
        for y in 0..13 {
            for x in 2..board.len() - 2 {
                assert!(!board[x][y]);
            }
        }
        // Hacky...
        assert!(!board[2][13]);
        assert!(!board[3][13]);
        assert!(!board[4][13]);
        assert!(board[5][13]);
        assert!(!board[6][13]);
        assert!(board[7][13]);
        assert!(!board[8][13]);
        assert!(!board[9][13]);
        assert!(!board[10][13]);
        assert!(!board[11][13]);

        assert!(!board[2][14]);
        assert!(!board[3][14]);
        for x in 4..10 {
            assert!(board[x][14]);
        }
        assert!(!board[10][14]);
        assert!(!board[11][14]);

        assert!(!board[2][15]);
        for x in 3..11 {
            assert!(board[x][15]);
        }
        assert!(!board[11][15]);
    }

    #[test]
    fn tetris_move_two() {
        // Make a new board and mark the last two line a tetris
        // Call shift, then verify everything moved down

        let mut board = new_empty_board();
        let y_max = board[0].len() - 3;
        assert_eq!(y_max, 15);

        for y in 14..16 {
            for x in 2..board.len() - 2 {
                board[x][y] = true;
            }
        }
        // Make the tetris
        let ts = tetris_check(&board);
        assert_eq!(ts, [14, 15]);

        board[5][y_max - 2] = true;
        board[7][y_max - 3] = true;
        piece::_print_matrix(&board, true);
        tetris_shift(&mut board, &ts);
        piece::_print_matrix(&board, true);

        // Nothing should be in this range
        for y in 0..13 {
            for x in 2..board.len() - 2 {
                assert!(!board[x][y]);
            }
        }
        // Hacky...
        assert!(!board[5][14]);
        assert!(board[7][14]);
        assert!(board[5][15]);
    }

    #[test]
    fn tetris_move_up_one() {
        // Make a new board and mark the second to last line a
        // tetris. Call shift, then verify everything moved down

        let mut board = new_empty_board();
        assert_eq!(board[0].len(), 18);
        let t_row = 15;

        // Make the tetris
        let ts = vec![t_row];

        // Add some stuff on the line below the tetris
        for x in 4..board.len() - 3 {
            board[x][14] = true;
        }
        for x in 3..board.len() - 2 {
            board[x][13] = true;
        }
        for x in 3..board.len() - 3 {
            board[x][12] = true;
        }
        board[4][11] = true;
        board[5][11] = true;
        piece::_print_matrix(&board, true);
        tetris_shift(&mut board, &ts);
        piece::_print_matrix(&board, true);

        // Nothing should be in this range
        for y in 0..12 {
            for x in 2..board.len() - 2 {
                assert!(!board[x][y]);
            }
        }
        // Hacky...
        assert!(!board[2][12]);
        assert!(!board[3][12]);
        assert!(board[4][12]);
        assert!(board[5][12]);
        assert!(!board[6][12]);
        assert!(!board[7][12]);
        assert!(!board[8][12]);
        assert!(!board[9][12]);
        assert!(!board[10][12]);
        assert!(!board[11][12]);

        assert!(!board[2][13]);
        for x in 3..10 {
            assert!(board[x][13]);
        }
        assert!(!board[11][13]);

        assert!(!board[2][14]);
        for x in 3..11 {
            assert!(board[x][14]);
        }

        assert!(!board[2][15]);
        assert!(!board[3][15]);
        for x in 4..11 {
            assert!(board[x][15]);
        }
        assert!(!board[11][15]);
    }

    #[test]
    fn tetris_double_gap() {
        // Make a new board and mark the second to last and fourth
        // to last line a tetris . Call shift, then verify everything moved
        // down
        // Starting with:
        //    0 1 2 3 4 5 6 7 8 9 0 1 2 3
        // .. # # _ _ _ _ _ _ _ _ _ _ # # All empty rows
        // 8  # # _ _ _ _ _ _ _ _ _ _ # #
        // 9  # # _ _ _ # # # # # _ _ # #
        // 10 # # _ _ # _ # _ _ _ _ _ # #
        // 11 # # # # # # # # # # # _ # #
        // 12 # # _ _ _ _ _ _ _ _ _ _ # #
        // 13 # # _ # # # # # # # # # # #
        // 14 # # _ _ _ _ _ _ _ _ _ _ # #
        // 15 # # _ _ # _ # _ _ _ _ _ # #
        // 16 # # # # # # # # # # # # # #
        // 17 # # # # # # # # # # # # # #
        //
        // Becomes
        //    0 1 2 3 4 5 6 7 8 9 0 1 2 3
        // .. # # _ _ _ _ _ _ _ _ _ _ # # All empty rows
        // 10 # # _ _ _ _ _ _ _ _ _ _ # #
        // 11 # # _ _ _ # # # # # _ _ # #
        // 12 # # _ _ # _ # _ _ _ _ _ # #
        // 13 # # # # # # # # # # # _ # #
        // 14 # # _ # # # # # # # # # # #
        // 15 # # _ _ # _ # _ _ _ _ _ # #
        // 16 # # # # # # # # # # # # # #
        // 17 # # # # # # # # # # # # # #

        let mut board = new_empty_board();
        assert_eq!(board[0].len(), 18);

        // specify where our tetris is
        let ts = vec![12, 14];

        // This row has a "hole"
        board[4][10] = true;
        board[6][10] = true;
        for x in 5..board.len() - 4 {
            board[x][9] = true;
        }
        for x in 3..board.len() - 3 {
            board[x][11] = true;
        }
        for x in 3..board.len() - 2 {
            board[x][13] = true;
        }
        board[4][15] = true;
        board[6][15] = true;
        piece::_print_matrix(&board, true);
        tetris_shift(&mut board, &ts);
        piece::_print_matrix(&board, true);

        // Nothing should be in this range
        for y in 0..10 {
            for x in 2..board.len() - 2 {
                assert!(!board[x][y]);
            }
        }
        // Row 11
        for x in 2..5 {
            assert!(!board[x][11]);
        }
        for x in 5..10 {
            assert!(board[x][11]);
        }
        assert!(!board[10][11]);
        assert!(!board[11][11]);

        // Row 12
        assert!(board[1][12]);
        assert!(!board[2][12]);
        assert!(!board[3][12]);
        assert!(board[4][12]);
        assert!(!board[5][12]);
        assert!(board[6][12]);
        assert!(!board[7][12]);
        assert!(!board[8][12]);
        assert!(!board[9][12]);
        assert!(!board[10][12]);

        // Row 13
        assert!(!board[2][13]);
        for x in 3..11 {
            assert!(board[x][13]);
        }
        assert!(!board[11][13]);

        // Row 14
        assert!(!board[2][14]);
        for x in 3..board.len() {
            assert!(board[x][14]);
        }

        // Row 15, unchanged
        assert!(board[1][15]);
        assert!(!board[2][15]);
        assert!(!board[3][15]);
        assert!(board[4][15]);
        assert!(!board[5][15]);
        assert!(board[6][15]);
        assert!(!board[7][15]);
        assert!(!board[8][15]);
        assert!(!board[9][15]);
        assert!(!board[10][15]);
        assert!(!board[11][15]);
    }

    #[test]
    fn tetris_from_the_top() {
        // Make a new board and mark the second to last and fourth
        // to last line a tetris . Call shift, then verify everything moved
        // down
        // Starting with:
        //    0 1 2 3 4 5 6 7 8 9 0 1 2 3
        // 0  # # # # _ _ _ _ _ _ # # # #
        // 1  # # _ _ _ _ _ _ _ _ _ _ # #
        // 2  # # # _ _ _ _ _ _ _ _ _ # #
        // 3  # # _ # _ _ _ _ _ _ _ _ # #
        // 4  # # _ _ # _ _ _ _ _ _ _ # #
        // 5  # # _ _ _ # _ _ _ _ _ _ # #
        // 6  # # _ _ _ _ # _ _ _ _ _ # #
        // 7  # # _ _ _ _ _ # _ _ _ _ # #
        // 8  # # _ _ _ _ _ _ # _ _ _ # #
        // 9  # # _ _ _ _ _ _ _ # _ _ # #
        // 10 # # _ _ _ _ _ _ _ _ _ _ # #
        // 11 # # _ # # # # # # # # _ # #
        // 12 # # _ _ _ _ _ _ _ _ _ _ # #
        // 13 # # _ _ # # # # # # _ _ # #
        // 14 # # _ _ _ _ _ _ _ _ _ _ # #
        // 15 # # _ _ _ # # # _ _ _ _ # #
        // 16 # # # # # # # # # # # # # #
        // 17 # # # # # # # # # # # # # #
        // Should result in:
        //    0 1 2 3 4 5 6 7 8 9 0 1 2 3
        // 0  # # _ _ _ _ _ _ _ _ _ _ # #
        // 1  # # _ _ _ _ _ _ _ _ _ _ # #
        // 2  # # _ _ _ _ _ _ _ _ _ _ # #
        // 3  # # _ _ _ _ _ _ _ _ _ _ # #
        // 4  # # # # _ _ _ _ _ _ # # # #
        // 5  # # # _ _ _ _ _ _ _ _ _ # #
        // 6  # # _ # _ _ _ _ _ _ _ _ # #
        // 7  # # _ _ # _ _ _ _ _ _ _ # #
        // 8  # # _ _ _ # _ _ _ _ _ _ # #
        // 9  # # _ _ _ _ # _ _ _ _ _ # #
        // 10 # # _ _ _ _ _ # _ _ _ _ # #
        // 11 # # _ _ _ _ _ _ # _ _ _ # #
        // 12 # # _ _ _ _ _ _ _ # _ _ # #
        // 13 # # _ # # # # # # # # _ # #
        // 14 # # _ _ # # # # # # _ _ # #
        // 15 # # _ _ _ # # # _ _ _ _ # #
        // 16 # # # # # # # # # # # # # #
        // 17 # # # # # # # # # # # # # #

        let mut board = new_empty_board();
        assert_eq!(board[0].len(), 18);

        // specify where our tetris is
        let ts = vec![1, 10, 12, 14];

        // At the top
        board[2][0] = true;
        board[3][0] = true;
        board[10][0] = true;
        board[11][0] = true;
        // Diagional to 9
        for xy in 2..10 {
            board[xy][xy] = true;
        }
        for x in 3..board.len() - 3 {
            board[x][11] = true;
        }
        for x in 4..board.len() - 4 {
            board[x][13] = true;
        }
        board[5][15] = true;
        board[6][15] = true;
        board[7][15] = true;
        piece::_print_matrix(&board, true);
        tetris_shift(&mut board, &ts);
        piece::_print_matrix(&board, true);

        // Nothing should be in this range
        for y in 0..4 {
            for x in 2..board.len() - 2 {
                assert!(!board[x][y]);
            }
        }
        // Row 4
        assert!(board[2][4]);
        assert!(board[3][4]);
        for x in 4..9 {
            assert!(!board[x][5]);
        }
        assert!(board[10][4]);
        assert!(board[11][4]);

        // New diagional location, y 5-12
        for xy in 2..10 {
            assert!(board[xy][xy + 3]);
            // These last two should always be empty
            assert!(!board[10][xy + 3]);
            assert!(!board[11][xy + 3]);
        }

        // row 13
        assert!(!board[2][13]);
        for x in 3..11 {
            assert!(board[x][13]);
        }
        assert!(!board[11][13]);

        // Row 14
        assert!(!board[2][14]);
        assert!(!board[3][14]);
        for x in 4..9 {
            assert!(board[x][14]);
        }
        assert!(!board[10][14]);
        assert!(!board[11][14]);

        // Row 15, unchanged
        assert!(!board[2][15]);
        assert!(!board[3][15]);
        assert!(!board[4][15]);
        assert!(board[5][15]);
        assert!(board[6][15]);
        assert!(board[7][15]);
        assert!(!board[8][15]);
        assert!(!board[9][15]);
        assert!(!board[10][15]);
        assert!(!board[11][15]);
    }

    #[test]
    fn tetris_clear_one() {
        // Test that the clear will clear out the specified rows

        let mut board = new_empty_board();

        // specify where to clear
        let ts = vec![9, 10, 12, 14];

        // Fill the board
        for y in 0..board[0].len() {
            for x in 0..board.len() {
                board[x][y] = true;
            }
        }
        tetris_clear(&mut board, &ts);

        // Should be all filled
        for y in 0..9 {
            for x in 0..board.len() {
                assert!(board[x][y]);
            }
        }
        // row 9 false
        assert!(board[0][9]);
        assert!(board[1][9]);
        for x in 2..board.len() - 2 {
            assert!(!board[x][9]);
        }
        assert!(board[12][9]);
        assert!(board[12][9]);

        // row 10 false
        assert!(board[0][10]);
        assert!(board[1][10]);
        for x in 2..board.len() - 2 {
            assert!(!board[x][10]);
        }
        assert!(board[12][10]);
        assert!(board[13][10]);

        // row 11 true
        for x in 0..board.len() {
            assert!(board[x][11]);
        }
        // row 12 false
        assert!(board[0][12]);
        assert!(board[1][12]);
        for x in 2..board.len() - 2 {
            assert!(!board[x][12]);
        }
        assert!(board[12][12]);
        assert!(board[13][12]);

        // row 13 true
        assert!(board[0][13]);
        assert!(board[1][13]);
        for x in 2..board.len() - 2 {
            assert!(board[x][13]);
        }
        assert!(board[12][13]);
        assert!(board[13][13]);

        // row 14 false
        assert!(board[0][14]);
        assert!(board[1][14]);
        for x in 2..board.len() - 2 {
            assert!(!board[x][14]);
        }
        assert!(board[12][14]);
        assert!(board[13][14]);

        // row 15
        for x in 1..board.len() - 1 {
            assert!(board[x][15]);
        }
    }

    #[test]
    fn tetris_clear_first() {
        // Test that the clear will clear out the first row

        let mut board = new_empty_board();

        // specify where to clear
        let ts = vec![0];

        // Fill the board
        for y in 0..board[0].len() {
            for x in 1..board.len() {
                board[x][y] = true;
            }
        }
        piece::_print_matrix(&board, true);
        tetris_clear(&mut board, &ts);
        piece::_print_matrix(&board, true);

        // row 0 false
        assert!(board[0][0]);
        assert!(board[1][0]);
        for x in 2..board.len() - 2 {
            assert!(!board[x][0]);
        }
        assert!(board[12][0]);
        assert!(board[13][0]);

        // The rest should be all filled
        for y in 1..board[0].len() {
            for x in 0..board.len() {
                assert!(board[x][y]);
            }
        }
    }

    #[test]
    fn tetris_clear_last() {
        // Test that the clear will clear out the last valid row

        let mut board = new_empty_board();

        // specify where to clear
        let ts = vec![15];

        // Fill the board
        for y in 0..board[0].len() {
            for x in 1..board.len() {
                board[x][y] = true;
            }
        }
        tetris_clear(&mut board, &ts);

        // Should be all filled
        for y in 0..15 {
            for x in 0..board.len() {
                assert!(board[x][y]);
            }
        }
        // row 15 false
        for x in 2..board.len() - 2 {
            assert!(!board[x][15]);
        }
    }

    // Make a test that rotation will be prevented
}
