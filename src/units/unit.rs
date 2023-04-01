use crate::{*, utils::{are_xy_same, are_positions_same}};



/**
 * Only unit for all types
 */
#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
    pub player_type: String,
    pub unit_type: String,
    pub id: String,
    pub pos: Pos,
    last_pos: Pos,
    game_id: String,
}

impl Unit {
    pub fn new(manager: &mut Manager, game: &mut Game, pos: Pos) -> Self {

        return Self {
            id: manager.new_id(),
            pos: pos.clone(),
            last_pos: pos.clone(),
            game_id: game.id.clone(),
            ..Default::default()
        }
    }
    pub fn move_to(&mut self, manager: &mut Manager, x: i32, y:i32) -> bool {

        if !self.can_move(manager, x, y) {

            return false
        }

        // Delete unit at pos if there is one

        self.pos.x = x;
        self.pos.y = y;
        return true
    }
    /**
     * Consider giving as inputs to the neural network
     */
    pub fn find_pawn_moves(&mut self, manager: &mut Manager) -> Vec<Pos> {

        let mut moves: Vec<Pos> = vec![];

        if self.player_type == "black" {

            moves.push(Pos {
                x: self.pos.x,
                y: self.pos.y - 1,
            });

            // We haven't moved yet and there are no units in the way

            let pos = self.pos.clone();

            if are_positions_same(&self.pos, &self.last_pos) && 
            match self.game(manager).find_unit_at_pos(&pos) {
                None => false,
                _ => true
            } {
                moves.push(Pos {
                    x: self.pos.x,
                    y: self.pos.y - 2,
                });
            }
            
        }

        // While player



        return vec![]
    }

    pub fn find_moves(&mut self, manager: &mut Manager) -> Vec<Pos> {

        if self.unit_type == "pawn" { 
            return self.find_pawn_moves(manager)
        }
        if self.unit_type == "knight" {
            return self.find_pawn_moves(manager)
        }

        return vec![]
        /* else if self.unit_type == "knight" {
            return self.find_pawn_moves(manager)
        }
        else {
            return vec![]
        } */
    }

    pub fn can_move(&mut self, manager: &mut Manager, x: i32, y: i32) -> bool {

        for move_pos in self.find_moves(manager) {

            if are_xy_same(move_pos.x, move_pos.y, x, y) {

                return true
            }
        }

        return false
    }
    fn game<'a>(&'a mut self, manager: &'a mut Manager) -> &mut Game {

        manager.games.get_mut(&self.game_id).unwrap()
    }
}