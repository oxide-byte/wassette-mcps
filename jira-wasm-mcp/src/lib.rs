use base64::Engine;
use spin_sdk::http::{send, Method, Request, Response};

wit_bindgen::generate!({
    world: "jira",
    path: "wit",
});

struct Component;

impl Guest for Component {
    fn get_issue(issue_key: String) -> Result<String, String> {
        spin_executor::run(fetch_issue(issue_key))
    }
}

export!(Component);

const DEFAULT_URL: &str = "http://localhost:8080";

/// Builds the `Authorization` header value from the environment, if any.
///
/// `JIRA_API_KEY` (bearer) takes precedence over `JIRA_PASSWORD` (basic),
/// mirroring the native jira-mcp server.
fn auth_header() -> Option<String> {
    let username = std::env::var("JIRA_USERNAME").unwrap_or_default();
    if let Ok(api_key) = std::env::var("JIRA_API_KEY") {
        if !api_key.is_empty() {
            return Some(format!("Bearer {api_key}"));
        }
    }
    let password = std::env::var("JIRA_PASSWORD").unwrap_or_default();
    if username.is_empty() || password.is_empty() {
        return None;
    }
    let encoded =
        base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));
    Some(format!("Basic {encoded}"))
}

fn validate_issue_key(key: &str) -> Result<(), String> {
    let valid = !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if valid {
        Ok(())
    } else {
        Err(format!("Invalid issue key '{key}'"))
    }
}

async fn fetch_issue(issue_key: String) -> Result<String, String> {
    validate_issue_key(&issue_key)?;

    let base = std::env::var("JIRA_URL").unwrap_or_else(|_| DEFAULT_URL.to_string());
    let url = format!(
        "{}/rest/api/3/issue/{}",
        base.trim_end_matches('/'),
        issue_key
    );

    let mut builder = Request::builder();
    builder
        .method(Method::Get)
        .uri(&url)
        .header("Accept", "application/json");
    if let Some(auth) = auth_header() {
        builder.header("Authorization", auth);
    }

    let response: Response = send(builder.build()).await.map_err(|e| e.to_string())?;
    let status = *response.status();
    match status {
        200 => String::from_utf8(response.into_body())
            .map_err(|e| format!("Response was not valid UTF-8: {e}")),
        404 => Err(format!("Jira issue '{issue_key}' not found (404)")),
        401 => Err("Unauthorized: Check your Jira credentials (401)".to_string()),
        403 => Err(
            "Access forbidden: You do not have permission to view this issue (403)".to_string(),
        ),
        _ => {
            let body = String::from_utf8_lossy(response.body());
            Err(format!("Jira API error ({status}): {body}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_keys() {
        assert!(validate_issue_key("PROJ-123").is_ok());
        assert!(validate_issue_key("").is_err());
        assert!(validate_issue_key("A/../B").is_err());
        assert!(validate_issue_key("A?x=1").is_err());
    }
}
