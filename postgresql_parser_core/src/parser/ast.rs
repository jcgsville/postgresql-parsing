#[derive(Debug, PartialEq)]
pub struct PostgresqlAbstractSyntaxTree {
    commands: Vec<Command>,
}

impl PostgresqlAbstractSyntaxTree {
    pub fn new() -> PostgresqlAbstractSyntaxTree {
        return PostgresqlAbstractSyntaxTree {
            commands: Vec::new(),
        };
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
    pub table_name: String,
}
