use std::iter::Peekable;

use regex::{Captures, Regex};

use crate::lexer::tokens::Token;

use self::tokens::Kind;

pub mod tokens;

// Don't mess up the order or it becomes hell
const REGEX_SET: [(&str, Option<Kind>); 25] = [
    (r"^\d+", Some(Kind::Integer)), // Integers
    (r"^\n", Some(Kind::Newline)),  // Newline
    (r"^\s+", None),                // Whitespace
    (r"^\blam\b", Some(Kind::Lam)),
    (r"^\buse\b", Some(Kind::Use)),
    (r"^\bif\b", Some(Kind::If)),
    (r"^::", Some(Kind::DoubleColon)),
    (r"^:", Some(Kind::Colon)),
    (r"^;", Some(Kind::SemiColon)),
    (r"^\|", Some(Kind::Bar)),
    (r#"^"[^"]*""#, Some(Kind::String)),
    (r"^->", Some(Kind::Arrow)),
    (r"^\*", Some(Kind::Mul)),
    (r"^/", Some(Kind::Div)),
    (r"^\+", Some(Kind::Plus)),
    (r"^\-", Some(Kind::Min)),
    (r"^<=", Some(Kind::Leq)),
    (r"^>=", Some(Kind::Geq)),
    (r"^==", Some(Kind::DoubleEq)),
    (r"^!=", Some(Kind::Neq)),
    (r"^,", Some(Kind::Comma)),
    (r"^\?", Some(Kind::QMark)),
    (r"^\(", Some(Kind::LParen)),
    (r"^\)", Some(Kind::RParen)),
    (r"^\w+", Some(Kind::Ident)),
];

#[derive(Clone)]
pub struct Lexer<'a> {
    program: &'a str,
    cursor: usize,
    pub coords: (usize, usize)
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            cursor: 0,
            coords: (1, 1)
        }
    }

    pub fn has_more_token(&self) -> bool {
        self.cursor < self.program.len()
    }

    fn match_token(&mut self, tok_kind: Option<Kind>, capture: &'a str) -> Option<Token<'a>> {

        let (line, col) = self.coords;

        self.cursor += capture.len();

        match tok_kind {
            Some(Kind::Newline) => {
                self.coords = (line + 1, 1);
                self.next()
            }
            Some(kind) => {
                self.coords = (line, col + capture.len() - 1);
                Some(Token {
                    kind,
                    value: capture,
                })
            },
            // Whitespace
            None => self.next(),
        }

    }

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_more_token() {
            return None;
        }

        let current = &self.program[self.cursor..];

        for (reg, tok_type) in REGEX_SET {

            let captured_regex = Regex::new(reg).unwrap().captures(current);

            if let Some(caps) = captured_regex {
                self.match_token(tok_type, caps.get(0).unwrap().as_str())
            }

        }

        None
    }
}
