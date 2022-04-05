use crate::lexer::token::Token;
use crate::parser::commands::parse_section::ParseCommandSectionResult;
use crate::parser::utils::empty_parsed_datum;
use crate::parser::utils::parse_simple_token;

pub const SELECT_KEYWORD: &str = "select";
pub const FROM_KEYWORD: &str = "from";

pub fn parse_keyword_from(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    return parse_keyword(tokens, idx, FROM_KEYWORD);
}

fn parse_keyword(tokens: &Vec<Token>, idx: usize, keyword: &str) -> ParseCommandSectionResult<()> {
    return parse_simple_token(
        tokens,
        idx,
        |token| return token.value.to_ascii_lowercase() == keyword,
        empty_parsed_datum,
    );
}
