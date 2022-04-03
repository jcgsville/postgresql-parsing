use postgresql_parser_core::ast::Command;
use postgresql_parser_core::ast::DataManipulationCommand;
use postgresql_parser_core::ast::EmptyCommand;
use postgresql_parser_core::ast::PostgresqlAbstractSyntaxTree;
use postgresql_parser_core::ast::SelectCommand;
use postgresql_parser_core::parse_postgresql;

#[test]
fn empty_command() {
    let parsed_tree = parse_postgresql(";");
    let mut expected_tree = PostgresqlAbstractSyntaxTree::new();
    expected_tree = expected_tree.push_command(Command::Empty(EmptyCommand {}));
    assert_eq!(parsed_tree, expected_tree);
}

#[test]
fn no_commands() {
    let parsed_tree = parse_postgresql("");
    let expected_tree = PostgresqlAbstractSyntaxTree::new();
    assert_eq!(parsed_tree, expected_tree);
}

#[test]
fn command_leading_spaces() {
    let parsed_tree = parse_postgresql(" ;");
    let mut expected_tree = PostgresqlAbstractSyntaxTree::new();
    expected_tree = expected_tree.push_command(Command::Empty(EmptyCommand {}));
    assert_eq!(parsed_tree, expected_tree);
}

#[test]
fn basic_select_command() {
    let parsed_tree = parse_postgresql("select * from foobar;");
    let mut expected_tree = PostgresqlAbstractSyntaxTree::new();
    expected_tree = expected_tree.push_command(Command::DataManipulation(
        DataManipulationCommand::Select(SelectCommand {
            table_name: String::from("foobar"),
        }),
    ));
    assert_eq!(parsed_tree, expected_tree);
}

#[test]
fn select_command_extra_spaces() {
    let parsed_tree = parse_postgresql("select *  from  \nfoobar ; ");
    let mut expected_tree = PostgresqlAbstractSyntaxTree::new();
    expected_tree = expected_tree.push_command(Command::DataManipulation(
        DataManipulationCommand::Select(SelectCommand {
            table_name: String::from("foobar"),
        }),
    ));
    assert_eq!(parsed_tree, expected_tree);
}

#[test]
fn invalid_command() {
    let parsed_tree = parse_postgresql("select *  fromm;");
    let expected_tree = PostgresqlAbstractSyntaxTree::new();
    assert_eq!(parsed_tree, expected_tree);
}
