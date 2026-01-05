use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use drdie::{roll_dice, RollOptions};
use serde::Deserialize;

#[derive(Deserialize)]
struct RollQuery {
    dice: Option<String>,
    explode: Option<bool>,
    keep: Option<u32>,
    drop: Option<u32>,
    success: Option<u32>,
    crit: Option<u32>,
}

async fn roll_handler(Query(params): Query<RollQuery>) -> impl IntoResponse {
    let notation = params.dice.unwrap_or_else(|| "1d6".to_string());

    let options = RollOptions {
        explode: params.explode.unwrap_or(false),
        keep: params.keep,
        drop: params.drop,
        success: params.success,
        crit: params.crit,
    };

    match roll_dice(&notation, &options) {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/roll", get(roll_handler))
        .route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("API server running on http://127.0.0.1:3000");
    println!("Try: http://127.0.0.1:3000/roll?dice=3d6&explode=true&keep=2");

    axum::serve(listener, app).await.unwrap();
}
