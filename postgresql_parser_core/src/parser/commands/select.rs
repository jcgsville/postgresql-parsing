use crate::lexer::token::Token;
use crate::parser::ast::SelectCommand;
use crate::parser::ast::{Command, DataManipulationCommand, FromItem};
use crate::parser::commands::parse_section::parse_section;
use crate::parser::commands::sections::from_item::parse_from_item;
use crate::parser::commands::sections::keywords::parse_keyword_from;
use crate::parser::commands::sections::semicolon::parse_semicolon;
use crate::parser::commands::sections::star::parse_star;
use crate::parser::commands::sections::whitespace::parse_whitespace;
use crate::parser::parse_command_result::ParseCommandResult;
use crate::parser::utils::idx_after_optional_whitespace;

pub fn parse_select_command(tokens: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    let mut idx = start_idx;
    let from_item: FromItem;
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, _) = parse_section!(parse_star, tokens, idx);
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, _) = parse_section!(parse_keyword_from, tokens, idx);
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, from_item) = parse_section!(parse_from_item, tokens, idx);
    idx = idx_after_optional_whitespace(tokens, idx);
    (idx, _) = parse_section!(parse_semicolon, tokens, idx);
    return ParseCommandResult::Valid(
        Command::DataManipulation(DataManipulationCommand::Select(SelectCommand {
            from_item: from_item,
        })),
        idx,
    );
}
