use crate::{Token, token::Literal};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Default for Expr {
    fn default() -> Self {
        Self::Literal {
            value: Literal::Null,
        }
    }
}

// impl Expr::Binary {
//     pub fn from(left: Box<Expr>, operator: Token, right: Box<Expr>)
// -> Box<Expr> {         return Box<Expr> {
//             left,
//             operator,
//             right,
//         }
//     }
// }
