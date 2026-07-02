use crate::{ast::Expr, token::Literal};

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(b) => parenthesize(&b.operator.lexeme, &[&b.left, &b.right]),
        Expr::Grouping(g) => parenthesize("group", &[&g.expression]),
        Expr::Unary(u) => parenthesize(&u.operator.lexeme, &[&u.right]),
        Expr::Literal(l) => match &l.value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
            Literal::Null => "NULL".to_string(),
        },
        Expr::Variable(name) => name.lexeme.clone(),
        Expr::Null => "NULL".to_string(),
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
