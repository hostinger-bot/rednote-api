mod scraper;

use axum::{
    extract::{Json, Query},
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use scraper::{is_valid_rednote_url, scrape};
use serde::Deserialize;
use serde_json::{json, to_string_pretty};
use std::{env, time::Instant};
use tower_http::cors::CorsLayer;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Params {
    url: String,
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticFiles;

async fn logging_middleware(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> impl IntoResponse {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().to_string();

    eprintln!(">>> {method} {uri}");
    let response = next.run(req).await;

    let duration = start.elapsed();
    eprintln!(
        "<<< {method} {uri} {} {duration:.2?}",
        response.status().as_u16()
    );

    response
}

async fn api_handler(Query(params): Query<Params>) -> impl IntoResponse {
    process_url(params.url).await
}

async fn api_handler_post(Json(params): Json<Params>) -> impl IntoResponse {
    process_url(params.url).await
}

async fn process_url(url: String) -> impl IntoResponse {
    if !is_valid_rednote_url(&url) {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            json!({
                "status": false,
                "error": "Invalid Xiaohongshu URL"
            })
            .to_string(),
        )
            .into_response();
    }

    match scrape(url).await {
        Ok(data) => {
            let mut obj = serde_json::to_value(data).unwrap_or(json!({}));
            if let serde_json::Value::Object(ref mut map) = obj {
                map.insert("status".to_string(), json!(true));
            }

            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                to_string_pretty(&obj).unwrap_or(
                    json!({
                        "status": false,
                        "error": "json serialize failed"
                    })
                    .to_string(),
                ),
            )
                .into_response()
        }
        Err(msg) => (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            json!({
                "status": false,
                "error": msg
            })
            .to_string(),
        )
            .into_response(),
    }
}

async fn docs() -> impl IntoResponse {
    match Templates::get("docs.html") {
        Some(content) => Html(String::from_utf8_lossy(content.data.as_ref()).to_string()),
        None => Html("<h1>docs.html not found!</h1>".to_string()),
    }
}

async fn openapi_json() -> impl IntoResponse {
    match StaticFiles::get("openapi.json") {
        Some(content) => (
            [(header::CONTENT_TYPE, "application/json")],
            String::from_utf8_lossy(content.data.as_ref()).to_string(),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "openapi.json not found!").into_response(),
    }
}

async fn not_found() -> (StatusCode, Html<&'static str>) {
    (StatusCode::NOT_FOUND, Html("<h1>404 - Not Found</h1>"))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("Server running on http://localhost:{port}");

    let app = Router::new()
        .route("/api/rednote", get(api_handler).post(api_handler_post))
        .route("/", get(docs))
        .route("/docs", get(docs))
        .route("/openapi.json", get(openapi_json))
        .fallback(not_found)
        .layer(CorsLayer::permissive())
        .layer(axum::middleware::from_fn(logging_middleware));

    axum::serve(listener, app).await.unwrap();
}