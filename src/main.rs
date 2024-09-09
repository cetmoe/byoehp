use dotenv::dotenv;
use reqwest::{self, header::HeaderValue};
use serde_json::{json, Value};
use std::env;
use tokio::net::TcpListener;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

const X: &str = "https://apps.runescape.com/runemetrics/profile/profile?user=mowws";

async fn fetch_profile(Path(name): Path<String>) -> Json<Value> {
    let url = format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player={}",
        name
    );

    match reqwest::get(url.clone()).await {
        Ok(request) => {
            if request.headers().get("content-type")
                != Some(&HeaderValue::from_static("application/json"))
            {
                return Json(json!({"Error": 32, "hehe": url.clone()}));
            }

            let body = request.json().await.unwrap();
            Json(body)
        }
        Err(_) => Json(json!({"Error": 42})),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    const DB_KEY: &str = "pgsql_db";
    let connection_string = env::var(DB_KEY)
        .expect(format!("Database connection string not found. {}", DB_KEY).as_str());

    let app = Router::new().route("/profile/:character_name", get(fetch_profile));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
