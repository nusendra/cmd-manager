use std::process::{Command, Stdio};

pub fn execute(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Executing ===\n");

    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    };

    if !status.success() {
        eprintln!("\nCommand exited with status: {}", status);
    }

    Ok(())
}
