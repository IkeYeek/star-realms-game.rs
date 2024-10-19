use std::ops::Deref;
use yew::prelude::*;
use yew::{classes, html};
use log::info;
use crate::abilities::Abilities;
use crate::abilities::AtomicAbilityFn::Cards;
use crate::cards::{Card, CardFactory, Faction};
use crate::star_realms::GameState;

mod star_realms;
mod abilities;
mod cards;
mod gamelogic;

// Structures de données

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub card: Card,
}

struct FlattenedCard {
    name: Option<String>,
    abilities: Option<Abilities>,
    faction: Option<Faction>,
    cost: Option<i32>,
    authority: Option<i32>,
    is_outpost: Option<bool>,
}

impl FlattenedCard {
    fn from_rec(c: Card, mut fc: &mut FlattenedCard) -> &mut FlattenedCard {
        match c {
            Card::Basic(c) => {
                fc.name = Some(c.name);
                fc.abilities = Some(c.abilities);
                fc
            }
            Card::Faction(c, f) => {
                fc.faction = Some(f);
                Self::from_rec(*c, fc)
            }
            Card::Cost(c, f) => {
                fc.cost = Some(f);
                Self::from_rec(*c, fc)
            }
            Card::Base(c, a, op) => {
                fc.authority = Some(a);
                fc.is_outpost = Some(op);
                Self::from_rec(*c, fc)
            }
        }
    }
    pub fn from(c: Card) -> FlattenedCard {
        let mut fc: FlattenedCard = FlattenedCard {
            name: None,
            abilities: None,
            faction: None,
            cost: None,
            authority: None,
            is_outpost: None,
        };
        Self::from_rec(c, &mut fc);
        fc
    }
}

#[derive(Properties, PartialEq)]
struct FactionProps {
    faction: Faction,
}

#[derive(Properties, PartialEq)]
struct RowProps {
    cards: Vec<Card>,
}

#[derive(Properties, PartialEq)]
struct PlayedCardsProps {
    bases: Vec<Card>,
    ships: Vec<Card>,
}

#[derive(Properties, PartialEq)]
struct PlayerDiscardProps {
    discard: Vec<Card>
}

#[derive(Properties, PartialEq)]
struct CardDeckProps {
    deck: Vec<Card>,
    visible: bool,
}

#[derive(Properties, PartialEq)]
struct AuthorityProps {
    authority: i32,
    p1: bool,
}

#[derive(Properties, PartialEq)]
struct PlayerProps {
    discard: Vec<Card>,
    deck: Vec<Card>,
    played: Vec<Card>,
    authority: i32,
    p1: bool
}

#[derive(Properties, PartialEq)]
struct CardsModalProps {
    cards: Vec<Card>
}

#[derive(Properties, PartialEq)]
struct CardModalProps {
    card: Card,
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    explorers: Vec<Card>,
    trade_row: Vec<Card>,
    trade_deck: Vec<Card>,
    scrap: Vec<Card>
}

// Composants

#[function_component]
fn FactionComponent(props: &FactionProps) -> Html {
    let faction = match props.faction {
        Faction::Blob => "BL",
        Faction::Machine => "MA",
        Faction::Star => "ST",
        Faction::Trade => "TR"
    };
    html! {
         <p>{faction}</p>
    }
}

#[function_component]
fn CardComponent(props: &CardProps) -> Html {
    let faction = props.card.get_faction();
    let cost = props.card.get_cost();
    let abilities = props.card.get_abilities();
    html! {
        <div class={classes!(
    "card",
    match props.card.get_faction() {
        Some(Faction::Blob) => "blob",
        Some(Faction::Machine) => "machine",
        Some(Faction::Trade) => "trade",
        Some(Faction::Star) => "star",
        None => "no-faction",
    }
)}>
            <div class={classes!("card-top")}>
                if let Some(faction) = faction {
                    <FactionComponent faction={faction} />
                }
                <div class={classes!("card-title")}>{ props.card.get_name() }</div>
                if let Some(cost) = cost {
                    <div class={classes!("card-price")}>{cost}</div>
                }
            </div>
            <div class={classes!("card-img")}></div>
            <div class={classes!("card-bot")}>
                <div class={classes!("ability-container")}>
                    if let Some(ability) = abilities.on_board {
                        <div class={classes!("ability", "on-play")}>
                            <span class="ability-text">{"On Play: "} { ability.to_string() }</span>
                        </div>
                    }
                    if let Some(ability) = abilities.on_faction {
                        <div class={classes!("ability", "on-faction")}>
                            <span class="ability-text">{"On Faction: "} { ability.to_string() }</span>
                        </div>
                    }
                    if let Some(ability) = abilities.on_scrap {
                        <div class={classes!("ability", "on-scrap")}>
                            <span class="ability-text">{"On Scrap: "} { ability.to_string() }</span>
                        </div>
                    }
            </div>
            </div>
        </div>
    }
}

#[function_component]
fn TradeRow(props: &RowProps) -> Html {
    html! {
       <div class="trade-row-container">
            <div class="trade-row">
                { for props.cards.iter().map(|card| html! { <div class="trade-row-elem"><CardComponent card={card.clone()} /> </div>}) }
            </div>
        </div>
    }
}

