use crate::{Pos, constants::BOARD_SIZE};

pub fn are_positions_same(pos1: &Pos, pos2: &Pos) -> bool {

    if pos1.x != pos2.x { return false }
    if pos1.y != pos2.y { return false }
    return true
}

pub fn are_xy_same(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {

    if x1 != x2 { return false }
    if y1 != y2 { return false }
    return true
}

pub fn is_pos_inside_board(pos: &Pos) -> bool {
    
    return pos.x >= 0 && pos.x < BOARD_SIZE.try_into().unwrap() && pos.y >= 0 && pos.y < BOARD_SIZE.try_into().unwrap()
}