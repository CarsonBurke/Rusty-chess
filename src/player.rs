use crate::{Pos, neural_network::NeuralNetwork, game::Game, manager::Manager};

#[derive(Default, Debug)]
pub struct Player {
    pub player_type: String,
    pub points: i32,
    /**
     * Wether the player moves on even ticks if true, and if false, odd
     */
    pub turn_mod: bool,
    pub can_castle: bool,
    pub network_id: String,
    pub game_id: String,
}

impl Player {
    pub fn new(player_type: String, turn_mod: bool) -> Self {

        return Self {
            player_type: player_type,
            turn_mod: turn_mod,
            ..Default::default()
        }
    }
    fn game<'a>(&'a mut self, manager: &'a mut Manager) -> &mut Game {

        manager.games.get_mut(&self.game_id).unwrap()
    }
    fn network<'a>(&'a mut self, manager: &'a mut Manager) -> &mut NeuralNetwork {

        manager.networks.get_mut(&self.network_id).unwrap()
    }
    fn run() {


    }
    fn delete() {

        
    }
}