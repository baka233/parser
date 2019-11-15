use std::cmp::{Ord, PartialOrd};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Token {
    Epsilon,
    Character(char),
    Identifier(char),
}
