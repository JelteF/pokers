use cards::Card;

// All these functions need sorted vectors of Cards
pub fn has_pair(card_vec: &Vec<Card>) -> Option<Card> {
    has_set(2, card_vec)
}

pub fn has_three_of_a_kind(card_vec: &Vec<Card>) -> Option<Card> {
    has_set(3, card_vec)
}

pub fn has_four_of_a_kind(card_vec: &Vec<Card>) -> Option<Card> {
    has_set(4, card_vec)
}

fn has_set(set_size: u8, card_vec: &Vec<Card>) -> Option<Card> {
    let mut prev = match card_vec.last() {
        Some(x) => x,
        None => return None
    };

    let mut count: u8 = 1;
    for card in card_vec.iter().rev().skip(1) {
        if card == prev {
            count += 1;
            if count == set_size {
                return Some(*prev)
            }
            continue;
        }
        prev = card;
        count = 1;
    }
    None
}

pub fn has_two_pair(
        card_vec: &Vec<Card>) -> Option<(Card, Card)> {
    has_sets(2, 2, card_vec)
}

pub fn has_full_house(
        card_vec: &Vec<Card>) -> Option<(Card, Card)> {
    has_sets(3, 2, card_vec)
}

fn has_sets(set_size1: u8, set_size2: u8,
                card_vec: &Vec<Card>) -> Option<(Card, Card)> {
    let largest = match has_set(set_size1, card_vec) {
        Some(x) => x,
        None => return None
    };
    // Remove the previous set from the set of cards
    let card_vec_rest: &Vec<Card> = &card_vec.clone().into_iter()
        .filter(|&card| card != largest)
        .collect();
    // Search for the second set
    let smallest = match has_set(set_size2, card_vec_rest) {
        Some(x) => x,
        None => return None
    };
    Some((largest, smallest))
}


pub fn has_straight(card_vec: &Vec<Card>) -> Option<Card> {
    let mut new_card_vec = card_vec.clone();
    // Only keep one card of each number, this makes looping easier
    new_card_vec.dedup();

    let mut prev = match card_vec.last() {
        Some(x) => x,
        None => return None
    };

    // Make the straight A2345 possible
    if prev.get_number() == 14 {
        new_card_vec.insert(0, Card::new(1, 0))
    }

    let mut count = 1;
    for card in new_card_vec.iter().rev().skip(1) {
        if prev.get_number() - count == card.get_number() {
            count += 1;
            if count == 5 {
                return Some(*prev)
            }
            continue
        }
        prev = card;
        count = 1;
    }
    None
}
