use crate::token_type::TokenType;

pub struct Tokens<L>(pub Vec<GenericToken<L>>);

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Literal {
    String(String),
    Number(f64),
    #[default]
    Null, // null is reserved for identifiers / keywords
}

impl Literal {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::String(_) => "STRING",
            Self::Number(_) => "NUMBER",
            Self::Null => "NULL",
        }
    }
}

/* free to_string() for debugging */
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericToken<L> {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<L>,
    pub line: usize,
}

impl<L: std::fmt::Display> std::fmt::Display for GenericToken<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.literal {
            Some(lit) => write!(f, "{:?} {} {}", self.token_type, self.lexeme, lit),
            None => write!(f, "{:?} {} null", self.token_type, self.lexeme),
        }
    }
}

/* token creation from params */
impl<L> GenericToken<L> {
    pub fn from(token_type: TokenType, lexeme: String, literal: L, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal: Some(literal),
            line,
        }
    }
}
impl<L: std::fmt::Display> std::fmt::Display for Tokens<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for tok in &self.0 {
            writeln!(f, "{tok}")?;
        }
        Ok(())
    }
}
