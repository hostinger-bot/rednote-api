mod scraper;

use axum::{
    extract::{Json, Query},
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use scraper::{is_valid_rednote_url, scrape};
use serde::Deserialize;
use serde_json::{json, to_string_pretty};
use std::{env, net::SocketAddr, time::Instant};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct Params {
    url: String,
}

async fn logging_middleware(req: Request<axum::body::Body>, next: Next) -> impl IntoResponse {
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

async fn docs() -> Html<String> {
    tokio::fs::read_to_string("templates/docs.html")
        .await
        .map(Html)
        .unwrap_or_else(|_| Html("<h1>docs.html tidak ditemukan</h1>".into()))
}

async fn openapi_json() -> impl IntoResponse {
    match tokio::fs::read_to_string("static/openapi.json").await {
        Ok(content) => ([(header::CONTENT_TYPE, "application/json")], content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "openapi.json tidak ada").into_response(),
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

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on http://localhost:{port}");

    let app = Router::new()
        .route("/api/rednote", get(api_handler).post(api_handler_post))
        .route("/", get(docs))
        .route("/docs", get(docs))
        .route("/openapi.json", get(openapi_json))
        .fallback(not_found)
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
