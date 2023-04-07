use crate::{*, utils::{are_xy_same, are_positions_same, is_pos_inside_board}, constants::{KNIGHT_OFFSETS, PAWN_ATTACK_OFFSETS_BLACK, ADJACENT_OFFSETS, BISHOP_OFFSETS, CASTLE_OFFSETS}};

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
    pub last_pos: Pos,
    game_id: String,
}

/**
 * Merge impl into Game for finer control with Game.units
 */
impl Unit {
    pub fn new(manager: &mut Manager, game: &Game, pos: Pos, player_type: String, unit_type: String) -> Self {

        return Self {
            id: manager.new_id(),
            pos: pos.clone(),
            last_pos: pos.clone(),
            game_id: game.id.clone(),
            player_type: player_type,
            unit_type: unit_type,
            ..Default::default()
        }
    }
    /**
     * Consider giving as inputs to the neural network
     */
    pub fn find_pawn_moves(&self, game: &Game) -> Vec<Pos> {

        let mut moves: Vec<Pos> = vec![];

        if self.player_type == "black" {

            let mut pos = Pos {
                x: self.pos.x,
                y: self.pos.y - 1
            };
            
            // If there is no unit

            if is_pos_inside_board(&pos) && match game.find_unit_at_pos(&pos) {
                None => true,
                _ => false
            } {
                
                moves.push(pos)
            }

            pos = Pos {
                x: self.pos.x,
                y: self.pos.y - 2,
            };
            
            // We haven't moved yet and there are no units in the way

            let self_pos = self.pos.clone();

            if is_pos_inside_board(&pos) && are_positions_same(&self.pos, &self.last_pos) && 
            match game.find_unit_at_pos(&self_pos) {
                None => false,
                _ => true
            } {
                moves.push(pos);
            }
            
            for offset in PAWN_ATTACK_OFFSETS_BLACK {
                pos = Pos {
                    x: self.pos.x + offset.x,
                    y: self.pos.y + offset.y
                };

                if !is_pos_inside_board(&pos) { continue }

                let self_player_type = self.player_type.clone();
                let unit_at_pos = game.find_unit_at_pos(&pos);

                match unit_at_pos {
                    Some(unit_at_pos) => {
                        // Don't allow move on our own units

                        if unit_at_pos.player_type == self_player_type { continue }
                    },
                    _ => {},
                };
            }
            
            return moves
        }

        // White player



        return moves
    }

    fn try_offset(&self, game: &Game, offset: &Pos) -> Option<Pos> {

        let pos = Pos {
            x: self.pos.x + offset.x,
            y: self.pos.y + offset.y,
        };

        // Make sure we are in the bounds of the board

        if !is_pos_inside_board(&pos) { return None }

        let self_player_type = self.player_type.clone();
        let unit_at_pos = game.find_unit_at_pos(&pos);

        // Make sure there is no unit of our type at the pos

        match unit_at_pos {
            Some(unit_at_pos) => {
                if unit_at_pos.player_type == self_player_type { return None }
            },
            _ => {}
        }

        return Some(pos);
    }

    pub fn find_knight_moves(&self, game: &Game) -> Vec<Pos> {

        let mut moves: Vec<Pos> = vec![];

        for offset in KNIGHT_OFFSETS {
            let pos = self.try_offset(game, &offset);
            match pos {
                Some(pos) => moves.push(pos),
                _ => {},
            }
        }

        return moves
    }

    pub fn find_king_moves(&self, game: &Game) -> Vec<Pos> {

        let mut moves: Vec<Pos> = vec![];

        for offset in ADJACENT_OFFSETS {

            let pos = self.try_offset(game, &offset);
            match pos {
                Some(pos) => moves.push(pos),
                _ => {},
            }
        }

        return moves
    }

    fn find_moves_for_offset(&self, game: &Game, offsets: Vec<Pos>) -> Vec<Pos> {

        let mut moves = vec![];

        for offset in offsets {

            // Keep checking offsets until

            loop {
                let pos = self.try_offset(game, &offset);
                match pos {
                    Some(pos) => moves.push(pos),
                    _ => break,
                }
            }
        }

        return moves
    }

    pub fn find_queen_moves(&self, game: &Game) -> Vec<Pos> {

        return self.find_moves_for_offset(game, ADJACENT_OFFSETS.to_vec())
    }

    pub fn find_bishop_moves(&self, game: &Game) -> Vec<Pos> {

        return self.find_moves_for_offset(game, BISHOP_OFFSETS.to_vec())
    }

    pub fn find_castle_moves(&self, game: &Game) -> Vec<Pos> {

        return self.find_moves_for_offset(game, CASTLE_OFFSETS.to_vec())
    }

    pub fn find_moves(&self, game: &Game) -> Vec<Pos> {

        if self.unit_type == "king" {
            return self.find_king_moves(game)
        }
        if self.unit_type == "queen" {
            return self.find_queen_moves(game)
        }
        if self.unit_type == "bishop" {
            return self.find_bishop_moves(game)
        }
        if self.unit_type == "knight" {
            return self.find_knight_moves(game)
        }
        if self.unit_type == "castle" {
            return self.find_castle_moves(game)
        }
        if self.unit_type == "pawn" { 
            return self.find_pawn_moves(game)
        }

        return vec![]
    }
/* 
    pub fn can_move(&self, game: &Game, x: i32, y: i32) -> bool {

        for move_pos in self.find_moves(game) {

            if are_xy_same(move_pos.x, move_pos.y, x, y) {

                return true
            }
        }

        return false
    } */
    pub fn any_move_request(&self, game: &Game) -> Option<MoveRequest> {

        let moves = self.find_moves(game);
        if moves.len() == 0 { return None }

        return Some(MoveRequest {
            unit_id: self.id.clone(),
            pos: moves[0],
        })
    }
    pub fn move_request(&mut self, game: &Game) {


    }
    /*     
    fn game<'a>(&'a mut self, manager: &'a mut Manager) -> &Game {

        manager.games.get_mut(&self.game_id).unwrap()
    } */
}