use reqwest::blocking::Client;
use serde::Deserialize;

use crate::config;

#[derive(Deserialize)]
struct ActionHit {
    name: String,
    method: String,
    path: String,
    summary: String,
}

#[derive(Deserialize)]
struct Group {
    name: String,
    #[serde(default)]
    actions: Vec<ActionHit>,
}

#[derive(Deserialize)]
struct Project {
    name: String,
    description: String,
    #[serde(default)]
    groups: Vec<Group>,
}

pub fn run(query: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("{}/api/search?q={}", config::api_base(), urlencoding(query));

    let response = match client.get(&url).send() {
        Ok(resp) => resp,
        Err(_) => {
            eprintln!("Failed to connect to postagent server.");
            std::process::exit(1);
        }
    };

    if !response.status().is_success() {
        let body: serde_json::Value = response.json()?;
        if let Some(error) = body.get("error").and_then(|v| v.as_str()) {
            eprintln!("{}", error);
        }
        std::process::exit(1);
    }

    let body_text = response.text()?;

    if format == "json" {
        // Re-parse and pretty-print the raw JSON
        let value: serde_json::Value = serde_json::from_str(&body_text)?;
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }

    let data: Vec<Project> = serde_json::from_str(&body_text)?;

    if data.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    let output: Vec<String> = data
        .iter()
        .map(|p| {
            let mut lines = vec![
                p.name.clone(),
                format!("  {}", p.description),
            ];
            if !p.groups.is_empty() {
                let has_actions = p.groups.iter().any(|g| !g.actions.is_empty());
                if has_actions {
                    for g in &p.groups {
                        if g.actions.is_empty() {
                            continue;
                        }
                        lines.push(format!("  {}", g.name));
                        for a in &g.actions {
                            lines.push(format!("    {} {} {}  {}", a.name, a.method, a.path, a.summary));
                        }
                    }
                } else {
                    let group_names: Vec<&str> = p.groups.iter().map(|g| g.name.as_str()).collect();
                    lines.push(format!("  Groups: {}", group_names.join(", ")));
                }
            }
            lines.join("\n")
        })
        .collect();
    println!("{}", output.join("\n\n"));
    Ok(())
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "%20".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlencoding_preserves_alphanumeric() {
        assert_eq!(urlencoding("abc123"), "abc123");
        assert_eq!(urlencoding("ABC"), "ABC");
    }

    #[test]
    fn urlencoding_preserves_unreserved_chars() {
        assert_eq!(urlencoding("-_.~"), "-_.~");
    }

    #[test]
    fn urlencoding_encodes_spaces_as_percent20() {
        assert_eq!(urlencoding("hello world"), "hello%20world");
    }

    #[test]
    fn urlencoding_encodes_special_characters() {
        assert_eq!(urlencoding("a+b"), "a%2Bb");
        assert_eq!(urlencoding("a&b=c"), "a%26b%3Dc");
    }

    #[test]
    fn urlencoding_encodes_slash() {
        assert_eq!(urlencoding("path/to/resource"), "path%2Fto%2Fresource");
    }

    #[test]
    fn urlencoding_empty_string() {
        assert_eq!(urlencoding(""), "");
    }

    #[test]
    fn urlencoding_all_spaces() {
        assert_eq!(urlencoding("   "), "%20%20%20");
    }

    #[test]
    fn urlencoding_mixed_content() {
        assert_eq!(urlencoding("search query!"), "search%20query%21");
    }
}
