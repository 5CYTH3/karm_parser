use std::fmt::Display;

use crate::lexer::tokens::Kind;
#[derive(Debug)]
pub struct SyntaxError(pub Vec<Kind>, pub Option<Kind>);

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SyntaxError -> Expected: {:?}, Got: {:?}",
            self.0, self.1
        )
    }
}
