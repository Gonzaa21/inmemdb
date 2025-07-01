use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),

    #[error("Missing arguments: '{0}'")]
    MissingCommand(String),

    #[error("Invalid arguments: '{0}'")]
    InvalidCommand(String),

    #[error("Command parse failed: '{0}'")]
    ParseError(String),
}
