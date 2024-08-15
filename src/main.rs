use ntex::web;
use reqwest::{self, header::HeaderValue};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

#[web::get("/profile/{username}")]
async fn fetch_profile(player_name: web::types::Path<String>) -> impl web::Responder {
    let url = format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player={}",
        player_name
    );

    match reqwest::get(url).await {
        Ok(request) => {
            if request.headers().get("content-type")
                != Some(&HeaderValue::from_static("application/json"))
            {
                return web::HttpResponse::ExpectationFailed()
                    .body(format!("Invalid content type."));
            }

            let body = request.text().await.unwrap();
            web::HttpResponse::Ok().body(body)
        }
        Err(_) => {
            web::HttpResponse::NoContent().body(format!("HTTP error while fetching profile."))
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    for v in std::env::vars() {
        println!("{:?}", v);
    }
    const DB_KEY: &str = "pgsql_db";
    let connection_string = env::var(DB_KEY)
        .expect(format!("Database connection string not found. {}", DB_KEY).as_str());

    println!("Connection string: {}", connection_string);

    let pool = PgPoolOptions::new()
        .max_connections(22)
        .connect(&connection_string.to_owned())
        .await
        .expect("Failed to connect to database.");

    web::HttpServer::new(|| web::App::new().service(fetch_profile))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
