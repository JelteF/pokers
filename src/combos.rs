use cards::Card;

// All these functions need sorted vectors of Cards
pub fn has_pair<'r>(card_vec: &'r Vec<&Card>) -> Option<&'r Card> {
    has_set(2, card_vec)
}

fn has_set<'r>(set_size: u8, card_vec: &'r Vec<&Card>) -> Option<&'r Card> {
    let mut prev: &&Card = match card_vec.last() {
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

