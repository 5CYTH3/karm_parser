use regex::Regex;

use crate::lexer::tokens::Token;

use self::tokens::Kind;

pub mod tokens;

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

    fn _match_token(&mut self, _regexp: (&str, Kind), _ctx: &str) {}

    pub fn get_next(&mut self) -> Option<Token> {
        if !self.has_more_token() {
            return None;
        }

        // Don't mess up the order or it becomes hell
        let r_set: Vec<(&str, Option<Kind>)> = vec![
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
        // Add a way to detect if the token is in the r_set.
        let s_str = &self.program[self.cursor..];
        for r_s in r_set {
            match Regex::new(r_s.0).unwrap().captures(s_str) {
                Some(caps) => {
                    let capture = caps.get(0).unwrap().as_str();
                    self.cursor += capture.len();
                    self.col_cursor += capture.len();
                    match r_s.1 {
                        Some(token_type) => {
                            if token_type == Kind::Newline {
                                self.col_cursor = 1;
                                self.line_cursor += 1;
                                return self.get_next();
                            }
                            return Some(Token {
                                kind: token_type,
                                value: capture.to_string(),
                            });
                        }
                        None => return self.get_next(),
                    }
                }
                None => continue,
            }
        }
        None
    }
}
