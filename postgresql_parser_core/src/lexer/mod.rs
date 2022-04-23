pub mod token;
use std::collections::HashSet;
use token::Token;
use token::TokenPosition;

lazy_static! {
    pub static ref WHITESPACE_CHARS: HashSet<char> = {
        let mut delimiters: HashSet<char> = HashSet::new();
        delimiters.insert(' ');
        delimiters.insert('\n');
        delimiters.insert('\t');
        return delimiters;
    };
    pub static ref TOKEN_TERMINATORS: HashSet<char> = {
        let mut delimiters: HashSet<char> = HashSet::new();
        delimiters.insert('.');
        delimiters.insert(';');
        delimiters.insert(',');
        return delimiters;
    };
}

pub fn char_is_whitespace(character: char) -> bool {
    return WHITESPACE_CHARS.contains(&character);
}

fn push_if_some<T>(mut opt_val: Option<T>, mut push_to: Vec<T>) -> (Option<T>, Vec<T>) {
    match opt_val.take() {
        Some(some_val) => {
            push_to.push(some_val);
        }
        None => {}
    };
    return (opt_val, push_to);
}

pub fn tokenize_postgresql(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token: Option<Token> = None;
    let mut current_token_is_whitespace: bool = false;
    let mut current_token_is_quoted_identifier: bool = false;
    let mut current_line: usize = 0;
    let mut current_column: usize = 0;

    for character in text.chars() {
        let changing_between_whitespace_and_non =
            current_token_is_whitespace != char_is_whitespace(character);
        if character == '"' {
            if current_token_is_quoted_identifier {
                tokens.push(current_token.take().unwrap().append(character));
                current_token_is_quoted_identifier = false;
            } else {
                (_, tokens) = push_if_some(current_token, tokens);
                current_token = Some(Token::new(
                    TokenPosition {
                        line: current_line,
                        column: current_column,
                    },
                    character,
                ));
                current_token_is_quoted_identifier = true;
            }
        } else if current_token_is_quoted_identifier {
            current_token = Some(current_token.unwrap().append(character));
        } else if TOKEN_TERMINATORS.contains(&character) {
            (current_token, tokens) = push_if_some(current_token, tokens);
            tokens.push(Token::new(
                TokenPosition {
                    line: current_line,
                    column: current_column,
                },
                character,
            ));
            current_token_is_whitespace = false;
        } else if changing_between_whitespace_and_non {
            (_, tokens) = push_if_some(current_token, tokens);
            current_token = Some(Token::new(
                TokenPosition {
                    line: current_line,
                    column: current_column,
                },
                character,
            ));
            current_token_is_whitespace = !current_token_is_whitespace;
        } else {
            current_token = match current_token {
                Some(token) => Some(token.append(character)),
                None => Some(Token::new(
                    TokenPosition {
                        line: current_line,
                        column: current_column,
                    },
                    character,
                )),
            };
        }

        if character == '\n' {
            current_line += 1;
            current_column = 0;
        } else {
            current_column += 1;
        }
    }

    match current_token.take() {
        Some(token) => {
            tokens.push(token);
        }
        None => {}
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_whitespace() {
        assert_eq!(
            tokenize_postgresql(" foo\t \n bar"),
            vec![
                Token {
                    position: TokenPosition { line: 0, column: 0 },
                    value: String::from(" ")
                },
                Token {
                    position: TokenPosition { line: 0, column: 1 },
                    value: String::from("foo")
                },
                Token {
                    position: TokenPosition { line: 0, column: 4 },
                    value: String::from("\t \n ")
                },
                Token {
                    position: TokenPosition { line: 1, column: 1 },
                    value: String::from("bar")
                }
            ]
        );
    }

    #[test]
    fn lexer_separators() {
        assert_eq!(
            tokenize_postgresql("a.b,c;"),
            vec![
                Token {
                    position: TokenPosition { line: 0, column: 0 },
                    value: String::from("a")
                },
                Token {
                    position: TokenPosition { line: 0, column: 1 },
                    value: String::from(".")
                },
                Token {
                    position: TokenPosition { line: 0, column: 2 },
                    value: String::from("b")
                },
                Token {
                    position: TokenPosition { line: 0, column: 3 },
                    value: String::from(",")
                },
                Token {
                    position: TokenPosition { line: 0, column: 4 },
                    value: String::from("c")
                },
                Token {
                    position: TokenPosition { line: 0, column: 5 },
                    value: String::from(";")
                },
            ]
        );
    }

    #[test]
    fn lexer_quoted_identifiers() {
        assert_eq!(
            tokenize_postgresql("a\"b ;\""),
            vec![
                Token {
                    position: TokenPosition { line: 0, column: 0 },
                    value: String::from("a")
                },
                Token {
                    position: TokenPosition { line: 0, column: 1 },
                    value: String::from("\"b ;\"")
                },
            ]
        );
    }

    #[test]
    fn lexer_onclosed_quoted_identifier() {
        assert_eq!(
            tokenize_postgresql("a\"b"),
            vec![
                Token {
                    position: TokenPosition { line: 0, column: 0 },
                    value: String::from("a")
                },
                Token {
                    position: TokenPosition { line: 0, column: 1 },
                    value: String::from("\"b")
                },
            ]
        );
    }
}
