use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub row: i32,
    pub column: i32,
    pub occupied: bool,
}

impl Square {
    pub fn draw(&self, size: f32, position: Vec2) {
        if self.occupied{
            draw_rectangle(
                size * self.column as f32 + position.x,
                size * self.row as f32 + position.y,
                size,
                size,
                BLACK);
        }
        draw_rectangle_lines(
            size * self.column as f32 + position.x,
            size * self.row as f32 + position.y,
            size,
            size,
            2.0,
            BLACK);
    }
}

