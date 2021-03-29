mod token;

use std::{
    iter::Peekable,
    str::{self, Chars},
};
use token::Token;

pub struct Lexer<'a> {
    row: usize,
    column: usize,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Lexer {
            chars: content.chars().peekable(),
            row: 1,
            column: 1,
        }
    }

    fn current(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(c) = next {
            if c == '\n' {
                self.row += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        next
    }

    fn skip_whitespace(&mut self) {
        while self
            .current()
            .map_or(false, |x| x.is_whitespace() && *x != '\n')
        {
            self.advance();
        }
    }

    fn name(&mut self, row: usize, column: usize) -> Token {
        let mut buffer = String::new();
        loop {
            match self.current() {
                Some(&c) if c.is_alphanumeric() || c == '_' => {
                    buffer.push(c);
                    self.advance();
                }
                _ => return Token::name(buffer, row, column),
            }
        }
    }

    fn letters(&mut self, row: usize, column: usize) -> Token {
        self.advance();
        let mut buffer = String::new();
        loop {
            match self.advance().unwrap() {
                '"' => return Token::letters(buffer, row, column),
                c => buffer.push(c),
            }
        }
    }

    fn number(&mut self, row: usize, column: usize) -> Token {
        let mut buffer = String::new();
        loop {
            match self.current() {
                Some(&c) if c.is_numeric() => {
                    buffer.push(c);
                    self.advance();
                }
                _ => return Token::number(buffer, row, column),
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let c = *match self.current() {
            Some(x) => x,
            None => return None,
        };

        let token = if c == '"' {
            self.letters(self.row, self.column)
        } else if c.is_ascii_alphabetic() || c == '_' {
            self.name(self.row, self.column)
        } else if c.is_numeric() {
            self.number(self.row, self.column)
        } else {
            self.advance();
            Token::symbol(c, self.row, self.column)
        };

        Some(token)
    }
}
