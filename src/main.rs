pub mod square;
pub mod grid;
pub mod piece;

use macroquad::prelude::*;
use piece::Piece;
use crate::grid::Grid;

#[macroquad::main("tetris")]
async fn main() {
    const TIME_PER_TICK: f32 = 0.25;
    let mut time_of_last_update = 0.0;
    let mut score = 0;

    println!("Hello, world!");
    let mut the_grid = Grid::new(20, 10);
    //the_grid.print();
    //the_grid.squares.append(&mut the_piece.squares);
    //the_grid.switch_square(the_grid.get_square(1, 2));
    the_grid.add_piece();

    'main: loop {
        clear_background(WHITE);

        if get_time() as f32 - time_of_last_update > TIME_PER_TICK {
            time_of_last_update = get_time() as f32;

            if the_grid.piece == Option::None {
                the_grid.add_piece();
            } else if the_grid.is_under_piece_valid() {         
                the_grid.move_piece_down();
            } else {
                the_grid.depieceify();
            }
        }

        if is_key_pressed(KeyCode::Left) {
            the_grid.move_piece_left();
        }

        if is_key_pressed(KeyCode::Right) {
            the_grid.move_piece_right();
        }

        if is_key_pressed(KeyCode::Up) {
            the_grid.hard_drop();       
        }


        the_grid.draw();
        draw_text(&format!("Score: {}", score), 10.0, 50.0, 25.0, BLACK);
        //println!("{:#?}", the_grid);

        next_frame().await;
    }
}
