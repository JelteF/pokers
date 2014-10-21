use cards::Card;

// All these functions need reverse sorted vectors of Cards
pub fn has_pair(card_vec: &Vec<&Card>) -> bool {
    let mut prev: &Card = card_vec[0];
    for card in card_vec.iter().skip(1) {
        if *card == prev {
            return true;
        }
        prev = *card
    }
    return false;
}
