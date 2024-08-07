use std::fmt::Display;

use crate::lexer::tokens::Kind;
#[derive(Debug)]
pub struct SyntaxError(pub Vec<Kind>, pub Option<Kind>, pub (usize, usize));
// SyntaxError(expected, got, (line, col))
// Might wanna just put line and col as args instead of a tuple.

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SyntaxError -> Expected: {:?}, Got: {:?} at col {:?}, line {:?}",
            self.0, self.1, self.2 .0, self.2 .1
        )
    }
}

#[derive(Debug)]
pub struct TypeError(pub String);
