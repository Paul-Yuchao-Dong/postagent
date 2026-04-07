use std::env;

const DEFAULT_API_BASE: &str = "https://api.postagent.dev";

pub fn api_base() -> String {
    env::var("POSTAGENT_API_URL").unwrap_or_else(|_| DEFAULT_API_BASE.to_string())
}
