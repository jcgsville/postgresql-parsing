use postgresql_parser_core::ast::{
    AllColumnsSelectedExpression, ColumnSelectedExpression, Command, DataManipulationCommand,
    EmptyCommand, FromItem, PostgresqlAbstractSyntaxTree, SelectCommand, SelectedExpression,
};
use postgresql_parser_core::parse_postgresql;

fn test_parse(input_string: &str, expected_commands: Vec<Command>) {
    let parsed_tree = parse_postgresql(input_string);
    assert_eq!(
        parsed_tree,
        PostgresqlAbstractSyntaxTree::from_commands(expected_commands)
    );
}

#[test]
fn empty_command() {
    test_parse(";", vec![Command::Empty(EmptyCommand {})]);
}

#[test]
fn no_commands() {
    test_parse("", vec![]);
}

#[test]
fn command_leading_spaces() {
    test_parse(" ;", vec![Command::Empty(EmptyCommand {})]);
}

#[test]
fn basic_select_command() {
    test_parse(
        "select * from foobar;",
        vec![Command::DataManipulation(DataManipulationCommand::Select(
            SelectCommand {
                from_item: FromItem {
                    schema_name: None,
                    table_name: String::from("foobar"),
                },
                selected_expressions: vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: None,
                    },
                )],
            },
        ))],
    );
}

#[test]
fn select_command_extra_spaces() {
    test_parse(
        "select *  from  \nfoobar ; ",
        vec![Command::DataManipulation(DataManipulationCommand::Select(
            SelectCommand {
                from_item: FromItem {
                    schema_name: None,
                    table_name: String::from("foobar"),
                },
                selected_expressions: vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: None,
                    },
                )],
            },
        ))],
    );
}

#[test]
fn select_command_table_schema() {
    test_parse(
        "select * from foo.bar;",
        vec![Command::DataManipulation(DataManipulationCommand::Select(
            SelectCommand {
                from_item: FromItem {
                    schema_name: Some(String::from("foo")),
                    table_name: String::from("bar"),
                },
                selected_expressions: vec![SelectedExpression::AllColumns(
                    AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: None,
                    },
                )],
            },
        ))],
    );
}

#[test]
fn select_multi_complex_columns() {
    test_parse(
        "select firstname,public.teacher.lastname , teacher.* from public.teacher;",
        vec![Command::DataManipulation(DataManipulationCommand::Select(
            SelectCommand {
                from_item: FromItem {
                    schema_name: Some(String::from("public")),
                    table_name: String::from("teacher"),
                },
                selected_expressions: vec![
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: None,
                        table_name: None,
                        column_name: String::from("firstname"),
                    }),
                    SelectedExpression::Column(ColumnSelectedExpression {
                        schema_name: Some(String::from("public")),
                        table_name: Some(String::from("teacher")),
                        column_name: String::from("lastname"),
                    }),
                    SelectedExpression::AllColumns(AllColumnsSelectedExpression {
                        schema_name: None,
                        table_name: Some(String::from("teacher")),
                    }),
                ],
            },
        ))],
    );
}

#[test]
fn todo_prevent_reserved_word_identifiers() {
    test_parse(
        "select from from from;",
        vec![Command::DataManipulation(DataManipulationCommand::Select(
            SelectCommand {
                from_item: FromItem {
                    schema_name: None,
                    table_name: String::from("from"),
                },
                selected_expressions: vec![SelectedExpression::Column(ColumnSelectedExpression {
                    schema_name: None,
                    table_name: None,
                    column_name: String::from("from"),
                })],
            },
        ))],
    );
}

#[test]
fn todo_no_expressions_should_be_allowed() {
    test_parse("select from foo;", vec![]);
}

#[test]
fn select_invalid_from_item() {
    test_parse("select * from foo.;", vec![]);
}

#[test]
fn select_misspelled_from() {
    test_parse("select *  fromm;", vec![]);
}

#[test]
fn select_invalid_comma_trailing_expressions() {
    test_parse("select *, from foo;", vec![]);
}

#[test]
fn select_invalid_comma_leading_expressions() {
    test_parse("select ,* from foo;", vec![]);
}

#[test]
fn select_invalid_trailing_dot_in_expression() {
    test_parse("select foo. from foo;", vec![]);
}

#[test]
fn select_invalid_identifier_in_expression() {
    // Yes, this is valid postgresql.
    // For now my grammar doesn't support it.
    test_parse("select 123 from foo;", vec![]);
}

#[test]
fn select_invalid_identifier_in_from_item() {
    // Yes, this is valid postgresql.
    // For now my grammar doesn't support it.
    test_parse("select * from foo1;", vec![]);
}
