use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::abilities::{Abilities, Ability, AbilityFactory, Predicate};
use crate::abilities::Ability::{And, Atomic, Cond, Or};
use crate::cards::Faction::{Blob, Machine, Star, Trade};
use crate::star_realms::GameState;

#[derive(Debug, Clone)]
pub enum Faction {
    Blob,
    Machine,
    Star,
    Trade
}

impl Display for Faction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Blob => write!(f, "Blob"),
            Machine => write!(f, "Machine Cult"),
            Star => write!(f, "Star Empire"),
            Trade => write!(f, "Trade Federation")
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicCard {
    pub name: String,
    pub abilities: Abilities,
}

#[derive(Debug, Clone)]
pub enum Card {
    Basic(BasicCard),
    Faction(Box<Card>, Faction),
    Cost(Box<Card>, i32),
    Base(Box<Card>, i32, bool),
}

#[derive(Debug, Clone)]
pub struct CardFactory;

impl CardFactory {
    fn basic(name: String, abilities: Abilities) -> Card {
        Card::Basic(BasicCard {
            name,
            abilities,
        })
    }

    fn cost(name: String, abilities: Abilities, cost: i32) -> Card {
        Card::Cost(Box::new(Self::basic(name, abilities)), cost)
    }

    fn faction(name: String, abilities: Abilities, faction: Faction, cost: Option<i32>) -> Card {
        match cost {
            None => Card::Faction(Box::new(Self::basic(name, abilities)), faction),
            Some(cost) => Card::Faction(Box::new(Self::cost(name, abilities, cost)), faction)
        }
    }

    fn base(name: String, abilities: Abilities, faction: Option<Faction>, cost: Option<i32>, authority: i32, outpost: bool ) -> Card {
        match faction {
            Some(faction) => Card::Base(
                Box::new(Self::faction(name, abilities, faction, cost)),
                authority,
                outpost
            ),
            None => Card::Base(Box::new(Self::basic(name, abilities)), authority, outpost)
        }
    }

