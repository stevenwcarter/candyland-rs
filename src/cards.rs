#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub color: Option<String>,
    pub count: u8,
    pub symbol: Option<String>,
}

pub fn init_cards() -> Vec<Card> {
    let colors: Vec<&str> = vec!["red", "yellow", "green", "blue", "purple", "orange"];
    let symbols: Vec<&str> = vec!["lollipop", "cone", "peppermint", "gumdrop", "fudge"];

    let mut cards: Vec<Card> = Vec::new();

    for color in colors {
        for _ in 0..4 {
            cards.push(Card {
                color: Some(color.to_string()),
                count: 1,
                symbol: None,
            });
        }
        for _ in 0..3 {
            cards.push(Card {
                color: Some(color.to_string()),
                count: 2,
                symbol: None,
            });
        }
    }
    for symbol in symbols {
        cards.push(Card {
            color: None,
            count: 1,
            symbol: Some(symbol.to_string()),
        });
    }

    shuffle_deck(&mut cards);

    cards
}

fn shuffle_deck<T>(vector: &mut Vec<T>) {
    // Get the length of the vector.
    let length = vector.len();

    // Loop through the vector.
    for i in 0..length {
        // Get a random index between 0 and the current index.
        let random_index = rand::random::<usize>() % (length - i);

        // Swap the current element with the element at the random index.
        vector.swap(i, random_index);
    }
}

pub fn get_card(in_cards: Vec<Card>) -> (Card, Vec<Card>) {
    let mut cards = in_cards;
    if cards.is_empty() {
        cards = init_cards();
    }

    (cards.pop().unwrap(), cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_a_deck() {
        let deck = init_cards();
        assert_eq!(deck.len(), 47);
        println!("{:#?}", deck);
    }
}
