mod parser;

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
        // TODO: Call the module that reads the audit.log file
        
    } else if let Some(text) = cli.log_text {
        println!("Text mode: Analyzing the provided log...");
        println!("Received text: {}", text);
        // TODO: Pass the text to our SELinux parser module
        
    } else {
        // If the user runs the command without any arguments
        println!("No arguments provided!");
        println!("Tip: Use 'cargo run -- --help' to see available options.");
    }
}
