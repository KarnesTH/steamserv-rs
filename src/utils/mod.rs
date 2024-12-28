pub mod config;
pub mod progress;

use std::{
    io::{BufRead, Write},
    thread,
    time::Duration,
};

pub use config::{Config, InstalledServer, ServerCache, ServerInfo};
pub use progress::{default_spinner, Progress, ProgressStyle};

/// Run a command with a spinner
///
/// # Arguments
///
/// - `command` - The command to run
/// - `message` - The message to display with the spinner
///
/// # Returns
///
/// Ok if the command was run successfully
///
/// # Errors
///
/// If the command could not be run
pub fn run_with_spinner(
    command: &mut std::process::Child,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = Progress::new(1, message, default_spinner()?)?;

    if let Some(stdout) = command.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        for _ in reader.lines().map_while(Result::ok) {
            spinner.tick()?;
            thread::sleep(Duration::from_millis(100));
        }
    }

    spinner.finish()?;
    Ok(())
}

/// Run a command and print the output
///
/// # Arguments
///
/// - `command` - The command to run
///
/// # Returns
///
/// Ok if the command was run successfully
///
/// # Errors
///
/// If the command could not be run
pub fn run_with_output(
    command: &mut std::process::Child,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    if let Some(stdout) = command.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if line.contains("Redirecting stderr")
                || line.contains("UpdateUI")
                || line.contains("ILocalize")
            {
                continue;
            }

            if line.starts_with('[') {
                println!("Status: {}", line);
            } else {
                println!("{}", line);
            }
            std::io::stdout().flush()?;
        }
    }
    println!();
    Ok(())
}
