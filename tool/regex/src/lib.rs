#![no_std]

extern crate alloc;

mod automata;
mod character;
mod symbol;

pub use automata::Automata;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let _: Automata = r"^\d(\l+|\u*)[^a-d0-3_{}]{2,3}[^\w()*]{3}$"
            .parse()
            .unwrap();
    }
}
