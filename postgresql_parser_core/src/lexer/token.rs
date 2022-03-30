#[derive(Clone, Debug, PartialEq)]
pub struct TokenPosition {
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug)]
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
