extern crate pokers;
use pokers::cards::Card;
use pokers::combos;

#[test]
fn test_correct_pair() {
    let mut card_vec = Card::new_card_vec([(8, 2), (7, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_pair(&card_vec);
    assert_eq!(card, Some(Card::new(8, 2)));
}
#[test]
fn test_incorrect_pair() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn test_correct_two_pair() {
    let mut card_vec = Card::new_card_vec([(8, 2), (6, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn test_incorrect_two_pair() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn test_correct_three_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (6, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn test_incorrect_three_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn test_correct_full_house() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (6, 3), (6, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn test_incorrect_full_house() {
    let mut card_vec = Card::new_card_vec([(7, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

#[test]
fn test_correct_four_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(6, 0), (8, 2), (6, 3), (6, 1), (8, 1), (6, 2)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, Some((Card::new(8, 0), Card::new(6, 0))));
}
#[test]
fn test_incorrect_four_of_a_kind() {
    let mut card_vec = Card::new_card_vec([(14, 2), (7, 3), (7, 1), (8, 1)]);
    card_vec.sort();
    let card = combos::has_two_pair(&card_vec);
    assert_eq!(card, None);
}

