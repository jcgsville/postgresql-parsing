// use crate::lexer::token::Token;
// use crate::parser::commands::parse_section::ParseCommandSectionResult;
// use crate::parser::utils::parse_simple_token;

// pub fn parse_identifier(tokens: &Vec<Token>, idx: usize) -> ParseCommandSectionResult<String> {
//     return parse_simple_token(
//         tokens,
//         idx,
//         |token| {
//             return is_identifier(&token.value);
//         },
//         |token| {
//             return token.value.clone();
//         },
//     );
// }

pub fn is_identifier(token_value: &String) -> bool {
    return token_value.len() > 0 && token_value.chars().all(|c| c.is_ascii_alphabetic());
}
