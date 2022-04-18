use crate::lexer::token::Token;
use crate::parser::ast::{
    AllColumnsSelectedExpression, ColumnSelectedExpression, SelectedExpression,
};
use crate::parser::commands::parse_section::{
    parse_section_from_section, ParseCommandSectionResult,
};
use crate::parser::commands::sections::comma::parse_comma;
use crate::parser::commands::sections::dot_separated_value::parse_dot_separated_value;
use crate::parser::commands::sections::identifier::is_identifier;
use crate::parser::utils::idx_after_optional_whitespace;

fn parse_selected_expression(
    tokens: &Vec<Token>,
    start_idx: usize,
) -> ParseCommandSectionResult<SelectedExpression> {
    let (idx_after, separated_values) =
        parse_section_from_section!(parse_dot_separated_value(tokens, start_idx, 2));
    if separated_values.len() > 3 {
        panic!("Received unexpectedly long vector from parse_dot_separated_value");
    }

    let mut schema_name: Option<String> = None;
    let mut table_name: Option<String> = None;
    if separated_values.len() == 3 {
        schema_name = Some(separated_values.get(0).unwrap().clone());
        table_name = Some(separated_values.get(1).unwrap().clone());
    } else if separated_values.len() == 2 {
        table_name = Some(separated_values.get(0).unwrap().clone())
    };

    let column = separated_values.last().unwrap().clone();
    if column == "*" {
        return ParseCommandSectionResult::Valid(
            idx_after,
            SelectedExpression::AllColumns(AllColumnsSelectedExpression {
                schema_name: schema_name,
                table_name: table_name,
            }),
        );
    } else if is_identifier(&column) {
        return ParseCommandSectionResult::Valid(
            idx_after,
            SelectedExpression::Column(ColumnSelectedExpression {
                schema_name: schema_name,
                table_name: table_name,
                column_name: column,
            }),
        );
    }
    return ParseCommandSectionResult::Invalid;
}

