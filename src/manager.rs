use std::{collections::HashMap};

use crate::{game::Game, neural_network::NeuralNetwork, neural_network::Output, neural_network::Input};

pub struct Manager {
    id_index: i32,
    pub tick: i32,
    pub tick_speed: i32,
    pub games: HashMap<String, Game>,
    pub games_amount: i32,
    pub games_ran: i32,
    pub networks: HashMap<String, NeuralNetwork>,
}

impl Manager {
    pub fn new(games_amount: i32, tick_speed: i32) -> Self {

        return Self { 
            id_index: 0,
            tick: 0,
            tick_speed,
            games: HashMap::new(),
            games_amount,
            games_ran: 0,
            networks: HashMap::new(),
        }
    }

    pub fn init(&mut self) {

        let mut i = 0;
        while i < self.games_amount {

            // game

            let id = self.new_id();
            let game = Game::new(id.clone());
            self.games.insert(id, game);

            // neural net

            let inputs: Vec<Input> = vec![
                Input::new(
                    "x".to_string(),
                    vec![1., 3.],
                    vec!["1".to_string(), "2".to_string()],
                ),
                Input::new(
                    "y".to_string(),
                    vec![2.],
                    vec!["1".to_string()],
                ),
            ];
            let outputs: Vec<Output> = vec![
                Output::new("result".to_string()),
            ];

            let mut neural_network = NeuralNetwork::new();
            neural_network.build(&inputs, outputs.len());
            self.networks.insert(neural_network.id.clone(), neural_network);

            i += 1;
        }
    }

    pub fn new_id(&mut self) -> String {
        
        self.id_index += 1;
        return (&self.id_index).to_string()
    }

    pub fn reset(&mut self) {

        for (id, game) in self.games.iter_mut() {
            /* Split up the parts of self we're trying to pass so we aren't passing self itself */
            game.reset()
        }
    }

    /**
    * Have one game running until all have been run, reset and repeat once all have been run
    */
    pub fn run(&mut self) {

        for (id, game) in self.games.iter_mut() {
            
/*             
            if game.winner.unwrap() {

                continue
            }
 */
            // The game doesn't have a winner, run it

            game.run(self.tick_speed);
            self.tick += 1;
            break
        }
    }
}