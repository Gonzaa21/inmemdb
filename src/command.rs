#[derive(Debug)]
// enum
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    CommandError(String)
}

// parse command
pub fn parse_command(input: &str) -> Command {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() >= 3 && tokens[0].eq_ignore_ascii_case("SET") {
        return Command::Set(tokens[1].to_string(), tokens[2].to_string());
    }
    else if tokens.len() >= 2 && tokens[0].eq_ignore_ascii_case("GET") {
        return Command::Get(tokens[1].to_string());
    }
    else if tokens.len() >= 2 && tokens[0].eq_ignore_ascii_case("DEL") {
        return Command::Del(tokens[1].to_string());
    } else {
        Command::CommandError(format!("Unknown command: '{}'", tokens[0]))
    }
}
