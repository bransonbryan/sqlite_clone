use std::io::{self, Write};
use sqlite_clone::repl_commands;

static PROMPT: &str = "db > ";

fn main() {

    loop {
        print_prompt();

        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer).unwrap();

        let command = repl_commands::parse_command(&buffer.trim());

        match command {
            repl_commands::ReplCommand::Exit => break,
            repl_commands::ReplCommand::Unrecognized => println!("Unrecognized command: {}", buffer),
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
