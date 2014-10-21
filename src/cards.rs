use std::fmt;

pub struct Card {
    number: u8,
    suit: u8,
}

impl Card {
    // Constructor
    pub fn new (number: u8, suit: u8) -> Card {
        Card {suit:suit, number:number}
    }

    // Getters
    pub fn get_number(&self) -> u8 { self.number }
    pub fn get_suit(&self) -> u8 { self.suit }
}

// Define ordering, suits don't matter for ordering in poker
impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.number.eq(&other.number)
    }
}

impl Eq for Card { }

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering{
        self.number.cmp(&other.number)
    }
}

// Format a card as a readable string
impl fmt::Show for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let suit_str = match self.suit {
            0 => "hearts",
            1 => "diamonds",
            2 => "spades",
            3 => "clubs",
            x @ _ => {
                return write!(f, "{} is not a valid suit of a card", x)
            }
        };

        let number_str;
        let number_str_slice = match self.number {
            num @ 2 ... 10 => {
                number_str = num.to_string();
                number_str.as_slice()
            }
            11 => "Jack",
            12 => "Queen",
            13 => "King",
            14 => "Ace",
            num @ _ => {
                return write!(f, "{} is not a valid number of a card", num);
            }
        };

        write!(f, "{} of {}", number_str_slice, suit_str)
    }
}
