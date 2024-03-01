use regex::Regex;

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
pub struct Lexer {
    program: String,
    cursor: usize,
    pub line_cursor: usize,
    pub col_cursor: usize,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        Self {
            program,
            cursor: 0,
            col_cursor: 1,
            line_cursor: 1,
        }
    }

    pub fn has_more_token(&self) -> bool {
        self.cursor < self.program.len()
    }

    fn match_token(&mut self, (reg, tok_kind): (&str, Option<Kind>), capture: &str) -> Option<Token> {
        self.cursor += capture.len();
        self.col_cursor += capture.len() - 1;
        match tok_kind {
            Some(Kind::Newline) => {
                self.col_cursor = 1;
                self.line_cursor += 1;
                self.next()
            },
            Some(kind) => Some(Token {
                kind,
                value: capture.to_string(),
            }),
            None => self.next(),
        }

    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_more_token() {
            return None;
        }

        let s_str = &self.program[self.cursor..];

        for (reg, tok_type) in REGEX_SET {
            match Regex::new(reg).unwrap().captures(s_str) {
                Some(caps) => return self.match_token((reg, tok_type), caps.get(0).unwrap().as_str()),
                None => continue
            }
        }

        None
    }

}
