use ntex::{
    http::{body, Request},
    util::HashMap,
    web,
};
use reqwest::{self, header::HeaderValue};

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
            web::HttpResponse::NoContent().body(format!("HTTP error while fetching profile.",))
        }
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(fetch_profile))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
