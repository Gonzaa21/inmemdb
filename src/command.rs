// #[derive(Debug)]
use crate::error::CommandError;
// enum
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Exists(String),
    Incr(String),
    Flush,
    Scan,
}

// parse command
pub fn parse_command(input: &str) -> Result<Command, CommandError> {
    // create vec whithin whitespaces/suffixes
    let cleaned = input.trim_end_matches(|c| c == '\r' || c == '\n');
    let tokens: Vec<&str> = cleaned.split_whitespace().collect();

    // if token is empty
    if tokens.is_empty() {
        return Err(CommandError::ParseError("Empty command".into()));
    }

    // validate if have exact arguments 
    fn require_exact_args(cmd: &str, tokens: &[&str], expected: usize) -> Result<(), CommandError> {
        if tokens.len() < expected {
            Err(CommandError::MissingCommand(cmd.to_string()))
        } 
        else if tokens.len() > expected {
            Err(CommandError::InvalidCommand(cmd.to_string()))
        } else {
            Ok(())
        }
    }

    let cmd = tokens[0];

    // case sensitive and use validations
    if cmd.eq_ignore_ascii_case("SET") {
        require_exact_args("SET", &tokens, 3)?;
        return Ok(Command::Set(tokens[1].to_lowercase().into(), tokens[2].into()));
    } else if cmd.eq_ignore_ascii_case("GET") {
        require_exact_args("GET", &tokens, 2)?;
        return Ok(Command::Get(tokens[1].to_lowercase().into()));
    } else if cmd.eq_ignore_ascii_case("DEL") {
        require_exact_args("DEL", &tokens, 2)?;
        return Ok(Command::Del(tokens[1].to_lowercase().into()));
    } else if cmd.eq_ignore_ascii_case("EXISTS") {
        require_exact_args("EXISTS", &tokens, 2)?;
        return Ok(Command::Exists(tokens[1].to_lowercase().into()));
    } else if cmd.eq_ignore_ascii_case("INCR") {
        require_exact_args("INCR", &tokens, 2)?;
        return Ok(Command::Incr(tokens[1].to_lowercase().into()));
    } else if cmd.eq_ignore_ascii_case("FLUSH") {
        require_exact_args("FLUSH", &tokens, 1)?;
        return Ok(Command::Flush);
    } else if cmd.eq_ignore_ascii_case("SCAN") {
        require_exact_args("SCAN", &tokens, 1)?;
        return Ok(Command::Scan);
    } else {
        return Err(CommandError::UnknownCommand(cmd.to_string()));
    }
}