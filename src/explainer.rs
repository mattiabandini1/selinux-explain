use colored::Colorize;
use crate::parser::AvcData;

/// Takes the extracted SELinux data and prints a human-readable explanation
pub fn print_explanation(data: &AvcData) {
    // Let's print a flashy header
    println!("\n{}", "🚨 SELinux Denial Detected!".red().bold());
    println!("{}", "=============================".red());

    println!("\n{} {}", "Who:".bold(), data.process.yellow().bold());
    println!(
        "{} Tried to '{}' the target '{}'",
        "What:".bold(),
        data.action.cyan().bold(),
        data.target.cyan().bold()
    );
    // For now, let's print a generic suggestion. 
    // Later we can add specific logic based on the context.
    println!("\n{}", "💡 Suggestion:".green().bold());
    println!("Check if the process has the correct labels to access this file.");
    println!("You can temporarily test if SELinux is the issue by running: `setenforce 0`");
    println!("(But remember to re-enable it with `setenforce 1`!)\n");
}
