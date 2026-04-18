use crate::parser::AvcData;
use crate::rules;
use colored::Colorize;

/// Helper function to extract the 3rd part of a SELinux context (the Type).
/// Example: "system_u:system_r:httpd_t:s0" -> "httpd_t"
fn extract_type(context: &str) -> &str {
    // We split by ':', take the element at index 2 (the third one),
    // and if something goes wrong, we just return the whole context or "unknown".
    context.split(':').nth(2).unwrap_or(context)
}

/// Returns specific actionable advice based on the combination of source type and action.
fn get_specific_advice(
    source_type: &str,
    action: &str,
    tclass: &str,
    target_type: &str,
    target: &str,
    report: bool,
) -> String {
    // Load rules with fallback: system path → local file → embedded binary
    let rules_file = rules::load_rules_with_fallback();

    if let Some(rule) = rules::find_rule(&rules_file, source_type, action, tclass) {
        return format!("{}\n{}", rule.suggestion, rule.fix);
    }

    // Fallback to hardcoded match
    match (source_type, action, tclass) {
        // Case 1: Web server trying to READ files or directories
        ("httpd_t", "read" | "open" | "getattr", "file" | "dir") => {
                "The web server is trying to read a file or directory labeled '{target_type}'.\n\
                Fix its context with: `sudo restorecon -Rv </path/to/{target}>`\n\
                Or set it manually: `sudo chcon -t httpd_sys_content_t </path/to/{target}>`".to_string()
        }

        // Case 2: Web server trying to CONNECT to a network socket
        ("httpd_t", "name_connect", "tcp_socket") => {
            format!(
                "The web server is trying to make an outbound network connection.\n\
                 By default, SELinux blocks this. To allow it, run:\n\
                 `sudo setsebool -P httpd_can_network_connect 1`"
            )
        }

        // Case 3: Containers (we use `_` because we care about ANY action blocked for containers)
        ("container_t", _, _) => {
                "A container is trying to access a host resource.\n\
                 If you are mounting a volume, append ':z' or ':Z' to your volume path.\n\
                 Example: `-v /host/path:/container/path:z`".to_string()
        }

        // Default fallback for any other combination
        _ => {
            let mut advice = format!(
                "No specific advice for process '{}' attempting to '{}' on a '{}'.\n\
                 Check if the process has the correct labels to access '{}'.",
                source_type, action, tclass, target_type
            );

            if report {
                let url = generate_issue_url(source_type, action, tclass);

                let link_text = "Click here to open GitHub and create new issue with this rule."
                    .underline()
                    .bold();
                let clickable_link = format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, link_text);

                advice.push_str(&format!(
                    "\n\n\n{}{}\n{}",
                    "⚠️ ".yellow(),
                    "This denial has no rule yet. Help improve selinux-explain!"
                        .yellow()
                        .bold(),
                    clickable_link
                ));
            }

            advice
        }
    }
}

/// Takes the extracted SELinux data and prints a human-readable explanation
pub fn print_explanation(data: &AvcData, report: bool) {
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
        &data.target,
        report,
    );

    // Print the tailored advice
    println!("{}", advice);
}

/// Minimal URL encoder for GitHub issue query parameters.
fn url_encode(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('\n', "%0A")
        .replace('"', "%22")
        .replace('[', "%5B")
        .replace(']', "%5D")
        .replace('#', "%23")
        .replace(':', "%3A")
        .replace('=', "%3D")
        .replace('/', "%2F")
        .replace('?', "%3F")
        .replace('&', "%26")
}

/// Generates a pre-filled GitHub issue URL for a denial with no matching rule.
fn generate_issue_url(source_type: &str, action: &str, tclass: &str) -> String {
    let title = format!("New rule: {} {} {}", source_type, action, tclass);
    let body = format!(
        "## New rule suggestion\n\nI encountered a SELinux denial not covered by `rules.toml`.\n\n```toml\n[[rules]]\nsource_type = \"{}\"\naction = \"{}\"\ntclass = \"{}\"\nsuggestion = \"TODO: describe what is happening\"\nfix = \"TODO: add the fix command\"\n```\n\n**What I was doing when the denial occurred:**\n<!-- describe the context -->\n\n**Fix that worked for me (if any):**\n<!-- add it here so it can be included in the next release -->",
        source_type, action, tclass
    );
    format!(
        "https://github.com/mattiabandini1/selinux-explain/issues/new?title={}&body={}",
        url_encode(&title),
        url_encode(&body)
    )
}
