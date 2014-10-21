use cards::Card;

// All these functions need reverse sorted vectors of Cards
pub fn has_pair(card_vec: &Vec<&Card>) -> bool {
    has_set(2, card_vec)
}

fn has_set(set_size: u8, card_vec: &Vec<&Card>) -> bool {
    let mut prev: &Card = card_vec[0];
    let mut count: u8 = 1;
    for card in card_vec.iter().skip(1) {
        if *card == prev {
            count += 1;
            if count == set_size {
                return true
            }
            continue;
        }
        prev = *card;
        count = 1;
    }
    false
}
