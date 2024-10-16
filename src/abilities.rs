use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use crate::abilities::Ability::{Atomic, Delayed};
use crate::cards::{Card, Faction};
use crate::star_realms::{GameState, Player};

#[derive(Clone)]
pub struct Predicate {
    description: String,
    pred: Rc<dyn Fn (GameState) -> bool>,
}

impl Debug for Predicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Predicate")
            .field("description", &self.description)
            .field("pred", &"predicate <closure>")
            .finish()
    }
}

impl Predicate {
    pub fn new(description: String, pred: Rc<dyn Fn (GameState) -> bool>) -> Predicate {
        Predicate {
            description,
            pred
        }
    }
}


pub trait AtomicAbilityTrait {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
pub enum AtomicAbilityFn {
    Default(Box<dyn Fn(GameState) -> Result<GameState, String>>),
    Card(Box<dyn Fn(GameState, Card) -> Result<GameState, String>>),
    Cards(Box<dyn Fn(GameState, Vec<Card>) -> Result<GameState, String>>),
    CardsAndHolder(Box<dyn Fn(GameState, Vec<Card>, &mut Vec<Card>) -> Result<GameState, String>>)
}
#[derive(Clone)]
pub struct AtomicAbility
{
    name: String,
    description: String,
    ability: Rc<AtomicAbilityFn>,
}

impl Debug for AtomicAbility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YourStruct")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("ability", &"<closure>")
            .finish()
    }
}

impl AtomicAbilityTrait for AtomicAbility {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }
}
#[derive(Debug, Clone)]
pub enum Ability {
    Atomic(AtomicAbility),
    And(Box<Ability>, Box<Ability>),
    Or(Box<Ability>, Box<Ability>),
    Cond(Predicate, Box<Ability>),
    Delayed(Box<Ability>),
}

#[derive(Debug, Clone)]
pub struct Abilities {
    pub on_board: Option<Ability>,
    pub on_faction: Option<Ability>,
    pub on_scrap: Option<Ability>
}

#[derive(Clone, Debug)]
pub struct AbilityFactory;

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Card::Basic(c) => {
                match other {
                    Card::Basic(o) => c.name == o.name,
                    Card::Faction(c, _) | Card::Cost(c, _) | Card::Base(c, _, _) => c.deref() == other
                }
            }
            Card::Faction(c, _) | Card::Cost(c, _) | Card::Base(c, _, _)=> c.deref() == other
        }
    }
}

