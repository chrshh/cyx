use crate::{ast::Expr, token::Literal};

/// Pretty-prints an expression tree as a Lisp-style s-expression,
/// e.g. `(* (- 123) (group 45.67))`. Every node is wrapped in parens
/// with its operator/name first, so the tree structure is
/// unambiguous.
pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(b) => {
            parenthesize(&b.operator.lexeme, &[&b.left, &b.right])
        }
        Expr::Logical(l) => {
            parenthesize(&l.operator.lexeme, &[&l.left, &l.right])
        }
        Expr::Grouping(g) => parenthesize("group", &[&g.expression]),
        Expr::Unary(u) => {
            parenthesize(&u.operator.lexeme, &[&u.right])
        }
        Expr::Literal(l) => literal(&l.value),
        Expr::Variable(name) => name.lexeme.clone(),
        Expr::Assignment(a) => {
            format!("(= {} {})", a.token, print(&a.value))
        }
        Expr::Null => "nil".to_string(),
    }
}

/// Formats a runtime literal. Strings are quoted so they're
/// distinguishable from identifiers/keywords in the output.
fn literal(value: &Literal) -> String {
    match value {
        Literal::Number(n) => n.to_string(),
        Literal::String(s) => format!("\"{s}\""),
        Literal::Bool(b) => b.to_string(),
        Literal::Null => "nil".to_string(),
    }
}

/// Wraps `name` and each child expression in a single parenthesized
/// group: `(name child1 child2 ...)`.
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
