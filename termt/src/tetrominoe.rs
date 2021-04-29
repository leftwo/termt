use crossterm::{
    cursor,
    style::{self, Colorize},
    QueueableCommand,
};
use std::error::Error;
use std::io::Write;
use std::io::Stdout;

use ndarray::prelude::*;
use ndarray::Data;

// The seven different game pieces we use.  Their official name in Tetris
// is "Tetrominoes".  Need a way to connect the enum of tet types with the
// contents of each tets 3x3 or 4x4 array.
#[derive(Debug, PartialEq, Clone)]
enum Tetrominoe {
    //    I,
    //    O,
    T,
    //    J,
    //    L,
    //    S,
    //    Z,
}


#[derive(Debug)]
pub struct Piece {
    x: u16,
    y: u16,
    rotation: u16,
    tet: Tetrominoe,
}

impl Piece {
    pub fn new() -> Self {
        Self {
            x: 5,
            y: 0,
            rotation: 0,
            tet: Tetrominoe::T,
        }
    }

    pub fn draw(&mut self, stdout: &mut Stdout, clear: bool) -> Result<(), Box<dyn Error>> {
        stdout.queue(cursor::MoveTo(self.x, self.y))?;
        if clear {
            stdout.queue(style::PrintStyledContent(" ".white()))?;
        } else {
            stdout.queue(style::PrintStyledContent("█".white()))?;
        }
        Ok(())
    }
    pub fn move_down(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        if self.y > 24 {
            return Ok(());
        }
        // Check if move down possible
        self.draw(stdout, true)?;
        self.y += 1;
        self.draw(stdout, false)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn move_left(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        if self.x == 0 {
            return Ok(());
        }
        // Check if move left possible
        self.draw(stdout, true)?;
        self.x -= 1;
        self.draw(stdout, false)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn move_right(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        if self.x > 80 {
            return Ok(());
        }
        // Check if move right possible
        // If so, clear and move, if not, then just return.
        self.draw(stdout, true)?;
        self.x += 1;
        self.draw(stdout, false)?;
        stdout.flush()?;
        Ok(())
    }
}

pub fn mat() {
    let data = array![[0, 1, 0], [1, 1, 1], [0, 0, 0]];
    let mat = [[0, 1, 0], [1, 1, 1], [0, 0, 0]];
    println!("Mat:  {:?}", mat);
    println!("Mat:  {:?}", mat[0]);
    println!("Mat:  {:?}", mat.len());

    for i in 0..mat.len() {
        println!("Mat{}:  {:?}", i, mat[i]);
    }
}
