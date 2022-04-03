use crate::lexer::char_is_whitespace;
use crate::lexer::token::Token;
pub mod ast;
use ast::Command;
use ast::DataManipulationCommand;
use ast::EmptyCommand;
use ast::PostgresqlAbstractSyntaxTree;
use ast::SelectCommand;
mod keywords;
use std::collections::HashMap;

enum ParseCommandResult {
    Valid(Command, usize),
    Invalid(usize),
    EndOfInput,
}

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
        parsers.insert(String::from(";"), parse_empty_command);
        parsers.insert(String::from("select"), parse_select_command);
        return parsers;
    };
}

fn parse_command(tokens: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    let idx_after_whitespace = skip_optional_whitespace(tokens, start_idx);
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

fn skip_invalid_command(tokens: &Vec<Token>, start_idx: usize) -> usize {
    let mut idx = start_idx;
    while !token_is_semicolon(tokens.get(idx)) && option_is_some(tokens.get(idx)) {
        idx += 1;
    }
    return idx + 1;
}

fn skip_optional_whitespace(tokens: &Vec<Token>, start_idx: usize) -> usize {
    if token_is_whitespace(tokens.get(start_idx)) {
        return start_idx + 1;
    }
    return start_idx;
}

fn parse_empty_command(_: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    return ParseCommandResult::Valid(Command::Empty(EmptyCommand {}), start_idx + 1);
}

macro_rules! parse_section {
    ($parse:ident, $tokens:ident, $start_idx:ident) => {
        match $parse($tokens, $start_idx) {
            ParseCommandSectionResult::Valid(start_idx, parsed_data) => (start_idx, parsed_data),
            ParseCommandSectionResult::Invalid => {
                return ParseCommandResult::Invalid(skip_invalid_command($tokens, $start_idx));
            }
        }
    };
}

fn parse_select_command(tokens: &Vec<Token>, start_idx: usize) -> ParseCommandResult {
    let mut idx = start_idx;
    let table_name: String;
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, _) = parse_section!(parse_star, tokens, idx);
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, _) = parse_section!(parse_keyword_from, tokens, idx);
    (idx, _) = parse_section!(parse_whitespace, tokens, idx);
    (idx, table_name) = parse_section!(parse_identifier, tokens, idx);
    idx = skip_optional_whitespace(tokens, idx);
    (idx, _) = parse_section!(parse_semicolon, tokens, idx);
    return ParseCommandResult::Valid(
        Command::DataManipulation(DataManipulationCommand::Select(SelectCommand {
            table_name: table_name,
        })),
        idx,
    );
}

enum ParseCommandSectionResult<TParsedData> {
    Valid(usize, TParsedData),
    Invalid,
}

// Parse fns
fn parse_star(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    // For now, we can just use the keyword helper
    return parse_keyword(tokens, idx, "*");
}

fn parse_keyword_from(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    return parse_keyword(tokens, idx, keywords::FROM_KEYWORD);
}

fn parse_whitespace(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    return match tokens.get(idx) {
        Some(token) => {
            let first_char = token.value.chars().next();
            match first_char {
                Some(character) => {
                    if char_is_whitespace(character) {
                        ParseCommandSectionResult::Valid(idx + 1, ())
                    } else {
                        ParseCommandSectionResult::Invalid
                    }
                }
                None => ParseCommandSectionResult::Invalid,
            }
        }
        None => ParseCommandSectionResult::Invalid,
    };
}

fn parse_identifier(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<String> {
    return match tokens.get(idx) {
        Some(token) => {
            if token.value.len() > 0 {
                ParseCommandSectionResult::Valid(idx + 1, token.value.clone())
            } else {
                ParseCommandSectionResult::Invalid
            }
        }
        None => ParseCommandSectionResult::Invalid,
    };
}

fn parse_semicolon(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<()> {
    // For now, we can just use the keyword helper
    return parse_keyword(tokens, idx, ";");
}

// Parse helpers
fn parse_keyword(tokens: &Vec<Token>, idx: usize, keyword: &str) -> ParseCommandSectionResult<()> {
    return match token_is_keyword(tokens.get(idx), keyword) {
        true => (ParseCommandSectionResult::Valid(idx + 1, ())),
        false => ParseCommandSectionResult::Invalid,
    };
}

fn token_is_keyword(token: Option<&Token>, keyword: &str) -> bool {
    return option_is(token, |token| {
        return token.value.to_ascii_lowercase() == keyword;
    });
}

fn token_is_str(token: Option<&Token>, test_str: &str) -> bool {
    return option_is(token, |token| {
        return token.value == test_str;
    });
}

fn token_is_semicolon(token: Option<&Token>) -> bool {
    return token_is_str(token, ";");
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

fn option_is<T, F>(option: Option<&T>, test_function: F) -> bool
where
    F: FnOnce(&T) -> bool,
{
    return match option {
        Some(value) => test_function(value),
        None => false,
    };
}

fn option_is_some<T>(option: Option<&T>) -> bool {
    return match option {
        Some(_) => true,
        None => false,
    };
}
