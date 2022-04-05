use crate::lexer::token::Token;
use crate::parser::ast::Command;
use crate::parser::ast::DataManipulationCommand;
use crate::parser::ast::SelectCommand;
use crate::parser::commands::parse_section::parse_section;
use crate::parser::commands::sections::identifier::parse_identifier;
use crate::parser::commands::sections::keywords::parse_keyword_from;
use crate::parser::commands::sections::semicolon::parse_semicolon;
use crate::parser::commands::sections::star::parse_star;
use crate::parser::commands::sections::whitespace::parse_whitespace;
use crate::parser::parse_command_result::ParseCommandResult;
use crate::parser::utils::idx_after_optional_whitespace;

pub fn parse_select_command(tokens: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    let mut idx = start_idx;
    let table_name: String;
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, _) = parse_section!(parse_star, tokens, idx);
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, _) = parse_section!(parse_keyword_from, tokens, idx);
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, table_name) = parse_section!(parse_identifier, tokens, idx);
    idx = idx_after_optional_whitespace(tokens, idx);
    (idx, _) = parse_section!(parse_semicolon, tokens, idx);
    return ParseCommandResult::Valid(
        Command::DataManipulation(DataManipulationCommand::Select(SelectCommand {
            table_name: table_name,
        })),
        idx,
    );
}