    // Basic cards
    pub fn viper() -> Card {
        Self::basic("Viper".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(1)),
            on_faction: None,
            on_scrap: None,
        })
    }

    pub fn scout() -> Card {
        Self::basic("Scout".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_trade(1)),
            on_faction: None,
            on_scrap: None,
        })
    }

    pub fn explorer() -> Card {
        Self::cost("Explorer".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_trade(2)),
            on_faction: None,
            on_scrap: None,
        }, 2)
    }

    // Faction cards (base game)
    // Trade Federation
    pub fn federation_shuttle() -> Card {
        Self::faction("Federation Shuttle".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_trade(2)),
            on_faction: Some(AbilityFactory::give_authority(4)),
            on_scrap: None,
        }, Trade, Some(1))
    }

    pub fn cutter() -> Card {
        Self::faction("Cutter".to_string(), Abilities {
            on_board: Some(And(Box::new(AbilityFactory::give_authority(4)), Box::new(AbilityFactory::give_trade(2)))),
            on_faction: Some(AbilityFactory::give_trade(4)),
            on_scrap: None,
        }, Trade, Some(2))
    }

    pub fn embassy_yacht() -> Card {
        Self::faction("Embassy Yacht".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_authority(3)),
                Box::new(And(
                    Box::new(AbilityFactory::give_trade(2)),
                    Box::new(Cond(
                        Predicate::new(
                            "If you have two or more bases in play, draw two cards".to_string(),
                            Rc::new(|gs: GameState| {
                                let p = gs.get_current_player();
                                let played_bases = p.hand.get_played_bases();
                                played_bases.len() >= 2
                            })
                        ),
                        Box::new(AbilityFactory::draw(2))
                    ))
                ))
            )),
            on_faction: None,
            on_scrap: None,
        }, Trade, Some(3))
    }

    pub fn freighter() -> Card {
        Self::faction("Freighter".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_trade(4)),
            on_faction: Some(AbilityFactory::next_ship_on_top()),
            on_scrap: None,
        }, Trade, Some(4))
    }

    pub fn command_ship() -> Card {
        Self::faction("Command Ship".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_authority(4)),
                Box::new(And(
                    Box::new(AbilityFactory::give_damages(5)),
                    Box::new(AbilityFactory::draw(2))
                ))
            )),
            on_faction: Some(AbilityFactory::destroy_target_base()),
            on_scrap: None,
        }, Trade, Some(8))
    }

    pub fn trade_escort() -> Card {
        Self::faction("Trade Escort".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_authority(4)),
                Box::new(AbilityFactory::give_damages(4))
            )),
            on_faction: Some(AbilityFactory::draw(1)),
            on_scrap: None,
        }, Trade, Some(5))
    }

    pub fn flagship() -> Card {
        Self::faction("Flagship".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(5)),
                Box::new(AbilityFactory::draw(1))
            )),
            on_faction: Some(AbilityFactory::give_authority(5)),
            on_scrap: None,
        }, Trade, Some(6))
    }

    pub fn trading_post() -> Card {
        Self::base("Trading Post".to_string(), Abilities {
            on_board: Some(Or(Box::new(AbilityFactory::give_authority(1)), Box::new(AbilityFactory::give_trade(1)))),
            on_faction: None,
            on_scrap: Some(AbilityFactory::give_damages(3))
        }, Some(Trade), Some(6), 4, true)
    }

    pub fn barter_world() -> Card {
        Self::base("Barter World".to_string(), Abilities {
            on_board: Some(Or(Box::new(AbilityFactory::give_authority(2)), Box::new(AbilityFactory::give_trade(2)))),
            on_faction: None,
            on_scrap: Some(AbilityFactory::give_damages(5))
        }, Some(Trade), Some(4), 4, false)
    }

    pub fn defense_center() -> Card {
        Self::base("Defense Center".to_string(), Abilities {
            on_board: Some(Or(Box::new(AbilityFactory::give_authority(3)), Box::new(AbilityFactory::give_damages(2)))),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: None,
        }, Some(Trade), Some(5), 5, true)
    }

    pub fn central_office() -> Card {
        Self::base("Central Office".to_string(), Abilities {
            on_board: Some(And(Box::new(AbilityFactory::give_trade(2)), Box::new(AbilityFactory::next_ship_on_top()))),
            on_faction: Some(AbilityFactory::draw(1)),
            on_scrap: None,
        }, Some(Trade), Some(7), 6, false)
    }

    pub fn port_of_call() -> Card {
        Self::base("Port Of Call".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_trade(3)),
            on_faction: None,
            on_scrap: Some(And(Box::new(AbilityFactory::draw(1)), Box::new(AbilityFactory::destroy_target_base()))),
        }, Some(Trade), Some(6), 6, true)
    }
    // Blob

    pub fn blob_fighter() -> Card {
        Self::faction("Blob Fighter".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(3)),
            on_faction: Some(AbilityFactory::draw(1)),
            on_scrap: None
        }, Blob, Some(1))
    }

    pub fn trade_pod() -> Card {
        Self::faction("Trade Pod".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_trade(3)),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: None
        }, Blob, Some(2))
    }

    pub fn battle_pod() -> Card {
        Self::faction("Battle Pod".to_string(), Abilities {
            on_board: Some(And(Box::new(AbilityFactory::give_damages(4)), Box::new(AbilityFactory::scrap_trade_row()))),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: None
        }, Blob, Some(2))
    }

    pub fn ram() -> Card {
        Self::faction("Ram".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(5)),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: Some(AbilityFactory::give_trade(3))
        }, Blob, Some(3))
    }

    pub fn blob_destroyer() -> Card {
        Self::faction("Blob Destroyer".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(6)),
            on_faction: Some(And(
                Box::new(AbilityFactory::destroy_target_base()),
                Box::new(AbilityFactory::scrap_trade_row())
            )),
            on_scrap: None
        }, Blob, Some(4))
    }

    pub fn battle_blob() -> Card {
        Self::faction("Battle Blob".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(8)),
            on_faction: Some(AbilityFactory::draw(1)),
            on_scrap: Some(AbilityFactory::give_damages(4)),
        }, Blob, Some(6))
    }

    pub fn blob_carrier() -> Card {
        Self::faction("Blob Carrier".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(7)),
            on_faction: Some(AbilityFactory::free_ship_on_top()),
            on_scrap: None
        }, Blob, Some(6))
    }

    pub fn mothership() -> Card {
        Self::faction("Mothership".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(6)),
                Box::new(AbilityFactory::draw(1))
            )),
            on_faction: Some(AbilityFactory::draw(1)),
            on_scrap: None
        }, Blob, Some(7))
    }

    pub fn blob_wheel() -> Card {
        Self::base("Blob Wheel".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(1)),
            on_faction: None,
            on_scrap: Some(AbilityFactory::give_trade(3))
        }, Some(Blob), Some(3), 5, false)
    }

    pub fn the_hive() -> Card {
        Self::base("The Hive".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(3)),
            on_faction: Some(AbilityFactory::draw(1)),
            on_scrap: None
        }, Some(Blob), Some(5), 5, false)
    }

    pub fn blob_world() -> Card {
        Self::base("Blob World".to_string(), Abilities {
            on_board: Some(Or(
                Box::new(AbilityFactory::give_damages(5)),
                Box::new(AbilityFactory::draw_for_each(Blob))
            )),
            on_faction: None,
            on_scrap: None
        }, Some(Blob), Some(8), 7, false)
    }
    // Star Empire
    pub fn imperial_fighter() -> Card {
        Self::faction("Imperial Fighter".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(2)),
                Box::new(AbilityFactory::target_discard()),
            )),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: None,
        }, Star, Some(1))
    }

    pub fn imperial_frigate() -> Card {
        Self::faction("Imperial Frigate".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(4)),
                Box::new(AbilityFactory::target_discard()),
            )),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: Some(AbilityFactory::draw(1)),
        }, Star, Some(3))
    }

    pub fn survey_ship() -> Card {
        Self::faction("Survey Ship".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_trade(1)),
                Box::new(AbilityFactory::draw(1)),
            )),
            on_faction: None,
            on_scrap: Some(AbilityFactory::target_discard()),
        }, Star, Some(3))
    }

    pub fn corvette() -> Card {
        Self::faction("Corvette".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(1)),
                Box::new(AbilityFactory::draw(1))
            )),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: None
        }, Star, Some(2))
    }

    pub fn battlecruiser() -> Card {
        Self::faction("Battlecruiser".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(5)),
                Box::new(AbilityFactory::draw(1))
            )),
            on_faction: Some(AbilityFactory::target_discard()),
            on_scrap: Some(And(
                Box::new(AbilityFactory::draw(1)),
                Box::new(AbilityFactory::destroy_target_base())
            ))
        }, Star, Some(6))
    }

    pub fn dreadnaught() -> Card {
        Self::faction("Dreadnaught".to_string(), Abilities {
            on_board: Some(And(
                Box::new(AbilityFactory::give_damages(7)),
                Box::new(AbilityFactory::draw(1))
            )),
            on_faction: None,
            on_scrap: Some(AbilityFactory::give_damages(5))
        }, Star, Some(7))
    }

    pub fn space_station() -> Card {
        Self::base("Space Station".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(2)),
            on_faction: Some(AbilityFactory::give_damages(2)),
            on_scrap: Some(AbilityFactory::give_trade(4))
        }, Some(Star), Some(4), 4, true)
    }

    pub fn recycling_station() -> Card {
        Self::base("Recycling Station".to_string(), Abilities {
            on_board: Some(Or(
                Box::new(AbilityFactory::give_trade(1)),
                Box::new(AbilityFactory::discard_n_draw_n(2))
            )),
            on_faction: None,
            on_scrap: None
        }, Some(Trade), Some(4), 4, true)
    }

    pub fn war_world() -> Card {
        Self::base("War World".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(3)),
            on_faction: Some(AbilityFactory::give_damages(4)),
            on_scrap: None
        }, Some(Trade), Some(5), 4, true)
    }

    pub fn royal_redoubt() -> Card {
        Self::base("Royal Redoubt".to_string(), Abilities {
            on_board: Some(AbilityFactory::give_damages(3)),
            on_faction: Some(AbilityFactory::target_discard()),
            on_scrap: None
        }, Some(Trade), Some(5), 6, true)
    }

    pub fn fleet_hq() -> Card {
        Self::base("Fleet HQ".to_string(), Abilities {
            on_board: Some(AbilityFactory::all_ships_get(1)),
            on_faction: None,
            on_scrap: None
        }, Some(Trade), Some(5), 8, false)
    }

    // Machine Cult
}
