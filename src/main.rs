use std::io::{self, Read, IsTerminal};
use colored::Colorize;

mod parser;
mod explainer;
mod reader;
mod rules;

use clap::Parser;

/// selinux-explain: A human-readable translator for SELinux AVC denials
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Analyze the last AVC denial generated in the system audit log
    #[arg(short, long)]
    last: bool,

    /// Analyze a specific log passed as text (useful for testing)
    #[arg(short = 't', long = "text")]
    log_text: Option<String>,
}

/// Helper function to avoid duplicating the parsing and explaining logic.
/// It takes a raw log string, parses it, and prints the human-readable explanation or an error.
fn process_and_explain_log(log_line: &str) {
    let parsed_result = parser::parse_avc_log(log_line);

    match parsed_result {
        Some(data) => {
            explainer::print_explanation(&data);
        },            
        None => {
            println!("Could not parse the log. Are you sure is a valid SELinux AVC denial?");
        }
    }

}

fn main() {
    let cli = Cli::parse();

    if cli.last { 
        let last_result = reader::get_last_denial("/var/log/audit/audit.log");
        
        match last_result {
            Ok(Some(text)) => { 
                process_and_explain_log(&text);    
            },
            Ok(None) => {
                println!("No SELinux denials found in the log file.");
            },
            Err(e) => {
                // This happens if the OS blocks us (e.g., missing sudo) or file doesn't exist
                println!("{} {}", "Error reading the log file:".red().bold(), e);
                println!("Tip: The audit.log file usually requires root privileges. Try running the command with 'sudo'.");
            }
        }
    
    } else if let Some(text) = cli.log_text { 
            process_and_explain_log(&text);
    } else if !io::stdin().is_terminal() {
        let mut input = String::new();

        // Read all piped data into our string
        if let Err(e) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading from standard input: {}", e);
            return;
        }

        let mut found = false;

        // Process each line that looks like an SELinux denial
        for line in input.lines() {
            if line.contains("avc: denied") || line.contains("type=AVC") {
                process_and_explain_log(line);
                found = true;
            }
        }
        if !found {
            println!("No SELinux denials found in the piped input.");
        }

    } else {
        println!("No arguments provided!");
        println!("Tip: Use 'selinux-explain --help' to see available options.");
    }
}
