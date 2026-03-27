/// Token types for the expression lexer.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Numeric literal (decimal, hex, octal, or binary)
    Number(String),
    /// Named identifier (function name, constant, variable)
    Ident(String),

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    /// `**` exponentiation
    DoubleStar,
    /// `^` — exponentiation in Standard/Scientific, XOR in Programmer
    Caret,

    // Bitwise operators
    Ampersand,
    Pipe,
    Tilde,

    // Shift operators
    /// `<<`
    ShiftLeft,
    /// `>>`
    ShiftRight,
    /// `>>>`
    LogicalShiftRight,

    // Postfix
    /// `!` factorial
    Bang,

    // Grouping
    LParen,
    RParen,
    Comma,

    /// End of input
    Eof,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::Eof)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(s) | Token::Ident(s) => write!(f, "{s}"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::DoubleStar => write!(f, "**"),
            Token::Caret => write!(f, "^"),
            Token::Ampersand => write!(f, "&"),
            Token::Pipe => write!(f, "|"),
            Token::Tilde => write!(f, "~"),
            Token::ShiftLeft => write!(f, "<<"),
            Token::ShiftRight => write!(f, ">>"),
            Token::LogicalShiftRight => write!(f, ">>>"),
            Token::Bang => write!(f, "!"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Eof => write!(f, "EOF"),
        }
    }
}
