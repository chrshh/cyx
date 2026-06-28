use std::{collections::HashMap, ops::Add, sync::LazyLock};

use crate::{
    L, Token, error as Scolex,
    token::{Literal, Tokens},
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Drop for Scanner<'a> {
    fn drop(&mut self) {
        self.tokens.clear();
        self.source = "";
    }
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
        let i = 0;
        let _ = self.source.chars().next().map(|_| {
            self.start = self.current;
            let _ = i.add(1);
            self.scan_token();
        });

        println!("{}", Tokens(self.tokens.clone()));
        self.tokens.clone()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        if let Some(c) = c {
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

                _ => {
                    /* numbers */
                    if is_digit(Some(c)) {
                        self.scan_number();

                    /* identifiers */
                    } else if is_alpha(Some(c)) {
                        self.scan_identifier();

                    /* unrecognized */
                    } else {
                        Scolex::error(self.line, "unexpected character.");
                    }
                }
            }
        }
    }

    pub fn add_token_from_token_type(&mut self, t: TokenType) {
        self.add_token_from_token_type_and_literal(t, None);
    }

    pub fn add_token_from_token_type_and_literal(&mut self, t: TokenType, l: Option<L>) {
        let text: String = self.source.to_string()[self.start..self.current].to_string();
        let token = Token::from(t, text, l.unwrap_or_default(), self.line);
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

    pub fn add_num_token(&mut self, t: TokenType, lexeme: String) {
        let token = Token::from(
            t,
            lexeme.clone(),
            Literal::Number(lexeme.parse().unwrap_or_default()),
            self.line,
        );
        self.add(token);
    }

    pub fn add_identifier_token(&mut self, t: TokenType) {
        let token = Token::from(t, String::new(), L::Null, self.line);
        self.add(token);
    }

    pub fn add(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn advance(&mut self) -> Option<char> {
        self.current += 1;
        let c = self.source[self.current..].chars().next();
        match c.is_none() {
            true => None,
            false => c,
        }
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

    pub fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let res = self.source.chars().nth(self.current);
        match res.is_none() {
            true => None,
            false => res,
        }
    }

    pub fn peek_next(&mut self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }

        self.source.chars().nth(self.current + 1)
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /* literals */
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

    pub fn scan_number(&mut self) {
        while is_digit(self.peek()) && self.peek().is_some() {
            self.advance();
        }

        if self.peek().is_some() && self.peek().unwrap() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while self.peek().is_some() {
                if is_digit(self.peek()) {
                    self.advance();
                }
            }
        }

        let num: String = self.source.to_string()[self.start..self.current].to_string();
        self.add_num_token(TokenType::Number, num);
    }

    pub fn scan_identifier(&mut self) {
        while is_alpha(self.peek()) {
            self.advance();
        }

        let s: &str = &self.source[self.start..self.current];
        let mut token_type: Option<TokenType> = KEYWORDS.get(s).cloned();
        match token_type.is_none() {
            true => {
                token_type = Some(TokenType::Identifier);
                self.add_identifier_token(token_type.unwrap());
            }
            false => {
                self.add_identifier_token(token_type.unwrap());
            }
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token();
        Some(self.tokens[self.tokens.len()].clone())
    }
}

fn is_digit(c: Option<char>) -> bool {
    matches!(c, Some('0'..='9'))
}

fn is_alpha(c: Option<char>) -> bool {
    c.is_some() && c.unwrap().is_alphanumeric()
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("NULL", TokenType::Null),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("for", TokenType::For),
        ("while", TokenType::While),
    ])
});
