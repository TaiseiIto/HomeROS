use regex::*;

fn main() {
    let automata: Automata = dbg!(
        r"^\d(\l+|\u*)[^a-d0-3_{}]{2,3}[^\w()*]{3}$"
            .parse()
            .unwrap()
    );
}
