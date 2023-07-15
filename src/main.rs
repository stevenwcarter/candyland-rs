mod cards;
use crate::cards::{get_card, init_cards, Card};
use yew::prelude::*;

fn card_value(card: Card) -> String {
    format!("{:?} - {:?} - {:?}", card.color, card.count, card.symbol)
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub card: Card,
}

#[function_component]
fn CardBlock(props: &CardProps) -> Html {
    let card = props.card.clone();

    if card.color.is_some() {
        if card.count == 2 {
            let color = card.color.unwrap();
            html! {
                <div class="card">
                    <div>{color.clone()} {2}</div>
                    <div class={classes!("colorbox", color.clone())} />
                    <div class={classes!("colorbox", color.clone())} />
                </div>
            }
        } else {
            let color = card.color.unwrap();
            html! {
                <div class="card">
                    <div>{color.clone()} {1}</div>
                    <div class={classes!("colorbox", color.clone())} />
                </div>
            }
        }
    } else if card.symbol.is_some() {
        let symbol = card.symbol.unwrap();
        html! {
            <div class="card">
                <img src={format!("/img/{}.jpg", symbol)} />
            </div>
        }
    } else {
        html! {
            <div class="card">
                { "get your first card" }
            </div>
        }
    }
}

#[function_component]
fn App() -> Html {
    let cards: UseStateHandle<Vec<Card>> = use_state(|| init_cards());
    let card: UseStateHandle<Card> = use_state(|| Card {
        color: None,
        count: 0,
        symbol: None,
    });
    let next_card = {
        let card = card.clone();
        let cards = cards.clone();
        Callback::from(move |_| {
            let (new_card, new_cards) = get_card((*cards).clone());
            card.set(new_card);
            cards.set(new_cards);
        })
    };

    html! {
        <div class="container">
            <button class="card-button" onclick={next_card}><CardBlock card={(*card).clone()} /></button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
