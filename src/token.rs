use std::fmt::{Debug, Display};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    // 1 length
    Asterisk,
    Comma,
    LessThan,
    GreaterThan,
    Modulo,
    Bang, // not valid by itself

    // 2 length
    LessThanEq,
    GreaterThanEq,
    NotEq,

    // n length

    // 1. clauses
    Select,
    From,
    Where,
    Order,
    By,
    Limit,
    Offset,

    // 2. where operators
    And,
    Or,
    Not,
    Between,
    Like,
    In,

    // 3. order operators
    Asc,
    Desc,

    // non-keyword
    String,
    Number,
    Identifier,
    End,
}

pub struct Token {
    token_type: TokenType,
    text: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token {
            token_type,
            text: lexeme,
        }
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_lexeme(&self) -> &str {
        &self.text
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type = format!("{:?}", self.token_type);
        write!(f, "{}", token_type)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
