use crate::abilities::{Ability, AtomicAbilityFn, AtomicAbilityTrait};
use crate::cards::{Card, CardFactory};
use crate::cards::Card::{Cost, Faction};
use crate::star_realms::GameState;

mod star_realms;
mod abilities;
mod cards;

fn main() {
    let mut gs = GameState::new();
    let fleet_hq = CardFactory::fleet_hq();
    let scout = CardFactory::scout();
    gs.players.0.hand.played.push(scout.clone());
    if let Card::Base(c, a, op) = fleet_hq {
        if let Faction(c, f) = *c {
            if let Cost(c, co) = *c {
                if let(Card::Basic(c)) = *c {
                    if let Some(ab) = c.abilities.on_board {
                        if let Ability::Atomic(aab) = ab {
                        }
                    }
                }
            }
        }
    }
    println!("{:#?}", gs);
}
