use regex::*;

fn main() {
    let automata: Automata = dbg!("hello".parse().unwrap());
    dbg!(automata.input("hello"));
}
