use std::{collections::HashMap};

use crate::{game::Game, neural_network::NeuralNetwork};

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

            let id = self.new_id();
            let game = Game::new(self);
            self.games.insert(id, game);

            i += 1;
        }
    }

    pub fn new_id(&mut self) -> String {
        
        self.id_index += 1;
        return (&self.id_index).to_string()
    }

    pub fn reset() {

        for game in self.games/* .clone() */ {

            game.reset()
        }
    }

    /**
    * Have one game running until all have been run, reset and repeat once all have been run
    */
    pub fn run(&mut self) {

        for game in self.games/* .clone() */ {

            if game.winner {

                continue
            }

            // The game doesn't have a winner, run it

            game.run();
            break
        }
    }
}