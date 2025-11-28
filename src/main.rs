use std::str::FromStr;

use axum::{
    Router,
    extract::RawQuery,
    http::{HeaderName, header},
    response::IntoResponse,
    routing::get,
};
use url::form_urlencoded;

async fn root(RawQuery(raw): RawQuery) -> impl IntoResponse {
    let mut content_type = "text/html; charset=utf-8".to_string();
    let mut body = String::new();

    if let Some(q) = raw {
        for (key, value) in form_urlencoded::parse(q.as_bytes()) {
            match &*key {
                "t" => content_type = value.into_owned(),
                "b" => body = value.into_owned(),
                _ => {}
            }
        }
    }

    (
        [
            (header::CONTENT_TYPE, content_type),
            (
                HeaderName::from_str("X-THANKS").unwrap(),
                "Thanks to Leapcell for hosting.".to_string(),
            ),
        ],
        body,
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
