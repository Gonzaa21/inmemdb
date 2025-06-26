use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Unknown command: '{0}'")]
    UnknownCommand(String),

    #[error("Missing command: '{0}'")]
    MissingCommand(String),

    #[error("Command parse failed: '{0}'")]
    ParseError(String),
}
