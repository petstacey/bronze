use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("{}: ", agent_position);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

// Get user input
pub fn user_input(prompt: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("\n{}", prompt);

    stdout.execute(ResetColor).unwrap();

    let mut cli_input = String::new();
    stdin()
        .read_line(&mut cli_input)
        .expect("Failed to read CLI input");

    cli_input.trim().to_string()
}

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();

        println!("");
        println!("WARNING: You are about to run code written entirely by AI.");
        println!("Review your code and confirm you wish to continue");

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] Go");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] No go");

        stdout.execute(ResetColor).unwrap();

        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        let human_response = human_response.trim().to_lowercase();

        match human_response.as_str() {
            "1" | "go" | "ok" | "y" => return true,
            "2" | "no go" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Only '1' or '2' allowed")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_print_message() {
        PrintCommand::AICall
            .print_agent_message("Project Manager", "Testing testing, processing something");
    }
}
