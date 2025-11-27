# cmdlist - Command List Manager

A CLI tool written in Rust to save, manage, and execute custom commands with named aliases.

## Features

- **List commands**: View all your saved commands
- **Add commands**: Save new commands with custom names
- **Delete commands**: Remove commands you no longer need
- **Execute commands**: Run saved commands with full terminal control
- **JSON storage**: Commands are stored in `~/.config/cmdlist/commands.json`

## Building

```bash
cargo build --release
```

The binary will be at `target/release/cmdlist`

## Installation

You can install it globally:

```bash
cargo install --path .
```

## Usage

Run the application:

```bash
cmdlist
```

### Adding a Command

1. Select "Add command"
2. Enter a short name for your command (e.g., "ssh-server1")
3. Enter the complete command exactly as you want it to run

### Examples

**SSH with password:**
- Name: `ssh-server1`
- Command: `sshpass -p "your_password" ssh nusendra@192.168.100.100`

**List Docker containers:**
- Name: `docker-ps`
- Command: `docker ps -a`

**Database backup:**
- Name: `backup-db`
- Command: `mysqldump -u root -p mypassword mydatabase > backup.sql`

**Git push:**
- Name: `git-push-main`
- Command: `git add . && git commit -m "Update" && git push origin main`

## Configuration

Commands are stored in JSON format at `~/.config/cmdlist/commands.json`. You can edit this file directly if needed.

Example:
```json
[
  {
    "name": "ssh-server1",
    "command": "sshpass -p \"password123\" ssh nusendra@192.168.100.100"
  },
  {
    "name": "docker-ps",
    "command": "docker ps -a"
  },
  {
    "name": "backup-db",
    "command": "mysqldump -u root -p mypassword mydatabase > backup.sql"
  }
]
```

## Notes

- When executing, the command will be shown for review before execution
- You must confirm execution with `y` or `n`
- The command runs exactly as you saved it with full terminal control
- For interactive commands (SSH, bash, etc.), you'll have full access to the terminal
