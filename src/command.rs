/// meta commands in the repl that are not parsed into sql statements
/// are will start with a '.' for example .exit
pub fn is_meta_command(buffer: &str) -> bool {
    buffer.starts_with(".")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_meta_command() {
        let buffer = ".exit";

        let result = is_meta_command(buffer);

        assert!(result);
    }

    #[test]
    fn handles_non_meta_command() {
        let buffer = "SELECT * FROM blah";

        let result = is_meta_command(buffer);

        assert!(!result);
    }
}
