mod cards;

fn main() {
    println!("{}", &cards::Card::new(8, 2));
    println!("{}", &cards::Card::new(14, 3));
    println!("{}", &cards::Card::new(1, 2));
    println!("{}", &cards::Card::new(14, 4));
}
