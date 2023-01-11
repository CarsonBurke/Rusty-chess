use std::{collections::HashMap};

use crate::game::Game;

pub struct Manager {
    id_index: i32,
    pub tick: i32,
    pub tick_speed: i32,
    pub games: HashMap<String, Game>,
    pub games_amount: i32,
    pub games_ran: i32,
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
        }
    }
    pub fn init(&mut self) {

        let mut i = 0;
        while i < self.games_amount {

            let id = self.new_id();
            self.games.insert(id, Game::new());

            i += 1;
        }
    }
    pub fn new_id(&mut self) -> String {
        
        self.id_index += 1;
        return (&self.id_index).to_string()
    }
    pub fn reset() {


    }
    pub fn run() {

        
    }
}