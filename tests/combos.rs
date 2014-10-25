extern crate pokers;
use pokers::cards::Card;
use pokers::combos;

#[test]
fn correct_pair() {
    let mut card_vec = Card::new_card_vec([(8, 2), (7, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_pair(&card_vec);
    assert_eq!(card, Some(Card::new(8, 2)));
}
#[test]
fn incorrect_pair() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn correct_two_pair() {
    let mut card_vec = Card::new_card_vec([(8, 2), (6, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn incorrect_two_pair() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn correct_three_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (6, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn incorrect_three_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn correct_full_house() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (6, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn incorrect_full_house() {
    let mut card_vec = Card::new_card_vec([(7, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn correct_four_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (6, 3), (6, 1), (8, 1), (6, 2)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn incorrect_four_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn correct_straight() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (7, 3), (5, 1), (9, 1), (3, 2)]);
    card_vec.sort();
    let card = combos::has_straight(&card_vec);
    assert_eq!(card, Some(Card::new(9, 1)));
}
#[test]
fn incorrect_straight() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (7, 3), (5, 1), (8, 1), (3, 2)]);
    card_vec.sort();
    let card = combos::has_straight(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn correct_straight_a2345() {
    let mut card_vec = Card::new_card_vec([(4, 0), (14, 2), (2, 3), (5, 1), (9, 1), (3, 2)]);
    card_vec.sort();
    let card = combos::has_straight(&card_vec);
    assert_eq!(card, Some(Card::new(5, 1)));
}
#[test]
fn correct_straight_10jqka() {
    let mut card_vec = Card::new_card_vec([(10, 0), (14, 2), (11, 3), (13, 1), (9, 1), (12, 2)]);
    card_vec.sort();
    let card = combos::has_straight(&card_vec);
    assert_eq!(card, Some(Card::new(14, 2)));
}
