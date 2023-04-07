use std::{collections::HashMap};

use crate::{game::Game, neural_network::NeuralNetwork, neural_network::Output, neural_network::Input};
/* 
use eventual::Timer;
extern crate eventual;
 */

use tokio::time::{self, Duration};
extern crate tokio;

pub struct Manager {
    id_index: u64,
    pub tick: u64,
    pub tick_speed: u64,
    pub games: HashMap<String, Game>,
    pub games_amount: u32,
    pub games_ran: u64,
    pub networks: HashMap<String, NeuralNetwork>,
    pub unit_graphics: HashMap<String, HashMap<String, String>>,
}

impl Manager {
    pub fn new(games_amount: u32, tick_speed: u64) -> Self {

        let mut unit_graphics: HashMap<String, HashMap<String, String>> = HashMap::new();

        // Black
        
        let mut player_type_graphics: HashMap<String, String> = HashMap::new();
        player_type_graphics.insert("king".to_string(), "♚".to_string());
        player_type_graphics.insert("queen".to_string(), "♛".to_string());
        player_type_graphics.insert("bishop".to_string(), "♝".to_string());
        player_type_graphics.insert("castle".to_string(), "♜".to_string());
        player_type_graphics.insert("knight".to_string(), "♞".to_string());
        player_type_graphics.insert("pawn".to_string(), "♟".to_string());

        unit_graphics.insert("black".to_string(), player_type_graphics);

        // White

        player_type_graphics = HashMap::new();
        player_type_graphics.insert("king".to_string(), "♔".to_string());
        player_type_graphics.insert("queen".to_string(), "♕".to_string());
        player_type_graphics.insert("bishop".to_string(), "♗".to_string());
        player_type_graphics.insert("castle".to_string(), "♖".to_string());
        player_type_graphics.insert("knight".to_string(), "♘".to_string());
        player_type_graphics.insert("pawn".to_string(), "♙".to_string());

        unit_graphics.insert("white".to_string(), player_type_graphics);

        // Return construction

        return Self { 
            id_index: 0,
            tick: 0,
            tick_speed,
            games: HashMap::new(),
            games_amount,
            games_ran: 0,
            networks: HashMap::new(),
            unit_graphics: unit_graphics,
        }
    }

    pub fn init(&mut self) {

        let mut i = 0;
        while i < self.games_amount {

            // game

            let id = self.new_id();
            let mut game = Game::new(id.clone());
            game.init(self);
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
    pub async fn run(&mut self) {

        for (id, game) in self.games.iter_mut() {

            let mut interval = time::interval(Duration::from_millis(self.tick_speed));
            
            loop {

                let mut winner = &game.winner;
                if let Some(winner) = winner {

                    break;
                }

                // The game doesn't have a winner, run it

                game.run(self.unit_graphics.clone());
                self.tick += 1;

                interval.tick().await;
            }
        }

        println!("End")
    }
}