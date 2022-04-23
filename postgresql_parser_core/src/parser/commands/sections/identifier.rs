use crate::parser::ast::Identifier;

#[derive(Debug, PartialEq)]
pub enum SimpleParseResult<TParsedData> {
    Valid(TParsedData),
    Invalid,
}

pub fn parse_identifiers_from_dot_separated_values(
    separated_values: Vec<String>,
) -> SimpleParseResult<Vec<Identifier>> {
    let mut identifiers: Vec<Identifier> = Vec::new();
    for value in separated_values {
        match parse_identifier_token_value(&value) {
            SimpleParseResult::Valid(identifier) => identifiers.push(identifier),
            SimpleParseResult::Invalid => return SimpleParseResult::Invalid,
        }
    }
    return SimpleParseResult::Valid(identifiers);
}

pub fn parse_identifier_token_value(token_value: &String) -> SimpleParseResult<Identifier> {
    return match parse_quoted_identifier_token_value(token_value) {
        SimpleParseResult::Valid(identifier) => SimpleParseResult::Valid(identifier),
        SimpleParseResult::Invalid => parse_unquoted_identifier_token_value(token_value),
    };
}

fn parse_quoted_identifier_token_value(token_value: &String) -> SimpleParseResult<Identifier> {
    let mut identifier_value = String::new();
    let char_count = token_value.chars().count();
    if char_count < 3 {
        return SimpleParseResult::Invalid;
    }

    for (idx, character) in token_value.chars().enumerate() {
        if idx == 0 || idx == char_count - 1 {
            if character != '"' {
                return SimpleParseResult::Invalid;
            }
        } else {
            identifier_value.push(character);
        }
    }
    return SimpleParseResult::Valid(Identifier {
        quoted: true,
        value: identifier_value,
    });
}

fn parse_unquoted_identifier_token_value(token_value: &String) -> SimpleParseResult<Identifier> {
    if !is_unquoted_identifier(&token_value) {
        return SimpleParseResult::Invalid;
    }
    return SimpleParseResult::Valid(Identifier {
        quoted: false,
        value: token_value.clone(),
    });
}

fn is_unquoted_identifier(token_value: &String) -> bool {
    return token_value.len() > 0
        && char_is_valid_unquoted_identifier_start(token_value.chars().next().unwrap())
        && token_value
            .chars()
            .all(|c| c == '_' || c == '$' || c.is_alphanumeric());
}

fn char_is_valid_unquoted_identifier_start(character: char) -> bool {
    return character == '_' || character.is_alphabetic();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_basic_unquoted() {
        assert_eq!(
            parse_identifier_token_value(&String::from("firstname")),
            SimpleParseResult::Valid(Identifier {
                quoted: false,
                value: String::from("firstname")
            })
        );
    }

    #[test]
    fn identifier_basic_quoted() {
        assert_eq!(
            parse_identifier_token_value(&String::from("\"firstname\"")),
            SimpleParseResult::Valid(Identifier {
                quoted: true,
                value: String::from("firstname")
            })
        );
    }

    #[test]
    fn identifier_complex_quoted() {
        assert_eq!(
            parse_identifier_token_value(&String::from("\"$. \"\n丕firstname\"")),
            SimpleParseResult::Valid(Identifier {
                quoted: true,
                value: String::from("$. \"\n丕firstname")
            })
        );
    }

    #[test]
    fn identifer_invalid_leading_char_unquoted() {
        assert_eq!(
            parse_identifier_token_value(&String::from("$foo")),
            SimpleParseResult::Invalid
        );
    }

    #[test]
    fn identifer_invalid_char_unquoted() {
        assert_eq!(
            parse_identifier_token_value(&String::from("foo-barr")),
            SimpleParseResult::Invalid
        );
    }

    #[test]
    fn identifier_invalid_unclosed_quotes() {
        assert_eq!(
            parse_identifier_token_value(&String::from("\"first")),
            SimpleParseResult::Invalid
        );
    }

    #[test]
    fn identifier_invalid_just_quote_char() {
        assert_eq!(
            parse_identifier_token_value(&String::from("\"")),
            SimpleParseResult::Invalid
        );
    }

    #[test]
    fn identifier_invalid_empty_quotes() {
        assert_eq!(
            parse_identifier_token_value(&String::from("\"\"")),
            SimpleParseResult::Invalid
        );
    }
}
