use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub source_type: String,
    pub action: String,
    pub tclass: String,
    pub suggestion: String,
    pub fix: String,
}

#[derive(Debug, Deserialize)]
pub struct RulesFile {
    pub rules: Vec<Rule>,
}

/// Tries to load rules from the given path.
/// Returns None if the file doesn't exist or can't be parsed.
pub fn load_rules(path: &str) -> Option<RulesFile> {
    let content = std::fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

/// Searches for a matching rule given source_type, action, tclass.
/// Returns a reference to the first matching rule, or None.
pub fn find_rule<'a>(
    rules: &'a RulesFile,
    source_type: &str,
    action: &str,
    tclass: &str,
) -> Option<&'a Rule> {
    rules.rules.iter().find(|r| {
        r.source_type == source_type
            && r.action == action
            && r.tclass == tclass
    })
}
