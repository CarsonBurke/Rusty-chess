use crate::Pos;

#[derive(Default)]
pub struct Player {
    player_type: String,
    points: i32,
    last_move: Pos,
    can_castle: bool,
}

impl Player {
    fn new() -> Self {

        return Self {
            ..Default::default()
        }
    }
}