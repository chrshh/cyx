use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Literal {
    pub fn type_name(literal: Literal) -> &'static str {
        match literal {
            Self::String(_) => "STRING",
            Self::Number(_) => "NUMBER",
            Self::Bool(_) => "BOOL",
            Self::Null => "NULL",
        }
    }
}

/* free to_string() for debugging */
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self;
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericToken<L> {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<L>,
    pub line: usize,
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

    pub fn to_string(&self) -> String
    where
        L: std::fmt::Display + Clone + Clone + Copy,
    {
        self.token_type.to_string()
            + self.lexeme.as_str()
            + self.literal.unwrap().to_string().as_str()
            + self.line.to_string().as_str()
    }
}
