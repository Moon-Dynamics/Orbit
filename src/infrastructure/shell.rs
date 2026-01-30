use std::process::{Command, Stdio};
use colored::*;
use std::io::{self, Write};

pub fn execute(program: &str, args: &[&str], path: &str) -> Result<(), String> {

    println!("   {} Executing: {} {}".bright_black(), program, args.join(" "));

    let status = Command::new(program)
        .args(args)
        .current_dir(path) 
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to execute {}: {}", program, e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Command '{}' failed", program))
    }
}



pub fn is_installed(program: &str) -> bool {
    Command::new(program)
        .arg("--version") 
        .stdout(Stdio::null())
        .stderr(Stdio::null()) 
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}


pub fn ask_confirmation(question: &str) -> bool {
    print!("   {} {} [y/N]: ".yellow(), question);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let input = input.trim().to_lowercase();
    input == "y" || input == "yes" || input == "s" || input == "si"
}


pub fn install_script(command: &str, args: &[&str]) -> bool {
    println!("   {} Launching installer...".cyan());
    
    let status = Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())  
        .stdout(Stdio::inherit()) 
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}


pub fn execute_and_capture(program: &str, args: &[&str], path: &str) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(path)
        .stdout(Stdio::piped()) 
        .stderr(Stdio::inherit()) 
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", program, e))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(result)
    } else {
        Err(format!("Command '{}' failed", program))
    }
}


pub fn get_os_shell() -> &'static str {
    if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "sh"
    }
}