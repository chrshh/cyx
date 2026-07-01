use std::ops::Deref;

use crate::{
    Token,
    ast::{Binary, Expr, ExprLiteral, Grouping, Unary},
    token::Literal,
    token_type::TokenType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub err_str: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            err_str: "",
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression().deref().clone()
    }

    pub fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    pub fn equality(&mut self) -> Box<Expr> {
        let mut expr: Expr = self.comparison();
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::BangEqual, TokenType::EqualEqual]);

        while self.expr_matches(token_type_set.clone()) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Box::new(expr)
    }

    pub fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        let token_type_set: Vec<TokenType> = Vec::from([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]);

        while self.expr_matches(token_type_set.clone()) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    pub fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::Minus, TokenType::Plus]);

        while self.expr_matches(token_type_set.clone()) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    pub fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::Slash, TokenType::Star]);

        while self.expr_matches(token_type_set.clone()) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    pub fn unary(&mut self) -> Expr {
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::Bang, TokenType::Minus]);

        if self.expr_matches(token_type_set) {
            let operator: Token = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    pub fn primary(&mut self) -> Expr {
        let tk_vek = Vec::from([TokenType::False]);
        if self.expr_matches(tk_vek) {
            return Expr::Literal(ExprLiteral {
                value: Literal::Bool(true),
            });
        }

        let tk_vek = Vec::from([TokenType::True]);
        if self.expr_matches(tk_vek) {
            return Expr::Literal(ExprLiteral {
                value: Literal::Bool(true),
            });
        }

        let tk_vek = Vec::from([TokenType::Null]);
        if self.expr_matches(tk_vek) {
            return Expr::Literal(ExprLiteral {
                value: Literal::Null,
            });
        }

        let tk_vek =
            Vec::from([TokenType::Number, TokenType::String]);
        if self.expr_matches(tk_vek) {
            return Expr::Literal(ExprLiteral {
                value: self.previous().literal.unwrap_or_default(),
            });
        }

        let tk_vek = Vec::from([TokenType::LeftParen]);
        if self.expr_matches(tk_vek) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.",
            )
            .unwrap_or_default();
            return Expr::Grouping(Grouping { expression: expr });
        }

        Expr::Literal(ExprLiteral::default())
    }

    pub fn consume(
        &mut self,
        token_type: TokenType,
        msg: &'a str,
    ) -> Result<Token, (Token, &str)> {
        if self.expr_check(token_type) {
            return Ok(self.advance());
        }

        Err((self.peek(), msg))
    }

    pub fn error<T: AsRef<str>>(&self, token: Token, msg: T) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", msg.as_ref());
        } else {
            self.report(
                token.line,
                " at '".to_string() + token.lexeme.as_str() + "'",
                msg.as_ref().to_owned(),
            );
        }
    }

    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    pub fn report<T: AsRef<str>>(
        &self,
        line: usize,
        lexeme: T,
        msg: T,
    ) {
        println!(
            "{} |  {}  | {}",
            line,
            lexeme.as_ref(),
            msg.as_ref()
        );
    }

    pub fn expr_matches(
        &mut self,
        token_types: Vec<TokenType>,
    ) -> bool {
        token_types.iter().for_each(|&t| {
            let ok = self.expr_check(t);
            if ok {
                self.advance();
            }
        });
        false
    }

    pub fn expr_check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }
}
