use crate::{*};

#[derive(Debug)]
pub struct Unit {
    player_type: String,
    id: String,
    pos: Pos,
    last_pos: Pos,
}