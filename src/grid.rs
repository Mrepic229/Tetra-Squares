use macroquad::prelude::*;
use crate::square::Square;
use crate::piece::Piece;

const GRID_SIZE: f32 = 25.0;

#[derive(Clone, Debug)]
pub struct Grid {
    pub squares: Vec<Square>,
    pub piece: Option<Piece>,
    pub size: f32,
    pub rows: i32,
    pub columns: i32,
    pub position: Vec2,
}

impl Grid {
    pub fn new(rows: i32, columns: i32) -> Grid {
        let mut squares: Vec<Square> = Vec::new();
        for i in 0..rows {
            for j in 0..columns {
                squares.push(Square {
                    row: i,
                    column: j,
                    occupied: false
                })
            }
        }
        Grid {
            size: GRID_SIZE,
            piece: None,
            rows: rows,
            columns: columns,
            squares: squares,
            position: Vec2 { x: 100.0, y: 20.0 }
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn draw(&self) {
        for i in self.squares.clone().into_iter() {
            i.draw(self.size, self.position);
        }
        match &self.piece {
            Some(some_piece) => {
                for i in 0..some_piece.squares.len() {
                    some_piece.squares[i].draw(self.size, self.position)
                }
            },
            None => {},
        }
    }

    pub fn get_square(&self, row: i32, column: i32) -> usize {
        let multiplied = row * self.columns;
        let result = multiplied + column;

        return result as usize;
    }

    pub fn switch_square(&mut self, square_index: usize) {
        if !self.squares[square_index].occupied {
            self.squares[square_index].occupied = !self.squares[square_index].occupied;
        } else {
            panic!("try to switch squares of a switched square");
        }
    }

    pub fn add_piece(&mut self) {
        match self.piece {
            Some(_) => panic!("AAAAA"),
            None => {
                self.piece = Some(Piece::new())
            },
        }
    }

    pub fn is_under_piece_valid(&mut self) -> bool {
        let a_clone = self.clone();
        match self.piece {
            None => { println!("called is_under_piece_valid() without piece"); return false;},
            Some(ref mut some_piece) => {
                for i in 0..some_piece.squares.len() {
                    if !a_clone.is_square_empty(a_clone.get_square(some_piece.squares[i].row+1,some_piece.squares[i].column)){
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn is_left_piece_valid(&mut self) -> bool {
        let a_clone = self.clone();
        match self.piece {
            None => { println!("called is_left_piece_valid() without piece"); return false;},
            Some(ref mut some_piece) => {
                for i in 0..some_piece.squares.len() {
                    let target_index = a_clone.get_square(some_piece.squares[i].row,some_piece.squares[i].column-1);
                    if !a_clone.is_square_empty(target_index){
                        return false;
                    }
                    if a_clone.squares[target_index].row != some_piece.squares[i].row {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn is_right_piece_valid(&mut self) -> bool {
        let a_clone = self.clone();
        match self.piece {
            None => { println!("called is_right_piece_valid() without piece"); return false;},
            Some(ref mut some_piece) => {
                for i in 0..some_piece.squares.len() {
                    let target_index = a_clone.get_square(some_piece.squares[i].row,some_piece.squares[i].column+1);
                    if !a_clone.is_square_empty(target_index){
                        return false;
                    }
                    if a_clone.squares[target_index].row != some_piece.squares[i].row {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn is_square_empty(&self, index: usize) -> bool {
        if self.squares.len() <= index {
            return false;
        }
        if self.squares[index].occupied {
            return false;
        }
        return true;
    }

    pub fn depieceify(&mut self) {
        let a_clone = self.clone();
        match a_clone.piece {
            None => {panic!("depieceify() is called with grid.piece == Option::None")},
            Some(ref some_piece) => {
                for i in 0..some_piece.squares.len() {
                    self.switch_square(self.get_square(some_piece.squares[i].row, some_piece.squares[i].column));
                }
                self.piece = None;
            },
        }
    }

    pub fn hard_drop(&mut self) {
        println!("only print once");
        match self.piece {
            None => {println!("invalid hard-drop")},
            Some(ref mut some_piece) => {
                self.repeat_move_down();
            },
        }  
    }

    fn repeat_move_down(&mut self) {
        //println!("aaaa");
        for i in 0..100 {
            if self.is_under_piece_valid() {
                self.move_piece_down();
            } else {
                return;
            }
        }
        panic!("Somehow repeatedly moved piece down 100 times");
    }
    
    pub fn move_piece_down(&mut self) {
        if self.is_under_piece_valid() {
            match self.piece {
                Some(ref mut some_piece) => some_piece.move_down(),
                None => {},
            }
        } else {
            //self.depieceify();
        }
    }

    pub fn move_piece_right(&mut self) {
        if self.is_right_piece_valid() {
            match self.piece {
                Some(ref mut some_piece) => some_piece.move_right(),
                None => {},
            }
        }
    }

    pub fn move_piece_left(&mut self) {
        if self.is_left_piece_valid() {
            match self.piece {
                Some(ref mut some_piece) => some_piece.move_left(),
                None => {},
            }
        }
    }

}