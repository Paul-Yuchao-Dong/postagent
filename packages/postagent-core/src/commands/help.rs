use reqwest::blocking::Client;
use serde::Deserialize;

use crate::config;

#[derive(Deserialize)]
struct ResourceSummary {
    name: String,
    actions: Vec<String>,
}

#[derive(Deserialize)]
struct ActionSummary {
    name: String,
    method: String,
    path: String,
    summary: String,
}

#[derive(Deserialize)]
struct Parameter {
    name: String,
    #[serde(rename = "in")]
    location: String,
    #[serde(rename = "type")]
    param_type: String,
    required: bool,
    description: String,
}

#[derive(Deserialize)]
struct RequestBody {
    #[serde(rename = "contentType")]
    content_type: String,
    schema: serde_json::Value,
}

#[derive(Deserialize)]
struct ResponseInfo {
    status: String,
    description: String,
}

pub fn run(
    project: Option<&str>,
    resource: Option<&str>,
    action: Option<&str>,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let project = match project {
        Some(p) => p,
        None => {
            // No project specified — print general help via clap
            // We return a special error that main.rs can handle
            return Err("show_help".into());
        }
    };

    let mut params = vec![("project", project.to_string())];
    if let Some(r) = resource {
        params.push(("resource", r.to_string()));
    }
    if let Some(a) = action {
        params.push(("action", a.to_string()));
    }

    let query_string: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding(v)))
        .collect::<Vec<_>>()
        .join("&");

    let client = Client::new();
    let url = format!("{}/api/help?{}", config::api_base(), query_string);

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
        if let Some(available) = body.get("available").and_then(|v| v.as_array()) {
            let items: Vec<&str> = available
                .iter()
                .filter_map(|v| v.as_str())
                .collect();
            eprintln!("Available: {}", items.join(", "));
        }
        std::process::exit(1);
    }

    let body_text = response.text()?;

    if format == "json" {
        let value: serde_json::Value = serde_json::from_str(&body_text)?;
        println!("{}", serde_json::to_string_pretty(&value)?);
        return Ok(());
    }

    let data: serde_json::Value = serde_json::from_str(&body_text)?;

    if resource.is_none() {
        // Level 1: project → list resources
        let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let description = data.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let resources: Vec<ResourceSummary> = serde_json::from_value(
            data.get("resources").cloned().unwrap_or(serde_json::Value::Array(vec![])),
        )?;

        println!("{}", name);
        println!("  {}", description);
        println!();
        println!("Resources:");
        for r in &resources {
            println!("  {}  ({})", r.name, r.actions.join(", "));
        }
    } else if action.is_none() {
        // Level 2: resource → list actions
        let resource_name = data.get("resource").and_then(|v| v.as_str()).unwrap_or("");
        let actions: Vec<ActionSummary> = serde_json::from_value(
            data.get("actions").cloned().unwrap_or(serde_json::Value::Array(vec![])),
        )?;

        println!("{} > {}", project, resource_name);
        println!();
        println!("Actions:");
        for a in &actions {
            println!("  {}  {} {}  {}", a.name, a.method, a.path, a.summary);
        }
    } else {
        // Level 3: action detail
        let proj = data.get("project").and_then(|v| v.as_str()).unwrap_or("");
        let res = data.get("resource").and_then(|v| v.as_str()).unwrap_or("");
        let act = data.get("action").and_then(|v| v.as_str()).unwrap_or("");
        let method = data.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let path = data.get("path").and_then(|v| v.as_str()).unwrap_or("");
        let description = data.get("description").and_then(|v| v.as_str()).unwrap_or("");

        println!("{} > {} > {}", proj, res, act);
        println!();
        println!("  {} {}", method, path);
        println!();
        println!("  {}", description);

        if let Some(params_val) = data.get("parameters") {
            let params: Vec<Parameter> = serde_json::from_value(params_val.clone())?;
            if !params.is_empty() {
                println!();
                println!("Parameters:");
                for p in &params {
                    let req = if p.required { "required" } else { "optional" };
                    println!(
                        "  --{}  <{}>  ({}, {})  {}",
                        p.name, p.param_type, p.location, req, p.description
                    );
                }
            }
        }

        if let Some(body_val) = data.get("requestBody") {
            if !body_val.is_null() {
                let body: RequestBody = serde_json::from_value(body_val.clone())?;
                println!();
                println!("Request Body ({}):", body.content_type);
                println!("{}", serde_json::to_string_pretty(&body.schema)?);
            }
        }

        if let Some(responses_val) = data.get("responses") {
            let responses: Vec<ResponseInfo> = serde_json::from_value(responses_val.clone())?;
            if !responses.is_empty() {
                println!();
                println!("Responses:");
                for r in &responses {
                    println!("  {}  {}", r.status, r.description);
                }
            }
        }
    }

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

    // Test the urlencoding function (same as in search.rs but local copy)
    #[test]
    fn urlencoding_basic() {
        assert_eq!(urlencoding("hello world"), "hello%20world");
        assert_eq!(urlencoding("abc123"), "abc123");
        assert_eq!(urlencoding("-_.~"), "-_.~");
    }

    #[test]
    fn urlencoding_special_chars() {
        assert_eq!(urlencoding("a+b"), "a%2Bb");
        assert_eq!(urlencoding("foo@bar"), "foo%40bar");
    }

    // Test that run returns show_help error when project is None
    #[test]
    fn run_without_project_returns_show_help() {
        let result = run(None, None, None, "text");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "show_help");
    }
}
