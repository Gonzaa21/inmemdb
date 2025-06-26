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

    // for validate if miss arguments
    fn require_args(cmd: &str, tokens: &[&str], expected: usize) -> Result<(), CommandError> {
        if tokens.len() != expected {
            Err(CommandError::MissingCommand(cmd.to_string()))
        } else {
            Ok(())
        }
    }

    // validating if it have more arguments
    if tokens.len() >= 3 && tokens[0].eq_ignore_ascii_case("SET") {
        require_args("SET", &tokens, 3)?;
        return Ok(Command::Set(tokens[1].to_string().to_lowercase(), tokens[2].to_string()));
    }
    else if tokens.len() >= 2 && tokens[0].eq_ignore_ascii_case("GET") {
        require_args("GET", &tokens, 2)?;
        return Ok(Command::Get(tokens[1].to_string().to_lowercase()));
    }
    else if tokens.len() >= 2 && tokens[0].eq_ignore_ascii_case("DEL") {
        require_args("DEL", &tokens, 2)?;
        return Ok(Command::Del(tokens[1].to_string().to_lowercase()));
    } else {
        Err(CommandError::UnknownCommand(tokens[0].to_string()))
    }
}
