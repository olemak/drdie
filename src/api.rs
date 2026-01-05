use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use drdie::{parse_dice_notation, DiceRoll};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RollQuery {
    dice: Option<String>,
    explode: Option<bool>,
    keep: Option<u32>,
    success: Option<u32>,
}

#[derive(Serialize)]
struct RollResponse {
    roll: DiceRoll,
    options: RollOptions,
}

#[derive(Serialize)]
struct RollOptions {
    explode: bool,
    keep: Option<u32>,
    success: Option<u32>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn roll_handler(Query(params): Query<RollQuery>) -> impl IntoResponse {
    let notation = params.dice.unwrap_or_else(|| "1d6".to_string());

    match parse_dice_notation(&notation) {
        Ok(roll) => {
            let response = RollResponse {
                roll,
                options: RollOptions {
                    explode: params.explode.unwrap_or(false),
                    keep: params.keep,
                    success: params.success,
                },
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            let error = ErrorResponse {
                error: e.to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        }
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
