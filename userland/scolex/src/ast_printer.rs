use crate::{ast::Expr, token::Literal};

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => parenthesize(&operator.lexeme, &[left, right]),
        Expr::Grouping { expression } => parenthesize("group", &[expression]),
        Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &[right]),
        Expr::Literal { value } => match value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
            Literal::Null => "NULL".to_string(),
        },
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut s = String::with_capacity(32);
    s.push('(');
    s.push_str(name);
    for e in exprs {
        s.push(' ');
        s.push_str(&print(e));
    }
    s.push(')');
    s
}
