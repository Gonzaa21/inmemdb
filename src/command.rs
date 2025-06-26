// #[derive(Debug)]
use crate::error::CommandError;
// enum
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
}

// parse command
pub fn parse_command(input: &str) -> Result<Command, CommandError> {
    let tokens: Vec<&str> = input.trim().split_whitespace().collect(); // create vec whithin whitespaces

    // if token is empty
    if tokens.is_empty() {
        return Err(CommandError::ParseError("Empty command".into()));
    }

    // validate if have exact arguments 
    fn require_exact_args(cmd: &str, tokens: &[&str], expected: usize) -> Result<(), CommandError> {
        if tokens.len() != expected {
            Err(CommandError::MissingCommand(cmd.to_string()))
        } else {
            Ok(())
        }
    }

    let cmd = tokens[0];

    // case sensitive and use validations
    if cmd.eq_ignore_ascii_case("SET") {
        require_exact_args("SET", &tokens, 3)?;
        return Ok(Command::Set(tokens[1].into(), tokens[2].into()));
    } else if cmd.eq_ignore_ascii_case("GET") {
        require_exact_args("GET", &tokens, 2)?;
        return Ok(Command::Get(tokens[1].into()));
    } else if cmd.eq_ignore_ascii_case("DEL") {
        require_exact_args("DEL", &tokens, 2)?;
        return Ok(Command::Del(tokens[1].into()));
    } else {
        return Err(CommandError::UnknownCommand(cmd.to_string()));
    }
}