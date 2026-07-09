#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TokenType {
    /* single char tokens */
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /* one or two char tokens */
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /* literals */
    Identifier,
    String,
    Number,

    /* keywords */
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    #[default]
    Eof,
}

/* free to_string() for debugging */
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let t = self;
        write!(f, "{}", t)
    }
}
