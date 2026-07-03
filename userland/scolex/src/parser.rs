use std::{cell::RefCell, rc::Rc};

use crate::{
    Token,
    ast::{
        Assignment, Binary, Expr, ExprLiteral, Grouping, Logical,
        Unary,
    },
    error as Scolex,
    token::Literal,
    token_type::TokenType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

/// A parse error. The error is already reported (via `Parser::error`)
/// by the time this is returned; the value just unwinds parsing back to
/// a statement boundary where `synchronize` can recover. Mirrors jlox's
/// `ParseError` exception.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError;

pub type PResult<T> = Result<T, ParseError>;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn combine_statements(
        original: Stmt,
        additional: Stmt,
    ) -> Vec<Stmt> {
        let mut v: Vec<Stmt> = Vec::new();
        v.push(original);
        v.push(additional);
        v
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Rc<RefCell<Stmt>>,
    pub else_branch: Rc<RefCell<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Rc<RefCell<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
    Block(Block),
    If(If),
    While(While),
    Null,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                // The error is already reported; skip to the next
                // statement so one mistake doesn't cascade.
                Err(_) => self.synchronize(),
            }
        }
        statements
    }

    pub fn declaration(&mut self) -> PResult<Stmt> {
        if self.expr_matches(Vec::from([TokenType::Var])) {
            return self.var_declaration();
        }

        self.statement()
    }

    pub fn var_declaration(&mut self) -> PResult<Stmt> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer = Expr::Null;
        if self.expr_matches(Vec::from([TokenType::Equal])) {
            initializer = self.expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    pub fn expression(&mut self) -> PResult<Expr> {
        self.assignment()
    }

    pub fn equality(&mut self) -> PResult<Expr> {
        let mut expr: Expr = self.comparison()?;
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::BangEqual, TokenType::EqualEqual]);

        while self.expr_matches(token_type_set.clone()) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> PResult<Expr> {
        let mut expr: Expr = self.term()?;
        let token_type_set: Vec<TokenType> = Vec::from([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]);

        while self.expr_matches(token_type_set.clone()) {
            let operator: Token = self.previous();
            let right: Expr = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    pub fn term(&mut self) -> PResult<Expr> {
        let mut expr: Expr = self.factor()?;
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::Minus, TokenType::Plus]);

        while self.expr_matches(token_type_set.clone()) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    pub fn factor(&mut self) -> PResult<Expr> {
        let mut expr: Expr = self.unary()?;
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::Slash, TokenType::Star]);

        while self.expr_matches(token_type_set.clone()) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    pub fn unary(&mut self) -> PResult<Expr> {
        let token_type_set: Vec<TokenType> =
            Vec::from([TokenType::Bang, TokenType::Minus]);

        if self.expr_matches(token_type_set) {
            let operator: Token = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    pub fn assignment(&mut self) -> PResult<Expr> {
        let expr = self.or()?;

        if self.expr_matches(Vec::from([TokenType::Equal])) {
            let equals = self.previous();
            let value = self.assignment()?;

            /* only a variable is a valid assignment target */
            match &expr {
                Expr::Variable(name) => {
                    return Ok(Expr::Assignment(Assignment {
                        name: name.clone(),
                        value: Box::new(value),
                    }));
                }
                // Report but don't unwind: jlox reports the bad target
                // and keeps parsing from the already-parsed expression.
                _ => {
                    self.error(equals, "Invalid assignment target.");
                }
            }
        }

        Ok(expr)
    }

    pub fn or(&mut self) -> PResult<Expr> {
        let mut expr = self.and()?;

        while self.expr_matches(Vec::from([TokenType::Or])) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    pub fn and(&mut self) -> PResult<Expr> {
        let mut expr = self.equality()?;

        while self.expr_matches(Vec::from([TokenType::And])) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    pub fn primary(&mut self) -> PResult<Expr> {
        let tk_vek = Vec::from([TokenType::False]);
        if self.expr_matches(tk_vek) {
            return Ok(Expr::Literal(ExprLiteral {
                value: Literal::Bool(false),
            }));
        }

        let tk_vek = Vec::from([TokenType::True]);
        if self.expr_matches(tk_vek) {
            return Ok(Expr::Literal(ExprLiteral {
                value: Literal::Bool(true),
            }));
        }

        let tk_vek = Vec::from([TokenType::Null]);
        if self.expr_matches(tk_vek) {
            return Ok(Expr::Literal(ExprLiteral {
                value: Literal::Null,
            }));
        }

        let tk_vek =
            Vec::from([TokenType::Number, TokenType::String]);
        if self.expr_matches(tk_vek) {
            return Ok(Expr::Literal(ExprLiteral {
                value: self.previous().literal.unwrap_or_default(),
            }));
        }

        let tk_vek = Vec::from([TokenType::Identifier]);
        if self.expr_matches(tk_vek) {
            return Ok(Expr::Variable(self.previous()));
        }

        let tk_vek = Vec::from([TokenType::LeftParen]);
        if self.expr_matches(tk_vek) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.",
            )?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr),
            }));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    pub fn consume(
        &mut self,
        token_type: TokenType,
        msg: &str,
    ) -> PResult<Token> {
        if self.expr_check(token_type) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), msg))
    }

    /// Reports a parse error at `token` and returns a `ParseError` so
    /// callers can `?`-propagate back to a statement boundary.
    pub fn error(&self, token: Token, msg: &str) -> ParseError {
        Scolex::error_at_token(&token, msg);
        ParseError
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

    pub fn expr_matches(
        &mut self,
        token_types: Vec<TokenType>,
    ) -> bool {
        for t in token_types {
            if self.expr_check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn statement(&mut self) -> PResult<Stmt> {
        if self.expr_matches(Vec::from([TokenType::Print])) {
            return self.print_statement();
        }

        if self.expr_matches(Vec::from([TokenType::LeftBrace])) {
            return self.block();
        }

        if self.expr_matches(Vec::from([TokenType::If])) {
            return self.if_statement();
        }

        if self.expr_matches(Vec::from([TokenType::While])) {
            return self.while_statement();
        }

        if self.expr_matches(Vec::from([TokenType::For])) {
            return self.for_statement();
        }

        self.expression_statement()
    }

    pub fn block(&mut self) -> PResult<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.expr_check(TokenType::RightBrace)
            && !self.is_at_end()
        {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' after block.",
        )?;
        Ok(Stmt::Block(Block { statements }))
    }

    pub fn print_statement(&mut self) -> PResult<Stmt> {
        let value: Expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after value.",
        )?;
        Ok(Stmt::Print(value))
    }

    pub fn while_statement(&mut self) -> PResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'while'.",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after condition.",
        )?;
        let body = self.statement()?;

        Ok(Stmt::While(While {
            condition,
            body: Rc::new(RefCell::new(body)),
        }))
    }

    pub fn for_statement(&mut self) -> PResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'for'.",
        )?;

        let initializer: Stmt =
            if self.expr_matches(Vec::from([TokenType::Semicolon])) {
                Stmt::Null
            } else if self.expr_matches(Vec::from([TokenType::Var])) {
                self.var_declaration()?
            } else {
                self.expression_statement()?
            };

        let mut condition: Expr = Expr::Null;
        if !self.expr_check(TokenType::Semicolon) {
            condition = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after loop condition.",
        )?;

        let mut increment: Expr = Expr::Null;
        if !self.expr_check(TokenType::RightParen) {
            increment = self.expression()?;
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after for clauses.",
        )?;

        let mut body = self.statement()?;

        if increment != Expr::Null {
            let incr = Stmt::Expression(increment);
            body = Stmt::Block(Block {
                statements: Block::combine_statements(body, incr),
            });
        }

        if condition == Expr::Null {
            condition = Expr::Literal(ExprLiteral {
                value: Literal::Bool(true),
            });
        }

        body = Stmt::While(While {
            condition,
            body: Rc::new(RefCell::new(body)),
        });

        if initializer != Stmt::Null {
            body = Stmt::Block(Block {
                statements: Block::combine_statements(
                    initializer,
                    body,
                ),
            })
        }

        Ok(body)
    }

    pub fn if_statement(&mut self) -> PResult<Stmt> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'if'.",
        )?;
        let condition: Expr = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after if condition.",
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = Stmt::Null;

        if self.expr_matches(Vec::from([TokenType::Else])) {
            else_branch = self.statement()?;
        }

        let then_branch = Rc::new(RefCell::new(then_branch));
        let else_branch = Rc::new(RefCell::new(else_branch));

        Ok(Stmt::If(If {
            condition,
            then_branch,
            else_branch,
        }))
    }

    pub fn expression_statement(&mut self) -> PResult<Stmt> {
        let expr: Expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after expression.",
        )?;
        Ok(Stmt::Expression(expr))
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
