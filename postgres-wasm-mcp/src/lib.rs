use spin_sdk::http::{send, Method, Request, Response};

wit_bindgen::generate!({
    world: "db-tools",
    path: "wit",
});

struct Component;

impl Guest for Component {
    fn fetch_user_from_db(user_id: i32) -> Result<String, String> {
        spin_executor::run(fetch_user(user_id))
    }
}

export!(Component);

const DEFAULT_URL: &str = "http://127.0.0.1:3000";

async fn fetch_user(user_id: i32) -> Result<String, String> {
    let base = std::env::var("POSTGREST_URL").unwrap_or_else(|_| DEFAULT_URL.to_string());
    let url = format!(
        "{}/users?select=id,name,email&id=eq.{}",
        base.trim_end_matches('/'),
        user_id
    );

    // PostgREST returns a single JSON object (or 406 if not exactly one row).
    let mut builder = Request::builder();
    builder
        .method(Method::Get)
        .uri(&url)
        .header("Accept", "application/vnd.pgrst.object+json");

    let response: Response = send(builder.build()).await.map_err(|e| e.to_string())?;
    let status = *response.status();
    match status {
        200 => String::from_utf8(response.into_body())
            .map_err(|e| format!("Response was not valid UTF-8: {e}")),
        406 => Err(format!("User {user_id} not found")),
        _ => {
            let body = String::from_utf8_lossy(response.body());
            Err(format!("PostgREST error ({status}): {body}"))
        }
    }
}
