use crate::lexer::char_is_whitespace;
use crate::lexer::token::Token;
pub mod ast;
use ast::Command;
use ast::DataManipulationCommand;
use ast::EmptyCommand;
use ast::PostgresqlAbstractSyntaxTree;
use ast::SelectCommand;
mod keywords;

pub fn parse_postgresql_tokens(tokens: Vec<Token>) -> PostgresqlAbstractSyntaxTree {
    let mut tree = PostgresqlAbstractSyntaxTree::new();

    let mut token_idx = 0;
    while token_idx < tokens.len() {
        match gather_command(&tokens, token_idx) {
            Some((command, new_idx)) => {
                tree = tree.push_command(command);
                token_idx = new_idx;
            }
            None => {}
        }
    }

    return tree;
}

macro_rules! return_if_gathered_command {
    ($gather_function:ident, $tokens:ident, $start_idx:ident) => {
        match $gather_function(&$tokens, $start_idx) {
            Some(command_and_new_idx) => {
                return Some(command_and_new_idx);
            }
            None => {}
        }
    };
}

fn gather_command(tokens: &Vec<Token>, start_idx: usize) -> Option<(Command, usize)> {
    let start_idx_after_whitespace = match gather_leading_whitespace(&tokens, start_idx) {
        Some(new_start_idx) => new_start_idx,
        None => start_idx,
    };
    return_if_gathered_command!(gather_empty_command, tokens, start_idx_after_whitespace);
    return_if_gathered_command!(gather_select_command, tokens, start_idx_after_whitespace);
    return None;
}

fn gather_leading_whitespace(tokens: &Vec<Token>, start_idx: usize) -> Option<usize> {
    let mut idx = start_idx;
    while token_is_whitespace(tokens.get(idx)) {
        idx += 1;
    }
    return if idx != start_idx { Some(idx) } else { None };
}

fn gather_empty_command(tokens: &Vec<Token>, start_idx: usize) -> Option<(Command, usize)> {
    if token_is_semicolon(tokens.get(start_idx)) {
        return Some((Command::Empty(EmptyCommand {}), start_idx + 1));
    }
    return None;
}

fn gather_select_command(tokens: &Vec<Token>, start_idx: usize) -> Option<(Command, usize)> {
    if token_is_keyword(tokens.get(start_idx), keywords::SELECT_KEYWORD)
        && token_is_whitespace(tokens.get(start_idx + 1))
        && token_is_star(tokens.get(start_idx + 2))
        && token_is_whitespace(tokens.get(start_idx + 3))
        && token_is_keyword(tokens.get(start_idx + 4), keywords::FROM_KEYWORD)
        && token_is_whitespace(tokens.get(start_idx + 5))
        && token_is_semicolon(tokens.get(start_idx + 7))
    {
        return match tokens.get(start_idx + 6) {
            Some(table_name_token) => Some((
                Command::DataManipulation(DataManipulationCommand::Select(SelectCommand {
                    table_name: table_name_token.value.clone(),
                })),
                start_idx + 8,
            )),
            None => None,
        };
    }
    return None;
}

fn token_is_keyword(token: Option<&Token>, keyword: &str) -> bool {
    return option_is(token, |token| {
        return token.value.to_ascii_lowercase() == keyword;
    });
}

fn token_is_whitespace(token: Option<&Token>) -> bool {
    return option_is(token, |token| {
        let first_char = token.value.chars().next();
        return match first_char {
            Some(character) => char_is_whitespace(character),
            None => false,
        };
    });
}

fn token_is_semicolon(token: Option<&Token>) -> bool {
    return token_is_str(token, ";");
}

fn token_is_star(token: Option<&Token>) -> bool {
    return token_is_str(token, "*");
}

fn token_is_str(token: Option<&Token>, test_str: &str) -> bool {
    return option_is(token, |token| {
        return token.value == test_str;
    });
}

fn option_is<T, F>(optional: Option<&T>, test_function: F) -> bool
where
    F: FnOnce(&T) -> bool,
{
    return match optional {
        Some(value) => test_function(value),
        None => false,
    };
}
