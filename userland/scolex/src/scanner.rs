use crate::{L, Token, error as Scolex, token::Literal, token_type::TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let source_iter = self.source.chars();

        for _ in source_iter {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.clone()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            /* single char tokens */
            '(' => self.add_token_from_token_type(TokenType::LeftParen),
            ')' => self.add_token_from_token_type(TokenType::RightParen),
            '{' => self.add_token_from_token_type(TokenType::LeftBrace),
            '}' => self.add_token_from_token_type(TokenType::RightBrace),
            ',' => self.add_token_from_token_type(TokenType::Comma),
            '.' => self.add_token_from_token_type(TokenType::Dot),
            '-' => self.add_token_from_token_type(TokenType::Minus),
            '+' => self.add_token_from_token_type(TokenType::Plus),
            ';' => self.add_token_from_token_type(TokenType::Semicolon),
            '*' => self.add_token_from_token_type(TokenType::Star),

            /* one or two char tokens */
            '!' => {
                if self.token_match('=') {
                    self.add_token_from_token_type(TokenType::BangEqual);
                } else {
                    self.add_token_from_token_type(TokenType::Bang);
                }
            }

            '=' => {
                if self.token_match('=') {
                    self.add_token_from_token_type(TokenType::EqualEqual);
                } else {
                    self.add_token_from_token_type(TokenType::Equal);
                }
            }

            '<' => {
                if self.token_match('=') {
                    self.add_token_from_token_type(TokenType::LessEqual);
                } else {
                    self.add_token_from_token_type(TokenType::Less);
                }
            }

            '>' => {
                if self.token_match('=') {
                    self.add_token_from_token_type(TokenType::GreaterEqual);
                } else {
                    self.add_token_from_token_type(TokenType::Greater);
                }
            }

            '/' => {
                if self.token_match('/') {
                    /* skip comments until the end of the line */
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_from_token_type(TokenType::Slash);
                }
            }

            '\n' => self.line += 1,

            ' ' | '\r' | '\t' => {}

            '"' => self.scan_string(),

            _ => Scolex::error(self.line, "unexpected character."),
        }
    }

    pub fn add_token_from_token_type(&mut self, t: TokenType) {
        self.add_token_from_token_type_and_literal(t, None);
    }

    pub fn add_token_from_token_type_and_literal(&mut self, t: TokenType, l: Option<L>) {
        let text: String = self.source.to_string()[self.start..self.current].to_string();
        let token = Token::from(t, text, l.unwrap(), self.line);
        self.add(token);
    }

    pub fn add_str_token(&mut self, t: TokenType, lexeme: String) {
        let token = Token::from(
            t,
            lexeme.clone(),
            Literal::String(lexeme.clone()),
            self.line,
        );
        self.add(token);
    }

    pub fn add(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current).unwrap()
    }

    /* conditional advance */
    pub fn token_match(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        Some(self.source.chars().nth(self.current).unwrap())
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Scolex::error(self.line, "unterminated string.");
        }

        /* the closing " */
        self.advance();

        let s: String = self.source.to_string()[self.start + 1..self.current - 1].to_string();
        self.add_str_token(TokenType::String, s);
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token();
        Some(self.tokens[self.tokens.len()].clone())
    }
}
