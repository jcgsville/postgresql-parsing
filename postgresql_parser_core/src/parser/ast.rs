#[derive(Debug, PartialEq)]
pub struct PostgresqlAbstractSyntaxTree {
    pub commands: Vec<Command>,
}

impl PostgresqlAbstractSyntaxTree {
    pub fn new() -> PostgresqlAbstractSyntaxTree {
        return PostgresqlAbstractSyntaxTree {
            commands: Vec::new(),
        };
    }

    pub fn from_commands(commands: Vec<Command>) -> PostgresqlAbstractSyntaxTree {
        return PostgresqlAbstractSyntaxTree { commands: commands };
    }

    pub fn push_command(mut self, command: Command) -> PostgresqlAbstractSyntaxTree {
        self.commands.push(command);
        return self;
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Empty(EmptyCommand),
    DataManipulation(DataManipulationCommand),
}

#[derive(Debug, PartialEq)]
pub struct EmptyCommand {}

#[derive(Debug, PartialEq)]
pub enum DataManipulationCommand {
    Select(SelectCommand),
}

#[derive(Debug, PartialEq)]
pub struct SelectCommand {
    pub selected_expressions: Vec<SelectedExpression>,
    pub from_item: FromItem,
}

#[derive(Debug, PartialEq)]
pub enum SelectedExpression {
    AllColumns(AllColumnsSelectedExpression),
    Column(ColumnSelectedExpression),
}

// Maybe this should be smarter to distinguish from aliased names
// vs table names
#[derive(Debug, PartialEq)]
pub struct AllColumnsSelectedExpression {
    pub schema_name: Option<Identifier>,
    pub table_name: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct ColumnSelectedExpression {
    pub schema_name: Option<Identifier>,
    pub table_name: Option<Identifier>,
    pub column_name: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct FromItem {
    pub schema_name: Option<Identifier>,
    pub table_name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub quoted: bool,
    pub value: String,
}
