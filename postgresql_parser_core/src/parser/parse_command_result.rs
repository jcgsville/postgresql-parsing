use crate::parser::ast::Command;

pub enum ParseCommandResult {
    Valid(Command, usize),
    Invalid(usize),
    EndOfInput,
}
