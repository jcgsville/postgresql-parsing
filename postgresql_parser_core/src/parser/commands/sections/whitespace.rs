use crate::lexer::token::Token;
use crate::parser::commands::parse_section::ParseCommandSectionResult;
use crate::parser::utils::empty_parsed_datum;
use crate::parser::utils::parse_simple_token;
use crate::parser::utils::token_is_whitespace;

pub fn parse_whitespace(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    return parse_simple_token(tokens, idx, token_is_whitespace, empty_parsed_datum);
}
