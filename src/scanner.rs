use crate::token::{Token, TokenType};

use core::panic;
use std::collections::HashMap;

pub struct Scanner {
    keywords: HashMap<&'static str, TokenType>,
    source: Vec<char>,
    start: usize,
    curr: usize, //invariant: curr always points to what advance returns.
    end: usize,
    done: bool,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        /*
         * We differentiate between when the scanner is 'done' and when it is at the end
         * because we want to return the 'End' token when we're at the end of the text. This is
         * different from actually being done.
         */
        if self.done {
            return None;
        }

        if self.at_end() {
            self.done = true;
            return Some(Token::new(TokenType::End, String::from("")));
        }

        let mut curr_char = self.advance();

        // loop to ignore whitespace.
        while curr_char.is_whitespace() {
            if self.at_end() {
                return None;
            }
            curr_char = self.advance();
        }

        self.start = self.curr - 1;
        Some(self.process(curr_char))
    }
}

impl Scanner {
    pub fn new(source: String, keywords: HashMap<&'static str, TokenType>) -> Self {
        let buf: Vec<char> = source.chars().collect();
        let end = buf.len();

        Scanner {
            keywords,
            source: buf,
            start: 0,
            curr: 0,
            end,
            done: false,
        }
    }

    fn at_end(&self) -> bool {
        self.curr >= self.end
    }

    fn peek(&self) -> Option<char> {
        if self.at_end() {
            return None;
        }
        Some(self.source[self.curr])
    }

    fn advance(&mut self) -> char {
        let curr = self.source[self.curr];
        self.curr += 1;
        curr
    }

    fn process(&mut self, c: char) -> Token {
        match c {
            'A'..='Z' | 'a'..='z' | '_' => self.match_identifier(),
            '0'..='9' => self.match_number(),
            '\'' => self.match_string(),
            '<' => self.match_two_chars('=', TokenType::LessThan, TokenType::LessThanEq),
            '>' => self.match_two_chars('=', TokenType::GreaterThan, TokenType::GreaterThanEq),
            '!' => self.match_two_chars('=', TokenType::Bang, TokenType::NotEq),
            _ => self.match_one_char(c),
        }
    }

    fn match_string(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c == '\'' {
                self.advance();
                return self.create_token(TokenType::String);
            } else {
                self.advance();
            }
        }
        panic!("Expected closing quote")
    }

    fn match_one_char(&self, c: char) -> Token {
        let stringified_c = c.to_string();
        match self.keywords.get(stringified_c.as_str()) {
            Some(token_type) => self.create_token(*token_type),
            None => panic!("Unexpected character: {}", c),
        }
    }

    fn match_two_chars(&mut self, expected: char, one_char_var: TokenType, two_char_var: TokenType) -> Token {
        let next = self.peek();

        if next.is_none() || next.unwrap() != expected {
            match one_char_var {
                TokenType::Bang => panic!("Expected = after !"),
                _ => self.create_token(one_char_var),
            }
        } else {
            self.advance();
            self.create_token(two_char_var)
        }
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        let substring: String = self.source[self.start..self.curr].iter().collect();

        // check if substring is a keyword
        if let Some(keyword) = self.keywords.get(substring.to_lowercase().as_str()) {
            Token::new(*keyword, substring)
        } else {
            Token::new(token_type, substring)
        }
    }

    fn match_number(&mut self) -> Token {
        let mut has_decimal = false;

        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.advance();
            } else if c.is_whitespace() {
                return self.create_token(TokenType::Number);
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                self.advance();
            } else {
                panic!("Unexpected character: {}", c)
            }
        }

        self.create_token(TokenType::Number)
    }

    fn match_identifier(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else if c == ',' || c.is_whitespace() {
                return self.create_token(TokenType::Identifier);
            } else {
                panic!("Unexpected character: {}", c)
            }
        }

        self.create_token(TokenType::Identifier)
    }
}
