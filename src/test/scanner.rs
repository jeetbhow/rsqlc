#[cfg(test)]
mod tests {
    use crate::{
        scanner::Scanner,
        token::{Token, TokenType},
    };
    use std::collections::HashMap;

    #[test]
    fn test_scanner() {
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
            ("", TokenType::End),
        ]
        .iter()
        .cloned()
        .collect();

        let scanner = Scanner::new(
            "SELECT name, age FROM users WHERE age > 18 AND name LIKE '%John%' ORDER BY age DESC".to_string(),
            keywords,
        );

        let tokens: Vec<Token> = scanner.collect();

        // Assert the number of tokens
        assert_eq!(tokens.len(), 19);

        // Assert the individual tokens
        assert_eq!(tokens[0].get_type(), TokenType::Select);
        assert_eq!(tokens[0].get_lexeme(), "SELECT");
        assert_eq!(tokens[1].get_type(), TokenType::Identifier);
        assert_eq!(tokens[1].get_lexeme(), "name");
        assert_eq!(tokens[2].get_type(), TokenType::Comma);
        assert_eq!(tokens[2].get_lexeme(), ",");
        assert_eq!(tokens[3].get_type(), TokenType::Identifier);
        assert_eq!(tokens[3].get_lexeme(), "age");
        assert_eq!(tokens[4].get_type(), TokenType::From);
        assert_eq!(tokens[4].get_lexeme(), "FROM");
        assert_eq!(tokens[5].get_type(), TokenType::Identifier);
        assert_eq!(tokens[5].get_lexeme(), "users");
        assert_eq!(tokens[6].get_type(), TokenType::Where);
        assert_eq!(tokens[6].get_lexeme(), "WHERE");
        assert_eq!(tokens[7].get_type(), TokenType::Identifier);
        assert_eq!(tokens[7].get_lexeme(), "age");
        assert_eq!(tokens[8].get_type(), TokenType::GreaterThan);
        assert_eq!(tokens[8].get_lexeme(), ">");
        assert_eq!(tokens[9].get_type(), TokenType::Number);
        assert_eq!(tokens[9].get_lexeme(), "18");
        assert_eq!(tokens[10].get_type(), TokenType::And);
        assert_eq!(tokens[10].get_lexeme(), "AND");
        assert_eq!(tokens[11].get_type(), TokenType::Identifier);
        assert_eq!(tokens[11].get_lexeme(), "name");
        assert_eq!(tokens[12].get_type(), TokenType::Like);
        assert_eq!(tokens[12].get_lexeme(), "LIKE");
        assert_eq!(tokens[13].get_type(), TokenType::String);
        assert_eq!(tokens[13].get_lexeme(), "'%John%'");
        assert_eq!(tokens[14].get_type(), TokenType::Order);
        assert_eq!(tokens[14].get_lexeme(), "ORDER");
        assert_eq!(tokens[15].get_type(), TokenType::By);
        assert_eq!(tokens[15].get_lexeme(), "BY");
        assert_eq!(tokens[16].get_type(), TokenType::Identifier);
        assert_eq!(tokens[16].get_lexeme(), "age");
        assert_eq!(tokens[17].get_type(), TokenType::Desc);
        assert_eq!(tokens[17].get_lexeme(), "DESC");
        assert_eq!(tokens[18].get_type(), TokenType::End);
        assert_eq!(tokens[18].get_lexeme(), "");
    }
}
