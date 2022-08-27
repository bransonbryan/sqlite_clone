const EXIT_STR: &str = ".exit";

#[derive(Debug, PartialEq)]
pub enum ReplCommand {
    Exit,
    Unrecognized,
}

pub fn parse_command(user_input: &str) -> ReplCommand {
    match user_input {
        EXIT_STR => ReplCommand::Exit,
        _ => ReplCommand::Unrecognized,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_exit_command() {
        let buffer = ".exit";

        let result = parse_command(buffer);

        assert_eq!(result, ReplCommand::Exit)
    }

    #[test]
    fn handles_unknown_input() {
        let buffer = "blah";

        let result = parse_command(buffer);

        assert_eq!(result, ReplCommand::Unrecognized);
    }

    #[test]
    fn handles_empty_string() {
        let buffer = "";

        let result = parse_command(buffer);

        assert_eq!(result, ReplCommand::Unrecognized);
    }
}
