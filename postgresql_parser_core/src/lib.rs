#[macro_use]
extern crate lazy_static;
mod lexer;
use lexer::tokenize_postgresql;
mod parser;
pub use parser::ast;
use parser::ast::PostgresqlAbstractSyntaxTree;
use parser::parse_postgresql_tokens;

pub fn parse_postgresql(text: &str) -> PostgresqlAbstractSyntaxTree {
    return parse_postgresql_tokens(tokenize_postgresql(text));
}
