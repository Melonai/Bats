#[derive(Debug)]
pub struct Token {
    position: Position,
    value: Option<String>,
    kind: TokenKind,
}

impl Token {
    fn new(value: Option<String>, kind: TokenKind, row: usize, column: usize) -> Self {
        Token {
            position: Position { row, column },
            value,
            kind,
        }
    }

    pub fn name(value: String, row: usize, column: usize) -> Self {
        Self::new(Some(value), TokenKind::Name, row, column)
    }

    pub fn number(value: String, row: usize, column: usize) -> Self {
        Self::new(Some(value), TokenKind::Number, row, column)
    }

    pub fn letters(value: String, row: usize, column: usize) -> Self {
        Self::new(Some(value), TokenKind::Letters, row, column)
    }

    pub fn symbol(symbol: char, row: usize, column: usize) -> Self {
        Self::new(None, TokenKind::Symbol(symbol), row, column)
    }
}

#[derive(Debug)]
pub struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug)]
pub enum TokenKind {
    Name,
    Number,
    Letters,
    Symbol(char),
}
