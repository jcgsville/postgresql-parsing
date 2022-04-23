use crate::lexer::token::Token;
use crate::parser::ast::FromItem;
use crate::parser::ast::Identifier;
use crate::parser::commands::parse_section::{
    parse_section_from_section, ParseCommandSectionResult,
};
use crate::parser::commands::sections::dot_separated_value::{
    parse_dot_separated_value, validate_separated_values_len,
};
use crate::parser::commands::sections::identifier::{
    parse_identifiers_from_dot_separated_values, SimpleParseResult,
};

pub fn parse_from_item(
    tokens: &Vec<Token>,
    start_idx: usize,
) -> ParseCommandSectionResult<FromItem> {
    let (idx_after, separated_values) =
        parse_section_from_section!(parse_dot_separated_value(tokens, start_idx, 1));
    validate_separated_values_len(&separated_values, 2);

    let identifiers = match parse_identifiers_from_dot_separated_values(separated_values) {
        SimpleParseResult::Valid(identifiers) => identifiers,
        SimpleParseResult::Invalid => return ParseCommandSectionResult::Invalid,
    };

    let mut schema_name: Option<Identifier> = None;
    if identifiers.len() == 2 {
        schema_name = Some(identifiers.get(0).unwrap().clone());
    }
    let table_name = identifiers.last().unwrap().clone();
    return ParseCommandSectionResult::Valid(
        idx_after,
        FromItem {
            schema_name: schema_name,
            table_name: table_name,
        },
    );
}
