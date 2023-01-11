use std::vec;
use crate::{*, utils::are_xy_same};

pub struct Pawn {
    player_type: String,
    id: String,
    pos: Pos,
    last_pos: Pos,
}

impl Pawn {
    pub fn move_to(&mut self, x: i32, y:i32) -> bool {

        if !self.can_move(x, y) {

            return false
        }

        // Delete unit at pos if there is one

        //

        self.pos.x = x;
        self.pos.y = y;
        return true
    }
    /**
     * Consider giving as inputs to the neural network
     */
    pub fn find_moves(&mut self) -> Vec<Pos> {

        let mut moves: Vec<Pos> = vec![];

        if self.player_type == "black" {

            moves.push(Pos {
                x: self.pos.x,
                y: self.pos.y - 1,
            });
/* 
            // We haven't moved yet
            if let Some(self.lastPos) = self.lastPos {
                moves.push(Pos {
                    x: self.pos.x,
                    y: self.pos.y - 2,
                });
            }
             */
        }

        // While player



        return vec![]
    }
    pub fn can_move(&mut self, x: i32, y: i32) -> bool {

        for move_pos in self.find_moves() {

            if are_xy_same(move_pos.x, move_pos.y, x, y) {

                return true
            }
        }

        return false
    }
}