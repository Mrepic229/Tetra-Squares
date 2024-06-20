use crate::square::Square;
use fastrand::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub squares: Vec<Square>
}

impl Piece {
    pub fn new() -> Piece {
        let random = fastrand::i8(0..4);
        let mut squares: Vec<Square> = Vec::new();
        for i in 0..4 {
            squares.push(Square{
                row: 0,
                column: i,
                occupied: true,
            });
        }
        Piece { squares: squares }
    }

    pub fn move_down(&mut self) {
        for i in 0..self.squares.len() {
            self.squares[i].row += 1;
        }
    }

    pub fn move_left(&mut self) {
        for i in 0..self.squares.len() {
            self.squares[i].column += -1;
        }
    }

    pub fn move_right(&mut self) {
        for i in 0..self.squares.len() {
            self.squares[i].column += 1;
        }
    }

    fn new_i_block() -> Piece {
        Piece{
            squares: [
                Square {
                    row: 0,
                    column: 0,
                    occupied: true,
                },
                Square {
                    row: 0,
                    column: 1,
                    occupied: true,
                },
                Square {
                    row: 0,
                    column: 2,
                    occupied: true,
                },
                Square {
                    row: 0,
                    column: 3,
                    occupied: true,
                }
            ].to_vec(),
        }
    }

    fn new_t_block() -> Piece {
        Piece{
            squares: [
                Square {
                    row: 0,
                    column: 1,
                    occupied: true,
                },
                Square {
                    row: 1,
                    column: 0,
                    occupied: true,
                },
                Square {
                    row: 1,
                    column: 1,
                    occupied: true,
                },
                Square {
                    row: 1,
                    column: 2,
                    occupied: true,
                }
            ].to_vec(),
        }
    }
    
    fn new_j_block() -> Piece {
        Piece{
            squares: [
                Square {
                    row: 0,
                    column: 0,
                    occupied: true,
                },
                Square {
                    row: 0,
                    column: 1,
                    occupied: true,
                },
                Square {
                    row: 0,
                    column: 2,
                    occupied: true,
                },
                Square {
                    row: 0,
                    column: 3,
                    occupied: true,
                }
            ].to_vec(),
        }
    }
}