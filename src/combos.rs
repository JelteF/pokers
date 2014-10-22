use cards::Card;

// All these functions need sorted vectors of Cards
pub fn has_pair<'r>(card_vec: &Vec<&'r Card>) -> Option<&'r Card> {
    has_set(2, card_vec)
}

pub fn has_three_of_a_kind<'r>(card_vec: &Vec<&'r Card>) -> Option<&'r Card> {
    has_set(3, card_vec)
}

pub fn has_four_of_a_kind<'r>(card_vec: &Vec<&'r Card>) -> Option<&'r Card> {
    has_set(4, card_vec)
}

fn has_set<'r>(set_size: u8, card_vec: &Vec<&'r Card>) -> Option<&'r Card> {
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

pub fn has_two_pair<'r>(
        card_vec: &Vec<&'r Card>) -> Option<(&'r Card, &'r Card)> {
    has_sets(2, 2, card_vec)
}

pub fn has_full_house<'r>(
        card_vec: &Vec<&'r Card>) -> Option<(&'r Card, &'r Card)> {
    has_sets(3, 2, card_vec)
}

fn has_sets<'r>(set_size1: u8, set_size2: u8,
                card_vec: &Vec<&'r Card>) -> Option<(&'r Card, &'r Card)> {
    let largest = match has_set(set_size1, card_vec) {
        Some(x) => x,
        None => return None
    };
    // Remove the previous set from the set of cards
    let card_vec_rest: &Vec<&'r Card> = &card_vec.clone().into_iter()
        .filter(|&card| card != largest)
        .collect();
    // Search for the second set
    let smallest = match has_set(set_size2, card_vec_rest) {
        Some(x) => x,
        None => return None
    };
    Some((largest, smallest))
}
