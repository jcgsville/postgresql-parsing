use crate::lexer::token::Token;
use crate::parser::ast::FromItem;
use crate::parser::commands::parse_section::{
    parse_section_from_section, ParseCommandSectionResult,
};
use crate::parser::commands::sections::dot::parse_dot;
use crate::parser::commands::sections::identifier::parse_identifier;
use crate::parser::commands::sections::semicolon::parse_semicolon;
use crate::parser::commands::sections::whitespace::parse_whitespace;

pub fn parse_from_item(
    tokens: &Vec<Token>,
    start_idx: usize,
) -> ParseCommandSectionResult<FromItem> {
    let mut idx = start_idx;
    let first_identifier: String;

    (idx, first_identifier) = parse_section_from_section!(parse_identifier, tokens, idx);
    match parse_dot(tokens, idx) {
        ParseCommandSectionResult::Valid(idx_after_dot, _) => {
            let (idx_after_table_name, table_name) =
                parse_section_from_section!(parse_identifier, tokens, idx_after_dot);
            return ParseCommandSectionResult::Valid(
                idx_after_table_name,
                FromItem {
                    schema_name: Some(first_identifier),
                    table_name: table_name,
                },
            );
        }
        ParseCommandSectionResult::Invalid => {
            return if parse_section_result_is_valid(parse_whitespace(tokens, idx), true)
                || parse_section_result_is_valid(parse_semicolon(tokens, idx), true)
            {
                ParseCommandSectionResult::Valid(
                    idx,
                    FromItem {
                        schema_name: None,
                        table_name: first_identifier,
                    },
                )
            } else {
                ParseCommandSectionResult::Invalid
            }
        }
        ParseCommandSectionResult::EndOfInput => {
            return ParseCommandSectionResult::Valid(
                idx,
                FromItem {
                    schema_name: None,
                    table_name: first_identifier,
                },
            )
        }
    }
}

fn parse_section_result_is_valid<TParsedData>(
    result: ParseCommandSectionResult<TParsedData>,
    accept_end_of_input: bool,
) -> bool {
    return match result {
        ParseCommandSectionResult::Valid(_, _) => true,
        ParseCommandSectionResult::Invalid => false,
        ParseCommandSectionResult::EndOfInput => accept_end_of_input,
    };
}
