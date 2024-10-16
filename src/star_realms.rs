use rand::prelude::SliceRandom;
use crate::abilities::Ability;
use crate::cards::Card;

#[derive(Debug, Clone)]
pub struct GameState {
    pub explorers: Vec<Card>,
    pub trade_row: Vec<Card>,
    pub trade_deck: Vec<Card>,
    pub scrap: Vec<Card>,
    pub turn: i32,
    pub players: (Player, Player)
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            explorers: vec![],
            trade_row: vec![],
            trade_deck: vec![],
            scrap: vec![],
            turn: 0,
            players: (Player::new(), Player::new()),
        }
    }

    pub fn mutate_players(&self, new_player: Player, player_id: i32) -> GameState {
        if player_id > 1 { return self.clone() }
        let new_players = if player_id == 0 {
            (new_player, self.players.1.clone())
        } else {
            (self.players.0.clone(), new_player)
        };
        GameState {
            explorers: self.explorers.clone(),
            trade_row: self.trade_row.clone(),
            trade_deck: self.trade_deck.clone(),
            scrap: self.scrap.clone(),
            turn: self.turn,
            players: new_players
        }
    }

    pub fn get_current_player(&self) -> Player {
        if self.turn % 2 == 0 {
            self.players.0.clone()
        } else {
            self.players.1.clone()
        }
    }

    pub fn get_opponent_player(&self) -> Player {
        if self.turn % 2 == 0 {
            self.players.1.clone()
        } else {
            self.players.0.clone()
        }
    }
}
#[derive(Debug, Clone)]
pub struct Hand {
    pub played: Vec<Card>,
    pub playable: Vec<Card>,
    pub abilities: Vec<Ability>,
    pub trade: i32,
    pub damage: i32,
    pub next_n_ships_on_top: i32,
    /*pub can_scrap_trade_row: i32,
    pub can_destroy_n_bases: i32,*/
    pub next_n_ships_free: i32,
    /*pub to_discard: i32,*/
    /*pub discard_to_peek: i32,*/
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            played: vec![],
            playable: vec![],
            abilities: vec![],
            trade: 0,
            damage: 0,
            next_n_ships_on_top: 0,
            /*can_scrap_trade_row: 0,
            can_destroy_n_bases: 0,*/
            next_n_ships_free: 0,
            /*to_discard: 0,*/
            /*discard_to_peek: 0,*/
        }
    }

    pub fn get_played_bases(&self) -> Vec<&Card> {
        self.played.iter().filter(|x| {
            if let Card::Base(_, _, _) = x {
                true
            } else {
                false
            }
        }).collect()
    }

    pub fn get_played_ships(&self) -> Vec<&Card> {
        self.played.iter().filter(|x| {
            match x {
                Card::Base(_, _, _) => false,
                _ => true
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub discard: Vec<Card>,
    pub deck: Vec<Card>,
    pub hand: Hand,
    pub authority: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            discard: vec![],
            deck: vec![],
            hand: Hand::new(),
            authority: 50,
        }
    }

    pub fn deck_from_discard(&mut self) {
        self.deck = self.discard.clone();
        self.discard.clear();
        let mut rng = rand::thread_rng();  // TODO find more suitable place
        self.deck.shuffle(&mut rng);
    }

    pub fn draw(&mut self) {
        if self.deck.len() == 0 {
            self.deck_from_discard();
        }
        if let Some(card) = self.deck.pop() {
            self.hand.playable.push(card);
        }
    }
}
