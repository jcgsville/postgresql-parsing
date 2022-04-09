use postgresql_parser_core::ast::{
    Command, DataManipulationCommand, EmptyCommand, FromItem, PostgresqlAbstractSyntaxTree,
    SelectCommand,
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
            },
        ))],
    );
}

#[test]
fn select_invalid_from_item() {
    test_parse("select * from foo.;", vec![]);
}

#[test]
fn invalid_command() {
    test_parse("select *  fromm;", vec![]);
}
