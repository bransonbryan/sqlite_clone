use std::io::{self, Write};
use sqlite_clone::repl_commands;
use sqlite_clone::command;
use sqlite_clone::sql_compiler;

static PROMPT: &str = "db > ";

fn main() {

    loop {
        print_prompt();

        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer).unwrap();

        if command::is_meta_command(&buffer.trim()) {
            let command = repl_commands::parse_command(&buffer.trim());

            match command {
                repl_commands::ReplCommand::Exit => break,
                repl_commands::ReplCommand::Unrecognized => println!("Unrecognized command: {}", buffer),
            }
        } else {
            match sql_compiler::Statement::new(&buffer.trim()) {
                Ok(statement) => println!("Parsed statement {:?}", statement),
                Err(e) => println!("Unable to parse statement {:?}", e),
            }
        }
    }

    println!("\nExiting repl")
}

fn print_prompt() {
    print!("{}", PROMPT);
    match io::stdout().flush() {
        Ok(_) => debug_assert_eq!(true, true),
        Err(_) => panic!()
    }
}
