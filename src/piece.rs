use crate::square::Square;

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub squares: Vec<Square>
}

impl Piece {
    pub fn new() -> Piece {
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
}