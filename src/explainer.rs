use colored::Colorize;
use crate::parser::AvcData;

/// Helper function to extract the 3rd part of a SELinux context (the Type).
/// Example: "system_u:system_r:httpd_t:s0" -> "httpd_t"
fn extract_type(context: &str) -> &str {
    // We split by ':', take the element at index 2 (the third one),
    // and if something goes wrong, we just return the whole context or "unknown".
    context.split(':').nth(2).unwrap_or(context)
}

/// Returns specific actionable advice based on the combination of source type and action.
fn get_specific_advice(source_type: &str, action: &str, tclass: &str, target_type: &str, target: &str) -> String {
    // We match on a tuple: (source_type, action)
    match (source_type, action, tclass) {
        // Case 1: Web server trying to READ files or directories
        ("httpd_t", "read" | "open" | "getattr", "file" | "dir") => {
            format!(
                "The web server is trying to read a file or directory labeled '{target_type}'.\n\
                Fix its context with: `sudo restorecon -Rv </path/to/{target}>`\n\
                Or set it manually: `sudo chcon -t httpd_sys_content_t </path/to/{target}>`"
            )
        },
        
        // Case 2: Web server trying to CONNECT to a network socket
        ("httpd_t", "name_connect", "tcp_socket") => {
            format!(
                "The web server is trying to make an outbound network connection.\n\
                 By default, SELinux blocks this. To allow it, run:\n\
                 `sudo setsebool -P httpd_can_network_connect 1`"
            )
        },
        
        // Case 3: Containers (we use `_` because we care about ANY action blocked for containers)
        ("container_t", _, _) => {
            format!(
                "A container is trying to access a host resource.\n\
                 If you are mounting a volume, append ':z' or ':Z' to your volume path.\n\
                 Example: `-v /host/path:/container/path:z`"
            )
        },

        // Default fallback for any other combination
        _ => {
            format!(
                "No specific advice for process '{source_type}' attempting to '{action}' on a '{tclass}'.\n\
                 Check if the process has the correct labels to access '{target_type}'.\n\
                 Test temporarily by running: `sudo setenforce 0` (Re-enable with `setenforce 1`!)"
            )
        }
    }
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

    println!("\n{}", "💡 Suggestion:".green().bold());
    // Call our new helper function, passing the action as well!
    let advice = get_specific_advice(
        source_type, 
        data.action.as_str(),
        data.tclass.as_str(),
        target_type, 
        &data.target
    );
    
    // Print the tailored advice
    println!("{}", advice);    
}
