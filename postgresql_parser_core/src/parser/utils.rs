use crate::lexer::char_is_whitespace;
use crate::lexer::token::Token;
use crate::parser::commands::parse_section::ParseCommandSectionResult;

pub fn skip_invalid_command(tokens: &Vec<Token>, start_idx: usize) -> usize {
    let mut idx = start_idx;
    while !token_is_semicolon(tokens.get(idx)) && option_is_some(tokens.get(idx)) {
        idx += 1;
    }
    return idx + 1;
}

fn token_is_semicolon(token: Option<&Token>) -> bool {
    return token_is_str(token, ";");
}

fn token_is_str(token: Option<&Token>, test_str: &str) -> bool {
    return option_is(token, |token| {
        return token.value == test_str;
    });
}

fn option_is_some<T>(option: Option<&T>) -> bool {
    return match option {
        Some(_) => true,
        None => false,
    };
}

fn option_is<T, F>(option: Option<&T>, test_function: F) -> bool
where
    F: FnOnce(&T) -> bool,
{
    return match option {
        Some(value) => test_function(value),
        None => false,
    };
}

pub fn idx_after_optional_whitespace(tokens: &Vec<Token>, start_idx: usize) -> usize {
    return match tokens.get(start_idx) {
        Some(token) => {
            if token_is_whitespace(token) {
                start_idx + 1
            } else {
                start_idx
            }
        }
        None => start_idx,
    };
}

pub fn token_is_whitespace(token: &Token) -> bool {
    let first_char = token.value.chars().next();
    return match first_char {
        Some(character) => char_is_whitespace(character),
        None => panic!("Received a token with no first character: {:?}", token),
    };
}

pub fn parse_simple_token<TParsedDatum, FTestFunction>(
    tokens: &Vec<Token>,
    idx: usize,
    parse_test: FTestFunction,
    extract_parsed_data: fn(token: &Token) -> TParsedDatum,
) -> ParseCommandSectionResult<TParsedDatum>
where
    FTestFunction: Fn(&Token) -> bool,
{
    return match tokens.get(idx) {
        Some(token) => {
            if parse_test(token) {
                ParseCommandSectionResult::Valid(idx + 1, extract_parsed_data(token))
            } else {
                ParseCommandSectionResult::Invalid
            }
        }
        None => ParseCommandSectionResult::EndOfInput,
    };
}

pub fn empty_parsed_datum(_: &Token) -> () {
    return ();
}

// pub fn parse_section_result_is_valid<TParsedData>(
//     result: ParseCommandSectionResult<TParsedData>,
//     accept_end_of_input: bool,
// ) -> bool {
//     return match result {
//         ParseCommandSectionResult::Valid(_, _) => true,
//         ParseCommandSectionResult::Invalid => false,
//         ParseCommandSectionResult::EndOfInput => accept_end_of_input,
//     };
// }
