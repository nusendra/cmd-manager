mod storage;
mod commands;
mod execution;

use inquire::{Select, Text};
use crossterm::terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\n=== Command List Manager ===");
        let options = vec![
            "List commands",
            "Add command",
            "Delete command",
            "Execute command",
            "Exit",
        ];

        let choice = Select::new("What would you like to do?", options)
            .prompt()?;

        match choice {
            "List commands" => list_commands()?,
            "Add command" => add_command()?,
            "Delete command" => delete_command()?,
            "Execute command" => execute_command()?,
            "Exit" => break,
            _ => {}
        }
    }

    println!("Goodbye!");
    Ok(())
}

fn list_commands() -> Result<(), Box<dyn std::error::Error>> {
    let commands = storage::load_commands()?;

    if commands.is_empty() {
        println!("No commands saved yet.");
        return Ok(());
    }

    println!("\n=== Saved Commands ===");
    for (i, cmd) in commands.iter().enumerate() {
        println!("{:2}. {} -> {}", i + 1, cmd.name, cmd.command);
    }
    Ok(())
}

fn add_command() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ADD NEW COMMAND ===");

    println!("\n[STEP 1] Give this command a SHORT NAME");
    println!("Examples: my-ssh, backup-db, docker-logs");
    let name = Text::new("Name:")
        .prompt()?;

    println!("\n[STEP 2] Enter the FULL COMMAND YOU WANT TO RUN");
    println!("Example: sshpass -p \"password\" ssh user@host");
    let command = Text::new("Command:")
        .prompt()?;

    if command.is_empty() {
        println!("Error: Command cannot be empty!");
        return Ok(());
    }

    let new_cmd = commands::Command {
        name,
        command,
    };

    storage::add_command(new_cmd)?;
    println!("\n✓ Command added successfully!");
    Ok(())
}

fn delete_command() -> Result<(), Box<dyn std::error::Error>> {
    let commands = storage::load_commands()?;

    if commands.is_empty() {
        println!("No commands to delete.");
        return Ok(());
    }

    let cmd_names: Vec<&str> = commands.iter().map(|c| c.name.as_str()).collect();
    let selected = Select::new("Select command to delete:", cmd_names)
        .prompt()?;

    storage::delete_command(selected)?;
    println!("Command deleted successfully!");
    Ok(())
}

fn execute_command() -> Result<(), Box<dyn std::error::Error>> {
    let commands = storage::load_commands()?;

    if commands.is_empty() {
        println!("No commands to execute.");
        return Ok(());
    }

    let cmd_names: Vec<&str> = commands.iter().map(|c| c.name.as_str()).collect();
    let selected = Select::new("Select command to execute:", cmd_names)
        .prompt()?;

    let cmd = commands.iter().find(|c| c.name == selected)
        .ok_or("Command not found")?;

    println!("\n=== Command Preview ===");
    println!("{}", cmd.command);

    let confirm = Text::new("\nExecute this command? (y/n, or press Enter to execute):")
        .prompt()?;

    let trimmed = confirm.trim().to_lowercase();
    if trimmed == "y" || trimmed.is_empty() {
        // Disable raw mode before spawning the subprocess
        let _ = terminal::disable_raw_mode();

        // Execute the command with full terminal control
        execution::execute(&cmd.command)?;

        // Re-enable raw mode after command completes
        let _ = terminal::enable_raw_mode();
    } else {
        println!("Execution cancelled.");
    }

    Ok(())
}
