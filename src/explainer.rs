use colored::Colorize;
use crate::parser::AvcData;

/// Helper function to extract the 3rd part of a SELinux context (the Type).
/// Example: "system_u:system_r:httpd_t:s0" -> "httpd_t"
fn extract_type(context: &str) -> &str {
    // We split by ':', take the element at index 2 (the third one),
    // and if something goes wrong, we just return the whole context or "unknown".
    context.split(':').nth(2).unwrap_or(context)
}

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

    let source_type = extract_type(&data.scontext);
    let target_type = extract_type(&data.tcontext);

    println!(
        "{} A process with label '{}' is not allowed to access objects with label '{}'",
        "Why:".bold(),
        source_type.red().bold(),
        target_type.red().bold()
    );

    // For now, let's print a generic suggestion. 
    // Later we can add specific logic based on the context.
    println!("\n{}", "💡 Suggestion:".green().bold());
    println!("Check if the process has the correct labels to access this file.");
    println!("You can temporarily test if SELinux is the issue by running: `setenforce 0`");
    println!("(But remember to re-enable it with `setenforce 1`!)\n");
}
