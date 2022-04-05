use crate::lexer::token::Token;
use crate::parser::ast::Command;
use crate::parser::ast::EmptyCommand;
use crate::parser::parse_command_result::ParseCommandResult;

pub fn parse_empty_command(_: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    return ParseCommandResult::Valid(Command::Empty(EmptyCommand {}), start_idx + 1);
}
