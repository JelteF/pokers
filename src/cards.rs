pub struct Card {
    number: u8,
    suit: u8,
}

impl Card {
    pub fn new (number: u8, suit: u8) -> Card {
        Card {suit:suit, number:number}
    }
}

pub fn print_card(card: &Card) {
    let suit_str = match card.suit {
        0 => "hearts",
        1 => "diamonds",
        2 => "spades",
        3 => "clubs",
        x @ _ => {
            println!("{} is not a valid suit of a card", x);
            return
        }
    };

    let number_str;
    let number_str_slice = match card.number {
        num @ 2 ... 10 => {
            number_str = num.to_string();
            number_str.as_slice()
        }
        11 => "Jack",
        12 => "Queen",
        13 => "King",
        14 => "Ace",
        num @ _ => {
            println!("{} is not a valid number of a card", num);
            return
        }
    };

    println!("{} of {}", number_str_slice, suit_str)
}
