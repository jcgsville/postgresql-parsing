use crate::lexer::token::Token;
use crate::parser::ast::FromItem;
use crate::parser::commands::parse_section::{
    parse_section_from_section, ParseCommandSectionResult,
};
use crate::parser::commands::sections::dot_separated_value::parse_dot_separated_value;
use crate::parser::commands::sections::identifier::is_identifier;

pub fn parse_from_item(
    tokens: &Vec<Token>,
    start_idx: usize,
) -> ParseCommandSectionResult<FromItem> {
    let (idx_after, separated_values) =
        parse_section_from_section!(parse_dot_separated_value(tokens, start_idx, 1));
    if separated_values.len() > 2 {
        panic!("Received unexpectedly long vector from parse_dot_separated_value");
    }

    for value in &separated_values {
        if !is_identifier(&value) {
            return ParseCommandSectionResult::Invalid;
        }
    }

    let mut schema_name: Option<String> = None;
    if separated_values.len() == 2 {
        schema_name = Some(separated_values.get(0).unwrap().clone());
    }
    let table_name = separated_values.last().unwrap().clone();
    return ParseCommandSectionResult::Valid(
        idx_after,
        FromItem {
            schema_name: schema_name,
            table_name: table_name,
        },
    );
}