impl AbilityFactory {
    fn remove_if_exists(mut from: &mut Vec<Card>, card: Card) {
        let mut to_rem: i32 = -1;
        for i in 0..from.len() {
            if from[i] == card {
                to_rem = i as i32;
            }
        }
        if to_rem >= 0 {
            from.remove(to_rem as usize);
        }
    }
    pub fn give_damages(amt: i32) -> Ability {
        Atomic(AtomicAbility {
            name: format!("Deal {amt}"),
            description: format!("Deal {amt} damages"),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut new_player: Player = gs.get_current_player();
                new_player.hand.damage += amt;
                Ok(gs.mutate_players(new_player, gs.turn%2))
            }))),
        })
    }

    pub fn give_trade(amt: i32) -> Ability {
        Atomic(AtomicAbility {
            name: format!("{amt} Trades"),
            description: format!("Gives {amt} trades"),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut new_player: Player = gs.get_current_player();
                new_player.hand.trade += amt;
                Ok(gs.mutate_players(new_player, gs.turn%2))
            }))),
        })
    }

    pub fn give_authority(amt: i32) -> Ability {
        Atomic(AtomicAbility {
            name: format!("{amt} Authority"),
            description: format!("Gives {amt} authority"),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut new_player: Player = gs.get_current_player();
                new_player.authority += amt;
                Ok(gs.mutate_players(new_player, gs.turn%2))
            }))),
        })
    }

    pub fn draw(amt: i32) -> Ability {
        Atomic(AtomicAbility {
            name: format!("{amt} Draw"),
            description: format!("Draw {amt} cards"),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut new_player: Player = gs.get_current_player();
                for _ in 0..amt {
                    new_player.draw();
                }
                Ok(gs.mutate_players(new_player, gs.turn%2))
            }))),
        })
    }

    pub fn next_ship_on_top() -> Ability {
        Atomic(AtomicAbility {
            name: "Next ship on top".to_string(),
            description: "You may put the next ship you acquire on top of your deck".to_string(),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(|gs: GameState| -> Result<GameState, String> {
                let mut current_player = gs.get_current_player();
                current_player.hand.next_n_ships_on_top += 1;
                Ok(gs.mutate_players(current_player, gs.turn%2))
            }))),
        })
    }

    pub fn destroy_target_base() -> Ability {
        Delayed(Box::new(Atomic(AtomicAbility {
            name: "Destroy target base".to_string(),
            description: "You may destroy target base".to_string(),
            ability: Rc::new(AtomicAbilityFn::Card(Box::new(move |gs: GameState, card: Card| -> Result<GameState, String> {
                let mut opponent = gs.get_opponent_player();
                Self::remove_if_exists(&mut opponent.hand.played, card.clone());
                Ok(gs.mutate_players(opponent, gs.turn%2+1))
            }))),
        })))
    }

    pub fn scrap_trade_row() -> Ability {
        Delayed(Box::new(Atomic(AtomicAbility {
            name: "Scrap a card in trade row".to_string(),
            description: "You may scrap a card in the trade ro".to_string(),
            ability: Rc::new(AtomicAbilityFn::Card(Box::new(move |mut gs: GameState, c: Card| -> Result<GameState, String> {
                gs = gs.clone();
                Self::remove_if_exists(&mut gs.trade_row, c.clone());
                Ok(gs)
            }))),
        })))
    }

    pub fn free_ship_on_top() -> Ability {
        Atomic(AtomicAbility {
            name: "Free Ship".to_string(),
            description: "Acquire any ship without payint its cost and put it on top of your deck".to_string(),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut current_player = gs.get_current_player();
                current_player.hand.next_n_ships_on_top += 1;
                current_player.hand.next_n_ships_free += 1;
                Ok(gs.mutate_players(current_player, gs.turn%2))
            }))),
        })
    }

    pub fn draw_for_each(f: Faction) -> Ability {
        Atomic(AtomicAbility {
            name: format!("Draw a card for each {f} card you've played this turn"),
            description: format!("Draw a card for each {f} card you've played this turn"),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut current_player = gs.get_current_player();
                let nb_to_draw = current_player.hand.played.iter().fold(0, |mut acc, curr| {
                   if let Card::Faction(_, cf) = curr {
                       if let Faction::Blob = cf {
                           acc += 1;
                       }
                   }
                    acc
                });
                for _ in 0..nb_to_draw {
                    current_player.draw();
                }
                Ok(gs.mutate_players(current_player, gs.turn%2))
            }))),
        })
    }

    pub fn target_discard() -> Ability {
        Atomic(AtomicAbility {
            name: "Target discard".to_string(),
            description: "Target opponent discards a card".to_string(),
            ability: Rc::new(AtomicAbilityFn::Card(Box::new(|gs: GameState, c: Card| -> Result<GameState, String> {
                let mut other_player: Player = gs.get_opponent_player();
                Self::remove_if_exists(&mut other_player.hand.playable, c);
                Ok(gs.mutate_players(other_player, (gs.turn+1)%2))
            }))),
        })
    }

    pub fn discard_n_draw_n(max: i32) -> Ability {
        Delayed(Box::new(Atomic(AtomicAbility {
            name: format!("Discard max {max} then draw as many"),
            description: format!("Discard up to {max} cards, then draw that many cards"),
            ability: Rc::new(AtomicAbilityFn::Cards(Box::new(move |gs: GameState, cs: Vec<Card>| -> Result<GameState, String> {
                let mut current_player = gs.get_current_player();
                for c in cs {
                    Self::remove_if_exists(&mut current_player.hand.playable, c.clone());
                    current_player.draw();
                }
                Ok(gs.mutate_players(current_player, gs.turn%2))
            }))),
        })))
    }

    pub fn all_ships_get(n: i32) -> Ability {
        Atomic(AtomicAbility {
            name: format!("Ships get {n}"),
            description: format!("All of your ships get {n} damages"),
            ability: Rc::new(AtomicAbilityFn::Default(Box::new(move |gs: GameState| -> Result<GameState, String> {
                let mut current_player = gs.get_current_player();
                let nb_ships = current_player.hand.get_played_ships().len();
                for _ in 0..nb_ships {
                    current_player.hand.damage += 1
                }
                Ok(gs.mutate_players(current_player, gs.turn%2))
            }))),
        })
    }
}
