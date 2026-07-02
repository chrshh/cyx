use crate::{
    Token,
    ast::{Assignment, Binary, Expr, ExprLiteral, Grouping, Unary},
    environment::Environment,
    parser::Stmt,
    token::Literal,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute(stmt);
        }
    }

    pub fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.visit_expression_stmt(expr)
            }
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Var { name, initializer } => {
                self.visit_var_stmt(name, initializer)
            }
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Literal {
        match expr {
            Expr::Literal(literal) => {
                self.visit_literal_expr(literal)
            }
            Expr::Grouping(grouping) => {
                self.visit_grouping_expr(grouping)
            }
            Expr::Unary(unary) => self.visit_unary_expr(unary),
            Expr::Binary(binary) => self.visit_binary_expr(binary),
            Expr::Variable(variable) => {
                self.visit_variable_expr(variable)
            }
            Expr::Assignment(assignment) => {
                self.visit_assign_expr(assignment)
            }
            Expr::Null => Literal::Null,
        }
    }

    pub fn visit_expression_stmt(&mut self, expr: Expr) {
        self.evaluate(expr);
    }

    pub fn visit_print_stmt(&mut self, expr: Expr) {
        let value = self.evaluate(expr);
        println!("{}", self.stringify(&value));
    }

    pub fn visit_var_stmt(&mut self, name: Token, initializer: Expr) {
        let value = self.evaluate(initializer);
        self.environment.define(name.lexeme, value);
    }

    pub fn visit_variable_expr(&self, variable: Token) -> Literal {
        self.environment.get(&variable)
    }

    pub fn visit_literal_expr(&self, expr: ExprLiteral) -> Literal {
        expr.value
    }

    pub fn visit_grouping_expr(&mut self, expr: Grouping) -> Literal {
        self.evaluate(*expr.expression)
    }

    pub fn visit_assign_expr(&mut self, expr: Assignment) -> Literal {
        let value = self.evaluate(*expr.expression);
        self.environment.assign(expr.token, value.clone());
        value
    }

    pub fn visit_unary_expr(&mut self, expr: Unary) -> Literal {
        let right = self.evaluate(*expr.right);

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(n) => Literal::Number(-n),
                _ => Literal::Null,
            },
            TokenType::Bang => {
                let b = !self.is_truthy(&right);
                Literal::Bool(b)
            }
            _ => Literal::Null,
        }
    }

    pub fn visit_binary_expr(&mut self, expr: Binary) -> Literal {
        let left = self.evaluate(*expr.left);
        let right = self.evaluate(*expr.right);

        match left {
            Literal::Number(l) => match right {
                Literal::Number(r) => {
                    match expr.operator.token_type {
                        TokenType::Minus => Literal::Number(l - r),
                        TokenType::Slash => Literal::Number(l / r),
                        TokenType::Star => Literal::Number(l * r),
                        TokenType::Plus => Literal::Number(l + r),
                        TokenType::BangEqual => Literal::Number(
                            !self.is_equal(&left, &right) as u64
                                as f64,
                        ),
                        TokenType::EqualEqual => Literal::Number(
                            self.is_equal(&left, &right) as u64
                                as f64,
                        ),
                        TokenType::Greater => {
                            let n = (l > r) as i64;
                            Literal::Number(n as f64)
                        }
                        TokenType::GreaterEqual => {
                            let n = (l >= r) as i64;
                            Literal::Number(n as f64)
                        }
                        TokenType::Less => {
                            let n = (l < r) as i64;
                            Literal::Number(n as f64)
                        }
                        TokenType::LessEqual => {
                            let n = (l <= r) as i64;
                            Literal::Number(n as f64)
                        }
                        _ => Literal::Null,
                    }
                }
                _ => Literal::Null,
            },
            Literal::String(l) => match right {
                Literal::String(r) => {
                    match expr.operator.token_type {
                        TokenType::Plus => {
                            Literal::String(l + r.as_str())
                        }
                        _ => Literal::Null,
                    }
                }
                _ => Literal::Null,
            },
            _ => Literal::Null,
        }
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Null => false,
            Literal::Bool(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Literal, b: &Literal) -> bool {
        if a.get_type() == Literal::Null
            && b.get_type() == Literal::Null
        {
            return true;
        }

        if a.get_type() == Literal::Null {
            return false;
        }

        a == b
    }

    fn stringify(&self, expr: &Literal) -> String {
        if expr.get_type() == Literal::Null {
            return "null".to_string();
        }

        if expr.get_type() == Literal::Number(0 as f64) {
            let text = expr.to_string();
            if text.ends_with(".0") {
                return text
                    .strip_suffix(".0")
                    .unwrap_or_default()
                    .to_string();
            }
        }

        expr.to_string()
    }
}
