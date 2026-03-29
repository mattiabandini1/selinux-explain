use colored::Colorize;

mod parser;
mod explainer;
mod reader;

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

fn main() {
    // This single line parses the user's terminal input into our Cli struct
    let cli = Cli::parse();

    // Handle the user's input based on the provided flags
    if cli.last {
        println!("Mode '--last': Searching for the latest denial in the system log...");
       
        let last_result = reader::get_last_denial("/var/log/audit/audit.log");
        
        match last_result {
            Ok(Some(text)) => {
                println!("Found the latest SELinux denial!");
                
                // 1. Call our parser function passing the text.
                // We borrow the text using `&text` because the function expects a string slice (&str).
                let parsed_result = parser::parse_avc_log(&text);

                // 2. Handle the Option returned by the parser using a `match` statement.
                match parsed_result {
                    // If parsing succeeded, print the extracted struct
                    Some(text) => {
                        explainer::print_explanation(&text);
                    },
                    None => {
                        // If parsing failed (regex didn't match)
                        println!("Could not parse the log. Are you sure is a valid SELinux AVC denial?");
                    }
                }
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
        println!("Text mode: Analyzing the provided log...");
        println!("Received text: {}", text);
        
        // 1. Call our parser function passing the text.
        // We borrow the text using `&text` because the function expects a string slice (&str).
        let parsed_result = parser::parse_avc_log(&text);

        // 2. Handle the Option returned by the parser using a `match` statement.
        match parsed_result {
            Some(data) => {
                // If parsing succeeded, print the extracted struct
                explainer::print_explanation(&data);
            },
            None => {
                // If parsing failed (regex didn't match)
                println!("Could not parse the log. Are you sure is a valid SELinux AVC denial?");
            }
        }
        
    } else {
        // If the user runs the command without any arguments
        println!("No arguments provided!");
        println!("Tip: Use 'cargo run -- --help' to see available options.");
    }
}
