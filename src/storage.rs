use crate::commands::Command;
use serde_json;
use std::fs;
use std::path::PathBuf;

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?;
    let cmdlist_dir = config_dir.join("cmdlist");

    fs::create_dir_all(&cmdlist_dir)?;
    Ok(cmdlist_dir.join("commands.json"))
}

pub fn load_commands() -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    let path = get_config_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)?;
    let commands = serde_json::from_str(&contents)?;
    Ok(commands)
}

pub fn save_commands(commands: &[Command]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path()?;
    let json = serde_json::to_string_pretty(&commands)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn add_command(new_cmd: Command) -> Result<(), Box<dyn std::error::Error>> {
    let mut commands = load_commands()?;
    commands.push(new_cmd);
    save_commands(&commands)?;
    Ok(())
}

pub fn delete_command(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut commands = load_commands()?;
    commands.retain(|cmd| cmd.name != name);
    save_commands(&commands)?;
    Ok(())
}
