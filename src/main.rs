use axum::{
    Router,
    extract::RawQuery,
    http::{HeaderName, HeaderValue, Method, header},
    response::IntoResponse,
    routing::get,
};
use base64::{Engine as _, prelude::BASE64_STANDARD};
use tower_http::cors::CorsLayer;
use url::form_urlencoded;

async fn root(RawQuery(raw): RawQuery) -> impl IntoResponse {
    let mut content_type = "text/html; charset=utf-8".to_string();
    let mut body = Vec::new();

    if let Some(q) = raw {
        for (key, value) in form_urlencoded::parse(q.as_bytes()) {
            match &*key {
                "t" => content_type = value.into_owned(),
                "b" => body = value.into_owned().into_bytes(),
                _ => {}
            }
        }
    }

    if !content_type.starts_with("text/")
        && let Ok(decoded) = BASE64_STANDARD.decode(&body)
    {
        body = decoded;
    }

    (
        [
            (header::CONTENT_TYPE, content_type),
            (
                HeaderName::from_static("X-THANKS"),
                "Thanks to Leapcell for hosting.".to_string(),
            ),
        ],
        body,
    )
}

#[tokio::main]
async fn main() {
    let allowed_origins: [HeaderValue; 2] = [
        "http://127.0.0.1:3000".parse().unwrap(),
        "https://qs-to-body.leapcell.app/".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::OPTIONS]);

    let app = Router::new().route("/", get(root)).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
