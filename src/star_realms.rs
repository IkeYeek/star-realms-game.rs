use rand::prelude::{SliceRandom};
use rand::thread_rng;
use crate::abilities::Ability;
use crate::cards::{Card, CardFactory};

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
    fn fill_trade_deck(&mut self) {
        // Trade Federation
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::federation_shuttle()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::cutter()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::embassy_yacht()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::freighter()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::command_ship()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::trade_escort()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::flagship()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::trading_post()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::barter_world()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::defense_center()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::central_office()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::port_of_call()));

        // Blob
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::blob_fighter()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::trade_pod()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::battle_pod()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::ram()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::blob_destroyer()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::battle_blob()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::blob_carrier()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::mothership()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::blob_wheel()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::the_hive()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::blob_world()));

        // Star Empire
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::imperial_fighter()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::imperial_frigate()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::survey_ship()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::corvette()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::battlecruiser()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::dreadnaught()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::space_station()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::recycling_station()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::war_world()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::royal_redoubt()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::fleet_hq()));

        // Machine Cult
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::trade_bot()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::missile_bot()));
        self.trade_deck.append(&mut CardFactory::n_of(3, CardFactory::supply_bot()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::battle_station()));
        self.trade_deck.append(&mut CardFactory::n_of(2, CardFactory::patrol_mech()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::stealth_needle()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::battle_mech()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::missile_mech()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::mech_world()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::brain_world()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::machine_base()));
        self.trade_deck.append(&mut CardFactory::n_of(1, CardFactory::junkyard()));
    }
    fn mix_trade_deck(&mut self) {
        let mut rng = thread_rng();
        self.trade_deck.shuffle(&mut rng);
    }
    fn from_trade_deck_to_row(&mut self) {
        let poped = self.trade_deck.pop();
        if let Some(c) = poped {
            self.trade_row.push(c);
        }
    }
    pub fn new() -> GameState {
        let mut gs = GameState {
            explorers: vec![],
            trade_row: vec![],
            trade_deck: vec![],
            scrap: vec![],
            turn: 0,
            players: (Player::new(), Player::new()),
        };
        gs.explorers.append(&mut CardFactory::n_of(10, CardFactory::explorer()));
        gs.fill_trade_deck();
        gs.mix_trade_deck();
        for _ in 0..5 {
            gs.from_trade_deck_to_row();
        }
        gs
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

    pub fn card_to_scrap(&mut self, c: Option<Card>) {
        if let Some(c) = c {
            self.scrap.push(c);
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
    pub next_n_ships_free: i32,
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
            next_n_ships_free: 0,
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
        let mut p = Player {
            discard: vec![],
            deck: vec![],
            hand: Hand::new(),
            authority: 50,
        };
        p.deck.append(&mut CardFactory::n_of(8, CardFactory::scout()));
        p.deck.append(&mut CardFactory::n_of(2, CardFactory::viper()));
        p
    }

    pub fn deck_from_discard(&mut self) {
        self.deck = self.discard.clone();
        self.discard.clear();
        self.mix_deck();
    }

    fn mix_deck(&mut self) {
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
