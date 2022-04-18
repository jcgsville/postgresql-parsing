use crate::lexer::token::Token;
use crate::parser::commands::parse_section::ParseCommandSectionResult;
use crate::parser::commands::sections::dot::{parse_dot, parse_non_dot};

// This version only cares that the values are non-dots.
// Future version should check for star or identifier
pub fn parse_dot_separated_value(
    tokens: &Vec<Token>,
    start_idx: usize,
    max_dots: usize,
) -> ParseCommandSectionResult<Vec<String>> {
    let mut remaining_dots = max_dots;
    let mut current_idx = start_idx;
    let mut separated_values: Vec<String> = Vec::new();
    loop {
        match parse_non_dot(tokens, current_idx) {
            ParseCommandSectionResult::Valid(idx_after_value, value) => {
                separated_values.push(value);
                current_idx = idx_after_value;
                if remaining_dots == 0 {
                    break;
                }
                match parse_dot(tokens, idx_after_value) {
                    ParseCommandSectionResult::Valid(idx_after_dot, _) => {
                        current_idx = idx_after_dot;
                        remaining_dots -= 1;
                    }
                    ParseCommandSectionResult::Invalid | ParseCommandSectionResult::EndOfInput => {
                        break;
                    }
                }
            }
            ParseCommandSectionResult::Invalid => return ParseCommandSectionResult::Invalid,
            ParseCommandSectionResult::EndOfInput => {
                if current_idx == start_idx {
                    return ParseCommandSectionResult::EndOfInput;
                }
                return ParseCommandSectionResult::Invalid;
            }
        }
    }
    return ParseCommandSectionResult::Valid(current_idx, separated_values);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::tokens_from_str_vector;

    #[test]
    fn pdsv_basic_0_dots() {
        assert_eq!(
            parse_dot_separated_value(&tokens_from_str_vector(vec!["teacher"]), 0, 2),
            ParseCommandSectionResult::Valid(1, vec![String::from("teacher")])
        );
    }

    #[test]
    fn pdsv_basic_1_dots() {
        assert_eq!(
            parse_dot_separated_value(
                &tokens_from_str_vector(vec!["public", ".", "teacher"]),
                0,
                2
            ),
            ParseCommandSectionResult::Valid(
                3,
                vec![String::from("public"), String::from("teacher")]
            )
        );
    }

    #[test]
    fn pdsv_basic_2_dots() {
        assert_eq!(
            parse_dot_separated_value(
                &tokens_from_str_vector(vec!["public", ".", "teacher", ".", "name"]),
                0,
                2
            ),
            ParseCommandSectionResult::Valid(
                5,
                vec![
                    String::from("public"),
                    String::from("teacher"),
                    String::from("name")
                ]
            )
        );
    }

    #[test]
    fn pdsv_star() {
        assert_eq!(
            parse_dot_separated_value(&tokens_from_str_vector(vec!["teacher", ".", "*"]), 0, 2),
            ParseCommandSectionResult::Valid(3, vec![String::from("teacher"), String::from("*")])
        );
    }

    #[test]
    fn pdsv_extra_dots() {
        assert_eq!(
            parse_dot_separated_value(
                &tokens_from_str_vector(vec!["public", ".", "teacher", ".", "name"]),
                0,
                1
            ),
            ParseCommandSectionResult::Valid(
                3,
                vec![String::from("public"), String::from("teacher")]
            )
        );
    }

    #[test]
    fn pdsv_leading_dot() {
        assert_eq!(
            parse_dot_separated_value(&tokens_from_str_vector(vec![".", "teacher"]), 0, 2),
            ParseCommandSectionResult::Invalid
        );
    }

    #[test]
    fn pdsv_trailing_dot() {
        assert_eq!(
            parse_dot_separated_value(&tokens_from_str_vector(vec!["teacher", "."]), 0, 2),
            ParseCommandSectionResult::Invalid
        );
    }

    #[test]
    fn pdsv_leading_end_of_input() {
        assert_eq!(
            parse_dot_separated_value(&tokens_from_str_vector(vec![]), 0, 2),
            ParseCommandSectionResult::EndOfInput
        );
    }
}