#[function_component]
fn PlayedCards(props: &PlayedCardsProps) -> Html {
    html! {
        <div class="played-cards">
            <div class="base-cards">
                { for props.bases.iter().map(|card| html! { <CardComponent card={card.clone()} /> }) }
            </div>
            <div class="ship-cards">
                { for props.ships.iter().map(|card| html! { <CardComponent card={card.clone()} /> }) }
            </div>
        </div>
    }
}

#[function_component]
fn EmptyPile() -> Html {
    html! {
        <div class={classes!("card-empty")}>
            <div class={classes!("empty-icon")}>{"X"}</div>
            <div class={classes!("empty-text")}>{"Pile Empty"}</div>
        </div>
    }
}

#[function_component]
fn PlayerDiscard(props: &PlayerDiscardProps) -> Html {
    match props.discard.clone().pop() {
        None => html! { <EmptyPile /> },
        Some(c) => html! { <CardComponent card={c.clone()} /> }
    }
}

#[function_component]
fn CardBackComponent() -> Html {
    html! {
        <div class={classes!("card-back")}>
            <div class={classes!("card-back-logo")}>{"star-realms.rs"}</div>
        </div>
    }
}

#[function_component]
fn CardDeck(props: &CardDeckProps) -> Html {
    let c = props.deck.clone().pop().clone();
    match c {
        None => html! { <EmptyPile /> },
        Some(c) => if props.visible {
            html! { <div onclick={Callback::from(|evt| {
                info!("??");
            })}><CardComponent card={c} /></div> }
        } else {
            html! { <CardBackComponent /> }
        }
    }
}

#[function_component]
fn PlayerAuthority(props: &AuthorityProps) -> Html {
    let playerName = if props.p1 { "Player 1"} else { "Player 2" };
    html! {
            <div class={classes!("authority-card")}>
                <div class={classes!("card-title")}>{"Authority"}</div>
                <div class={classes!("authority-display")}>
                    <div class={classes!("authority-icon")}>{playerName}</div>
                    <div class={classes!("authority-number")}>{props.authority}</div>
                </div>
            </div>
    }
}

#[function_component]
fn Player(props: &PlayerProps) -> Html {
    let played = props.played.clone();
    let bases = Card::filter_bases(played.clone());
    let ships = Card::filter_ships(played);
    if !props.p1 {
        html! {
            <div class={classes!("player-row")}>
                <PlayerDiscard discard={props.discard.clone()} />
                <CardDeck deck={props.deck.clone()} visible={false} />
                <PlayedCards bases={bases.clone()} ships={ships.clone()} />
                <PlayerAuthority authority={props.authority} p1={ props.p1 } />
            </div>
        }
    } else {
        html! {
            <div class={classes!("player-row-rev")}>
                <PlayerAuthority authority={props.authority}  p1={ props.p1 } />
                <PlayedCards bases={bases.clone()} ships={ships.clone()} />
                <CardDeck deck={props.deck.clone()} visible={false} />
                <PlayerDiscard discard={props.discard.clone()} />
            </div>
        }
    }
}

#[function_component]
fn CardsModal(props: &CardsModalProps) -> Html {
    html! {
        <div class={classes!("cards-modal")}>
           <div class={classes!("modal-content")}>
               <div class={classes!("trade-row-container")}>
                 <TradeRow cards={props.cards.clone()} />
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn CardModal(props: &CardModalProps) -> Html {
    html! {
        <div class={classes!("cards-modal")}>
           <div class={classes!("modal-content")}>
                 <CardComponent card={props.card.clone()} />
            </div>
        </div>
    }
}

#[function_component]
fn Board(props: &BoardProps) -> Html {
    html! {
        <div class={classes!("game-board")}>
            <CardDeck deck={props.explorers.clone()} visible={true}/>
            <TradeRow cards={props.trade_row.clone()}/>
            <CardDeck deck={props.trade_deck.clone()} visible={false}/>
            <CardDeck deck={props.scrap.clone()} visible={true}/>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let mut gs = GameState::new();
    let mut p1 = gs.players.0;
    let mut p2 = gs.players.1;

    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::mech_world());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::viper());
    p1.hand.played.push(CardFactory::mothership());

    p1.hand.played.push(CardFactory::space_station());
    p1.hand.played.push(CardFactory::space_station());
    html! {
        <div>
            <Player p1={false} discard={p1.discard.clone()} deck={p1.deck.clone()} played={p1.hand.played.clone()} authority={p1.authority.clone()} />
            <Board explorers={gs.explorers.clone()} trade_row={gs.trade_row.clone()} trade_deck={gs.trade_deck} scrap={gs.scrap.clone()} />
            <Player p1={true} discard={p1.discard.clone()} deck={p1.deck.clone()} played={p1.hand.played.clone()} authority={p1.authority.clone()} />
            //<CardsModal cards={p1.hand.played.clone()}/>
            //<CardModal card={CardFactory::scout()} />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}