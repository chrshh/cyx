use std::{cell::RefCell, rc::Rc};

use crate::{
    Token,
    ast::{
        Assignment, Binary, Call, Expr, ExprLiteral, Grouping,
        Logical, Unary,
    },
    callable::{CallableObject, Clock},
    environment::Environment,
    error::{self, RuntimeError},
    parser::{Block, Function, If, Return, Stmt, While},
    token::Literal,
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::default()));
        globals.borrow_mut().define(
            "clock".to_string(),
            Literal::Func(Clock::new_clock()),
        );

        Self {
            globals: Rc::clone(&globals),
            environment: globals,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            if let Err(err) = self.execute(stmt) {
                // Report and stop: like jlox, a runtime error aborts
                // execution (main then exits 70).
                error::runtime_error(&err);
                break;
            }
        }
    }

    pub fn execute(
        &mut self,
        stmt: Stmt,
    ) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.visit_expression_stmt(expr)
            }
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Var { name, initializer } => {
                self.visit_var_stmt(name, initializer)
            }
            Stmt::Block(statements) => {
                self.visit_block_stmt(statements)
            }
            Stmt::If(if_stmt) => self.visit_if_stmt(if_stmt),
            Stmt::While(while_stmt) => {
                self.visit_while_stmt(while_stmt)
            }
            Stmt::Function(function) => {
                self.visit_function_stmt(function)
            }
            Stmt::Return(r) => self.visit_return_stmt(r),
            Stmt::Null => Ok(()),
        }
    }

    pub fn execute_block_stmt(
        &mut self,
        statements: Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;

        // Run the block, but always restore the previous scope
        // afterwards — even if a statement errors out.
        let mut result = Ok(());
        for s in statements {
            result = self.execute(s);
            if result.is_err() {
                break;
            }
        }

        self.environment = previous;
        result
    }

    pub fn evaluate(
        &mut self,
        expr: Expr,
    ) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(literal) => {
                self.visit_literal_expr(literal)
            }
            Expr::Logical(logical) => {
                self.visit_logical_expr(logical)
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
            Expr::Call(call) => self.visit_call_expr(call),
            Expr::Null => Ok(Literal::Null),
        }
    }

    pub fn visit_expression_stmt(
        &mut self,
        expr: Expr,
    ) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    pub fn visit_print_stmt(
        &mut self,
        expr: Expr,
    ) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    pub fn visit_var_stmt(
        &mut self,
        name: Token,
        initializer: Expr,
    ) -> Result<(), RuntimeError> {
        let value = self.evaluate(initializer)?;
        self.environment.borrow_mut().define(name.lexeme, value);
        Ok(())
    }

    pub fn visit_function_stmt(
        &mut self,
        function: Function,
    ) -> Result<(), RuntimeError> {
        let name = function.name.lexeme.clone();
        let callable = CallableObject::new_function(function);
        self.environment
            .borrow_mut()
            .define(name, Literal::Func(callable));
        Ok(())
    }

    pub fn visit_return_stmt(
        &mut self,
        ret: Return,
    ) -> Result<(), RuntimeError> {
        let mut _val = Literal::Null;
        if ret.value != Expr::Null.into() {
            _val = self.evaluate(*ret.value)?;
        }

        // self.call_return(val);

        Ok(())
    }

    pub fn visit_if_stmt(
        &mut self,
        if_stmt: If,
    ) -> Result<(), RuntimeError> {
        let condition = self.evaluate(if_stmt.condition)?;
        if self.is_truthy(&condition) {
            let stmt = if_stmt.then_branch.borrow().clone();
            self.execute(stmt)?;
        } else if if_stmt.else_branch.borrow().clone() != Stmt::Null {
            let stmt = if_stmt.else_branch.borrow().clone();
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn visit_while_stmt(
        &mut self,
        while_stmt: While,
    ) -> Result<(), RuntimeError> {
        loop {
            let condition =
                self.evaluate(while_stmt.condition.clone())?;
            if !self.is_truthy(&condition) {
                break;
            }
            let body = while_stmt.body.borrow().clone();
            self.execute(body)?;
        }
        Ok(())
    }

    pub fn visit_block_stmt(
        &mut self,
        stmt: Block,
    ) -> Result<(), RuntimeError> {
        let scope =
            Environment::with_enclosing(Rc::clone(&self.environment));
        self.execute_block_stmt(
            stmt.statements,
            Rc::new(RefCell::new(scope)),
        )
    }

    pub fn visit_call_expr(
        &mut self,
        expr: Call,
    ) -> Result<Literal, RuntimeError> {
        let callee = self.evaluate(*expr.callee)?;

        let mut arguments: Vec<Literal> = Vec::new();

        for argument in expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        /* ensures callee implements Callable trait */
        match callee {
            Literal::Func(obj) => {
                /* fires when expected args  != received args count */
                if arguments.len() != obj.arity() {
                    return Err(RuntimeError {
                        token: expr.paren,
                        message: format!(
                            "Expected {} arguments but got {}.",
                            obj.arity(),
                            arguments.len()
                        ),
                    });
                }
                obj.call(self, arguments)
            }
            _ => Err(RuntimeError {
                token: Token::default(),
                message: "Can only call functions and classes"
                    .to_string(),
            }),
        }
    }

    pub fn visit_variable_expr(
        &self,
        variable: Token,
    ) -> Result<Literal, RuntimeError> {
        self.environment.borrow().get(&variable)
    }

    pub fn visit_literal_expr(
        &self,
        expr: ExprLiteral,
    ) -> Result<Literal, RuntimeError> {
        Ok(expr.value)
    }

    pub fn visit_grouping_expr(
        &mut self,
        expr: Grouping,
    ) -> Result<Literal, RuntimeError> {
        self.evaluate(*expr.expression)
    }

    pub fn visit_assign_expr(
        &mut self,
        expr: Assignment,
    ) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(*expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    pub fn visit_logical_expr(
        &mut self,
        expr: Logical,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(*expr.left)?;

        if expr.operator.token_type == TokenType::Or {
            // `or` short-circuits when the left side is truthy.
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            // `and` short-circuits when the left side is falsey.
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(*expr.right)
    }

    pub fn visit_unary_expr(
        &mut self,
        expr: Unary,
    ) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(*expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => {
                let n =
                    self.number_operand(&expr.operator, &right)?;
                Ok(Literal::Number(-n))
            }
            TokenType::Bang => {
                Ok(Literal::Bool(!self.is_truthy(&right)))
            }
            _ => Ok(Literal::Null),
        }
    }

    pub fn visit_binary_expr(
        &mut self,
        expr: Binary,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(*expr.left)?;
        let right = self.evaluate(*expr.right)?;
        let operator = &expr.operator;

        match operator.token_type {
            // Equality works across any operand types.
            TokenType::BangEqual => {
                Ok(Literal::Bool(!self.is_equal(&left, &right)))
            }
            TokenType::EqualEqual => {
                Ok(Literal::Bool(self.is_equal(&left, &right)))
            }

            // `+` is overloaded: add numbers or concatenate strings.
            TokenType::Plus => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => {
                    Ok(Literal::Number(l + r))
                }
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{l}{r}")))
                }
                _ => Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be two numbers or two strings.",
                )),
            },

            // Everything else requires two numbers.
            _ => {
                let (l, r) =
                    self.number_operands(operator, &left, &right)?;
                match operator.token_type {
                    TokenType::Minus => Ok(Literal::Number(l - r)),
                    TokenType::Slash => Ok(Literal::Number(l / r)),
                    TokenType::Star => Ok(Literal::Number(l * r)),
                    TokenType::Greater => Ok(Literal::Bool(l > r)),
                    TokenType::GreaterEqual => {
                        Ok(Literal::Bool(l >= r))
                    }
                    TokenType::Less => Ok(Literal::Bool(l < r)),
                    TokenType::LessEqual => Ok(Literal::Bool(l <= r)),
                    _ => Ok(Literal::Null),
                }
            }
        }
    }

    /// Checks a single operand is a number, mirroring jlox's
    /// `checkNumberOperand`.
    fn number_operand(
        &self,
        operator: &Token,
        operand: &Literal,
    ) -> Result<f64, RuntimeError> {
        match operand {
            Literal::Number(n) => Ok(*n),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operand must be a number.",
            )),
        }
    }

    /// Checks both operands are numbers (`checkNumberOperands`).
    fn number_operands(
        &self,
        operator: &Token,
        left: &Literal,
        right: &Literal,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => Ok((*l, *r)),
            _ => Err(RuntimeError::new(
                operator.clone(),
                "Operands must be numbers.",
            )),
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
