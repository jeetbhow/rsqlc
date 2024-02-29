use rsqlc::{scanner::Scanner, token::TokenType};
use std::collections::HashMap;

fn main() {
    let keywords: HashMap<&'static str, TokenType> = [
        ("*", TokenType::Asterisk),
        (",", TokenType::Comma),
        ("%", TokenType::Modulo),
        ("select", TokenType::Select),
        ("from", TokenType::From),
        ("where", TokenType::Where),
        ("and", TokenType::And),
        ("or", TokenType::Or),
        ("not", TokenType::Not),
        ("like", TokenType::Like),
        ("in", TokenType::In),
        ("order", TokenType::Order),
        ("by", TokenType::By),
        ("asc", TokenType::Asc),
        ("desc", TokenType::Desc),
        ("limit", TokenType::Limit),
        ("offset", TokenType::Offset),
        ("between", TokenType::Between),
    ]
    .iter()
    .cloned()
    .collect();

    let source =
        String::from("SELECT name, age FROM users WHERE age > 18 AND name LIKE '%John%' ORDER BY age DESC");
    let mut scanner = Scanner::new(source, keywords);
    while let Some(token) = scanner.next() {
        println!("{:?}", token);
    }
}
