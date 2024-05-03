use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(Debug, PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout = stdout();

        let statement_color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };

        // Print agent position
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Print statement in a specific color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

// Get user response from the command line
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();

    println!("");
    println!("{}", question);

    // Reset color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim user response
    return user_response.trim().to_string();
}

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("WARNING: You are about to run code written entirely by AI. ");
        println!("Review your code and confirm you wish to continue.");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Present Options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        // Trim whitespace and convert to lowercase
        let human_response: String = human_response.trim().to_lowercase();

        // Match response
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select '1' or '2'")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        let print_command = PrintCommand::AICall;
        let agent_pos = "AI";
        let agent_statement = "Hello, I am an AI agent";

        print_command.print_agent_message(agent_pos, agent_statement);

        let expected_output = format!("Agent: {}: {}\n", agent_pos, agent_statement);
        assert_eq!(expected_output, "Agent: AI: Hello, I am an AI agent\n");
    }
}
