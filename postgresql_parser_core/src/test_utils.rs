use crate::lexer::token::{Token, TokenPosition};

pub fn tokens_from_str_vector(str_vector: Vec<&str>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_line: usize = 0;
    let mut current_column: usize = 0;
    for item in str_vector {
        tokens.push(Token {
            position: TokenPosition {
                line: current_line,
                column: current_column,
            },
            value: String::from(item),
        });
        if item == "\n" {
            current_line += 1;
        } else {
            current_column += item.len();
        }
    }
    return tokens;
}
