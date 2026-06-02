use crate::config::{GRID_HEIGHT, GRID_WIDTH};

use macroquad::rand;

pub fn get_around(x: u16, y: u16) -> (u16, u16) {
    let directions = [(-1i8, -1i8), (-1i8, 0i8), (-1i8, 1i8), (0i8, -1i8), (0i8, 1i8), (1i8, -1i8), (1i8, 0i8), (1i8, 1i8)];
    let direction = directions[rand::gen_range(0, directions.len())];
    let _x : u16 = x.saturating_add_signed(direction.0 as i16);
    let _y : u16 = y.saturating_add_signed(direction.1 as i16);
    ( if _x >= GRID_WIDTH { GRID_WIDTH - 1 } else { _x } , if _y >= GRID_HEIGHT { GRID_HEIGHT - 1 } else { _y })
}