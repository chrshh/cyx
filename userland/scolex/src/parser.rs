use crate::{
    Token,
    ast::{Assignment, Binary, Expr, ExprLiteral, Grouping, Unary},
    token::Literal,
    token_type::TokenType,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub err_str: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            err_str: "",
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    pub fn declaration(&mut self) -> Stmt {
        if self.expr_matches(Vec::from([TokenType::Var])) {
            return self.var_declaration();
        }

        self.statement()
    }

    pub fn var_declaration(&mut self) -> Stmt {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")
            .expect("Expect variable name.");

        let mut initializer = Expr::Null;
        if self.expr_matches(Vec::from([TokenType::Equal])) {
            initializer = *self.expression();
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )
        .expect("Expect ';' after variable declaration.");

        Stmt::Var { name, initializer }
    }

    pub fn expression(&mut self) -> Box<Expr> {
        Box::new(self.assignment())
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

    pub fn assignment(&mut self) -> Expr {
        let expr = self.equality();

        if self.expr_matches(Vec::from([TokenType::Equal])) {
            let _equals = self.previous();
            let value = self.assignment();

            /* run a match here to check for variable type */
            match *expr {
                Expr::Variable(var) => {
                    let name = var.lexeme;
                    match value {
                        Expr::Literal(val) => {
                            return Expr::Assignment(Assignment {
                                expression: Box::new(Expr::Null),
                                token: name,
                                value: val.value,
                            });
                        }
                        _ => panic!("Invalid assignment target."),
                    }
                }
                _ => panic!("Invalid assignment target."),
            }
        }

        *expr
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

        let tk_vek = Vec::from([TokenType::Identifier]);
        if self.expr_matches(tk_vek) {
            return Expr::Variable(self.previous());
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
        for t in token_types {
            if self.expr_check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn statement(&mut self) -> Stmt {
        if self.expr_matches(Vec::from([TokenType::Print])) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    pub fn print_statement(&mut self) -> Stmt {
        let value: Expr = *self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.")
            .expect("Expect ';' after value.");
        Stmt::Print(value)
    }

    pub fn expression_statement(&mut self) -> Stmt {
        let expr: Expr = *self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after expression.",
        )
        .expect("Expect ';' after expression.");
        Stmt::Expression(expr)
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
