use std::fmt::Display;

#[derive(Debug)]
pub struct SyntaxError(pub String, pub (usize, usize));
// SyntaxError(expected, got, (line, col))
// Might wanna just put line and col as args instead of a tuple.

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SyntaxError -> {:?}. \nError occured on line: {:?}, col: {:?}",
            self.0, self.1.0, self.1.1
        )
    }
}

#[derive(Debug)]
pub struct TypeError(pub String);
