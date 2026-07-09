use crate::{ast::Expr, parser::Stmt, token::Literal};

/// Pretty-prints a statement as an s-expression, recursing into
/// nested statements and delegating expressions to [`print`]. Used by
/// the `-d` debug flag to dump the parsed program.
pub fn print_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expression(e) => print(e),
        Stmt::Print(e) => format!("(print {})", print(e)),
        Stmt::Var { name, initializer } => {
            format!("(var {} {})", name.lexeme, print(initializer))
        }
        Stmt::Block(block) => {
            let inner: Vec<String> =
                block.statements.iter().map(print_stmt).collect();
            format!("(block {})", inner.join(" "))
        }
        Stmt::If(i) => format!(
            "(if {} {} {})",
            print(&i.condition),
            print_stmt(&i.then_branch.borrow()),
            print_stmt(&i.else_branch.borrow()),
        ),
        Stmt::While(w) => format!(
            "(while {} {})",
            print(&w.condition),
            print_stmt(&w.body.borrow()),
        ),
        Stmt::Function(f) => {
            let body: Vec<String> =
                f.body.iter().map(print_stmt).collect();
            format!("(fn {} {})", f.name.lexeme, body.join(" "))
        }
        Stmt::Return(r) => {
            format!("(return {})", print(&r.value))
        }
        Stmt::Null => "nil".to_string(),
    }
}

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
            format!("(= {} {})", a.name.lexeme, print(&a.value))
        }
        Expr::Call(_) => "fn".to_string(),
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
        Literal::Func(func) => format!("\"{:?}\"", func),
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
