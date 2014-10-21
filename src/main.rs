mod cards;

fn main() {
    println!("{}", &cards::Card::new(14, 2));
    println!("{}", &cards::Card::new(8, 3));
    println!("{}", &cards::Card::new(1, 2));
    println!("{}", &cards::Card::new(14, 4));
    let one = &cards::Card::new(3, 2);
    let two = &cards::Card::new(8, 3);
    let twotwo = &cards::Card::new(8, 2);
    let three = &cards::Card::new(14, 1);
    println!("{}", one > two);
    println!("{}", two > one);
    println!("{}", one.get_number());
    println!("{}", one.get_suit());
    let mut card_vec = vec![two, one, three, twotwo];
    println!("{}", card_vec);
    card_vec.sort();
    println!("{}", card_vec);
}
