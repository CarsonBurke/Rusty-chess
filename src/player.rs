use crate::{Pos, neural_network::NeuralNetwork, game::Game, manager::Manager};

#[derive(Default)]
pub struct Player {
    player_type: String,
    points: i32,
    last_move: Pos,
    can_castle: bool,
    network_id: String,
    game_id: String,
}

impl Player {
    fn new() -> Self {

        return Self {
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
}