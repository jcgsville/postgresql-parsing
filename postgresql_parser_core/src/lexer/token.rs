#[derive(Clone, Debug, PartialEq)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub position: TokenPosition,
    pub value: String,
}

impl Token {
    pub fn new(position: TokenPosition, c: char) -> Token {
        return Token {
            position: position,
            value: String::from(c),
        };
    }

    pub fn append(mut self, c: char) -> Token {
        self.value.push(c);
        return self;
    }
}
