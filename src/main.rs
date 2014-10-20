mod cards;

fn main() {
    let card = &cards::Card::new(2, 2);
    cards::print_card(card);
    let card = &cards::Card::new(13, 4);
    cards::print_card(card);
    let card = &cards::Card::new(1, 2);
    cards::print_card(card);
}
