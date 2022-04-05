use crate::lexer::token::Token;
pub mod ast;
use ast::PostgresqlAbstractSyntaxTree;
pub mod parse_command_result;
use parse_command_result::ParseCommandResult;
use std::collections::HashMap;
pub mod commands;
mod utils;
use commands::sections::keywords;
use utils::idx_after_optional_whitespace;
use utils::skip_invalid_command;

pub fn parse_postgresql_tokens(tokens: Vec<Token>) -> PostgresqlAbstractSyntaxTree {
    let mut tree = PostgresqlAbstractSyntaxTree::new();

    let mut token_idx = 0;
    while token_idx < tokens.len() {
        match parse_command(&tokens, token_idx) {
            ParseCommandResult::Valid(command, new_idx) => {
                tree = tree.push_command(command);
                token_idx = new_idx;
            }
            ParseCommandResult::Invalid(new_idx) => {
                token_idx = new_idx;
            }
            ParseCommandResult::EndOfInput => {
                return tree;
            }
        }
    }

    return tree;
}

type ParseFunction = fn(&Vec<Token>, usize) -> ParseCommandResult;

lazy_static! {
    static ref COMMAND_PARSERS: HashMap<String, ParseFunction> = {
        let mut parsers: HashMap<String, ParseFunction> = HashMap::new();
        parsers.insert(String::from(";"), commands::empty::parse_empty_command);
        parsers.insert(
            String::from(keywords::SELECT_KEYWORD),
            commands::select::parse_select_command,
        );
        return parsers;
    };
}

fn parse_command(tokens: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    let idx_after_whitespace = idx_after_optional_whitespace(tokens, start_idx);
    return match tokens.get(idx_after_whitespace) {
        Some(token) => {
            let lowered = token.value.to_ascii_lowercase();
            return match COMMAND_PARSERS.get(&lowered) {
                Some(parse_fn) => return parse_fn(tokens, idx_after_whitespace + 1),
                None => {
                    ParseCommandResult::Invalid(skip_invalid_command(tokens, idx_after_whitespace))
                }
            };
        }
        None => ParseCommandResult::EndOfInput,
    };
}