pub fn parse_selected_expressions(
    tokens: &Vec<Token>,
    start_idx: usize,
) -> ParseCommandSectionResult<Vec<SelectedExpression>> {
    let mut selected_expressions: Vec<SelectedExpression> = Vec::new();
    let mut idx = start_idx;

    loop {
        let (idx_after_expression, selected_expression) =
            parse_section_from_section!(parse_selected_expression(tokens, idx));
        selected_expressions.push(selected_expression);
        let idx_after_whitespace = idx_after_optional_whitespace(tokens, idx_after_expression);
        match parse_comma(tokens, idx_after_whitespace) {
            ParseCommandSectionResult::Valid(idx_after_comma, _) => {
                idx = idx_after_optional_whitespace(tokens, idx_after_comma);
            }
            ParseCommandSectionResult::Invalid | ParseCommandSectionResult::EndOfInput => {
                idx = idx_after_expression;
                break;
            }
        }
    }
    return ParseCommandSectionResult::Valid(idx, selected_expressions);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{AllColumnsSelectedExpression, ColumnSelectedExpression};
    use crate::test_utils::tokens_from_str_vector;

    #[test]
    fn selected_expressions_basic_column_name() {
        assert_eq!(
            parse_selected_expressions(&tokens_from_str_vector(vec!["firstname"]), 0),
            ParseCommandSectionResult::Valid(
                1,
                vec![SelectedExpression::Column(ColumnSelectedExpression {
                    schema_name: None,
                    table_name: None,
                    column_name: String::from("firstname")
                })]
            )
        );
    }

    #[test]
    fn selected_expressions_column_name_with_table() {
        assert_eq!(
            parse_selected_expressions(
                &tokens_from_str_vector(vec!["teacher", ".", "firstname"]),
                0
            ),
            ParseCommandSectionResult::Valid(
                3,
                vec![SelectedExpression::Column(ColumnSelectedExpression {
                    schema_name: None,
                    table_name: Some(String::from("teacher")),
                    column_name: String::from("firstname")
                })]
            )
        );
    }

    #[test]
    fn selected_expressions_column_name_with_schema() {
        assert_eq!(
            parse_selected_expressions(
                &tokens_from_str_vector(vec!["public", ".", "teacher", ".", "firstname"]),
                0
            ),
            ParseCommandSectionResult::Valid(
                5,
                vec![SelectedExpression::Column(ColumnSelectedExpression {
                    schema_name: Some(String::from("public")),
                    table_name: Some(String::from("teacher")),
                    column_name: String::from("firstname")
                })]
            )
        );
    }

    #[test]
    fn selected_expressions_basic_star() {
        assert_eq!(
            parse_selected_expressions(&tokens_from_str_vector(vec!["*"]), 0),
            ParseCommandSectionResult::Valid(
                1,
                vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: None
                    }
                )]
            )
        );
    }

    #[test]
    fn selected_expressions_star_with_table() {
        assert_eq!(
            parse_selected_expressions(&tokens_from_str_vector(vec!["teacher", ".", "*"]), 0),
            ParseCommandSectionResult::Valid(
                3,
                vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: Some(String::from("teacher")),
                    }
                )]
            )
        );
    }

    #[test]
    fn selected_expressions_star_with_schema() {
        assert_eq!(
            parse_selected_expressions(
                &tokens_from_str_vector(vec!["public", ".", "teacher", ".", "*"]),
                0
            ),
            ParseCommandSectionResult::Valid(
                5,
                vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: Some(String::from("public")),
                        table_name: Some(String::from("teacher")),
                    }
                )]
            )
        );
    }

    #[test]
    fn selected_expressions_multiple_columns() {
        assert_eq!(
            parse_selected_expressions(
                &tokens_from_str_vector(vec!["firstname", ",", " ", "lastname", ",", " ", "*"]),
                0
            ),
            ParseCommandSectionResult::Valid(
                7,
                vec![
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: None,
                        table_name: None,
                        column_name: String::from("firstname")
                    }),
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: None,
                        table_name: None,
                        column_name: String::from("lastname")
                    }),
                    SelectedExpression::AllColumns(AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: None
                    })
                ]
            )
        );
    }

    #[test]
    fn selected_expressions_complex_multi_column() {
        assert_eq!(
            parse_selected_expressions(
                &tokens_from_str_vector(vec![
                    "firstname",
                    ",",
                    " ",
                    "public",
                    ".",
                    "teacher",
                    ".",
                    "lastname",
                    ",",
                    " ",
                    "teacher",
                    ".",
                    "*"
                ]),
                0
            ),
            ParseCommandSectionResult::Valid(
                13,
                vec![
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: None,
                        table_name: None,
                        column_name: String::from("firstname")
                    }),
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: Some(String::from("public")),
                        table_name: Some(String::from("teacher")),
                        column_name: String::from("lastname")
                    }),
                    SelectedExpression::AllColumns(AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: Some(String::from("teacher"))
                    })
                ]
            )
        );
    }

    #[test]
    fn selected_expressions_extra_spaces() {
        assert_eq!(
            parse_selected_expressions(
                &tokens_from_str_vector(vec!["firstname", " \n ", ",", " ", "lastname"]),
                0
            ),
            ParseCommandSectionResult::Valid(
                5,
                vec![
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: None,
                        table_name: None,
                        column_name: String::from("firstname")
                    }),
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: None,
                        table_name: None,
                        column_name: String::from("lastname")
                    })
                ]
            )
        );
    }

    #[test]
    fn selected_expressions_should_not_consume_trailing_spaces() {
        assert_eq!(
            parse_selected_expressions(&tokens_from_str_vector(vec!["*", " "]), 0),
            ParseCommandSectionResult::Valid(
                1,
                vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: None
                    }
                )]
            )
        );
    }
}
