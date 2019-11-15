#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub mod dfa;
pub mod nfa;
pub mod token;
pub mod formal;
