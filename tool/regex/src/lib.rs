#![no_std]

extern crate alloc;

mod automata;
mod symbol;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        symbol::Expression::parse(r"^\d(\l+|\u*)\w{2,3}$");
    }
}
