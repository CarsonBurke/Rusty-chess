use std::{collections::HashMap, future::Future};

use crate::{game::Game, neural_network::NeuralNetwork, neural_network::Output, neural_network::Input};
use async_recursion::async_recursion;
extern crate async_recursion;
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

        player_type_graphics.insert("king".to_string(), "♔".to_string());
        player_type_graphics.insert("queen".to_string(), "♕".to_string());
        player_type_graphics.insert("bishop".to_string(), "♗".to_string());
        player_type_graphics.insert("castle".to_string(), "♖".to_string());
        player_type_graphics.insert("knight".to_string(), "♘".to_string());
        player_type_graphics.insert("pawn".to_string(), "♙".to_string());

        unit_graphics.insert("black".to_string(), player_type_graphics);


        // White

        player_type_graphics = HashMap::new();

        player_type_graphics.insert("king".to_string(), "♚".to_string());
        player_type_graphics.insert("queen".to_string(), "♛".to_string());
        player_type_graphics.insert("bishop".to_string(), "♝".to_string());
        player_type_graphics.insert("castle".to_string(), "♜".to_string());
        player_type_graphics.insert("knight".to_string(), "♞".to_string());
        player_type_graphics.insert("pawn".to_string(), "♟".to_string());

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

            for (player_id, player) in &game.players {

                 // neural net

                let inputs: Vec<Input> = vec![
                    Input::new(
                        "pawn".to_string(),
                        vec![0., 0., 0., 0., 0., 0., 0., 0.],
                        vec!["0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
                    ),
                    Input::new(
                        "castle".to_string(),
                        vec![0., 0.],
                        vec!["1".to_string(), "1".to_string()],
                    ),
                    Input::new(
                        "knight".to_string(),
                        vec![0., 0.],
                        vec!["2".to_string(), "2".to_string()],
                    ),
                    Input::new(
                        "bishop".to_string(),
                        vec![0., 0.],
                        vec!["3".to_string(), "3".to_string()],
                    ),
                    Input::new(
                        "king".to_string(),
                        vec![0.],
                        vec!["4".to_string(), "4".to_string()],
                    ),
                    Input::new(
                        "queen".to_string(),
                        vec![0.],
                        vec!["5".to_string(), "5".to_string()],
                    ),
                ];
                let outputs: Vec<Output> = vec![
                    Output::new("move from x".to_string()),
                    Output::new("move from y".to_string()),
                    Output::new("move to x".to_string()),
                    Output::new("move to y".to_string()),
                ];
    
                let mut neural_network = NeuralNetwork::new();
                neural_network.build(&inputs, outputs.len());
                self.networks.insert(neural_network.id.clone(), neural_network);   
            }

            i += 1;
        }
    }

    pub fn new_id(&mut self) -> String {
        
        self.id_index += 1;
        return (&self.id_index).to_string()
    }

    pub fn reset(&mut self) {

        let mut games: Vec<(&String, &Game)> = self.games.iter().collect();
        games.sort_by(|a, b| a.1.tick.cmp(&b.1.tick));

        let mut winning_games_by_fitness: Vec<(&String, &Game)> = vec![];

        // Filter games that have winners

        for (game_id, game) in games {

            if game.winner.is_none() { continue }

            winning_games_by_fitness.push((game_id, game));
        }

        // Sort winners by ticks to spent
        
        for (game_id, game) in winning_games_by_fitness {

            println!("{:?}", game.tick);
        }

        for (id, game) in self.games.iter_mut() {
            
            /* Split up the parts of self we're trying to pass so we aren't passing self itself */
            game.reset()
        }
    }

    /**
    * Have one game running until all have been run, reset and repeat once all have been run
    */
    #[async_recursion]
    pub async fn run(&mut self) {

        for (id, game) in self.games.iter_mut() {

            let mut interval = time::interval(Duration::from_millis(self.tick_speed));
            
            loop {

                if game.tick > 500 {
                    println!("Times up");
                    break
                }

                let mut winner = &game.winner;
                if let Some(winner) = winner {
                    println!("Winner");
                    break;
                }

                // The game doesn't have a winner, run it

                game.run(self.unit_graphics.clone(), self.neural_networks);
                self.tick += 1;

                interval.tick().await;
            }
        }

        println!("End");

        self.reset();
        /* self.run().await; */
    }
}