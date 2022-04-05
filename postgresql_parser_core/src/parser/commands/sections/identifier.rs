use crate::lexer::token::Token;
use crate::parser::commands::parse_section::ParseCommandSectionResult;
use crate::parser::utils::parse_simple_token;

pub fn parse_identifier(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<String> {
    return parse_simple_token(
        tokens,
        idx,
        |token| {
            return token.value.len() > 0;
        },
        |token| {
            return token.value.clone();
        },
    );
}
