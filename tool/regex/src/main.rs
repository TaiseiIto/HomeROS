use regex::*;

fn main() {
    let automaton: Automaton = dbg!(r"^\d(\l+|\u*)[a-d0-3_{}]{2,3}[^\w()*]{3}$".parse().unwrap());
    dbg!(automaton.input("3a3!!!"));
}
