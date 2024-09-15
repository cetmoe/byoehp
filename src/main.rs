pub mod api;
pub mod database;

use crate::api::connection::establish_connection;
use crate::database::models::*;
use diesel::prelude::*;
// use reqwest::{self, header::HeaderValue};
// use serde_json::{json, Value};
// use std::env;
// use tokio::net::TcpListener;

// use axum::{
//     extract::Path,
//     http::StatusCode,
//     routing::{get, post},
//     Json, Router,
// };

// const RUNEMETRICS_PROFILE_URL: &str =
//     "https://apps.runescape.com/runemetrics/profile/profile?user=mowws";

// async fn fetch_profile(Path(name): Path<String>) -> Json<Value> {
//     let url = format!(
//         "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player={}",
//         name
//     );

//     match reqwest::get(url.clone()).await {
//         Ok(request) => {
//             if request.headers().get("content-type")
//                 != Some(&HeaderValue::from_static("application/json"))
//             {
//                 return Json(json!({"Error": 32, "hehe": url.clone()}));
//             }

//             let body = request.json().await.unwrap();
//             Json(body)
//         }
//         Err(_) => Json(json!({"Error": 42})),
//     }
// }

#[tokio::main]
async fn main() {
    //.route("/profile/:character_name", get(fetch_profile))
    use crate::database::schema::characters::dsl::*;

    // let app = Router::new();
    let connection = &mut establish_connection();
    let results = characters
        .limit(5)
        .load::<Character>(connection)
        .expect("Error loading characters");

    println!("Displaying {} characters", results.len());

    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
