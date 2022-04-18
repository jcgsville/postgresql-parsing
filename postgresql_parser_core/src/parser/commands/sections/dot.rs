use crate::lexer::token::Token;
use crate::parser::commands::parse_section::ParseCommandSectionResult;
use crate::parser::utils::empty_parsed_datum;
use crate::parser::utils::parse_simple_token;

pub fn parse_dot(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    return parse_simple_token(
        tokens,
        idx,
        |token| return token.value == ".",
        empty_parsed_datum,
    );
}

pub fn parse_non_dot(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<String> {
    return parse_simple_token(
        tokens,
        idx,
        |token| return token.value != ".",
        |token| {
            return token.value.clone();
        },
    );
}
