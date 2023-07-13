use std::fs;

use axum::{routing::get, Json, Router};

use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
struct Config {
    port: u32,
}

#[tokio::main]
async fn main() {
    let config = fs::read_to_string("config.toml").expect("Unable to read file");
    let json: Config = toml::from_str(&config).expect("JSON was not well-formatted");
    // build our application with a single route
    let app = Router::new().route("/", get(get_data));

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{}", json.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct SnakeData {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
    version: String,
}

async fn get_data() -> Json<SnakeData> {
    Json(SnakeData {
        apiversion: "1".to_string(),
        author: "BrokenKeyboard".to_string(),
        color: "#0a571e".to_string(),
        head: "dragon".to_string(),
        tail: "cosmic-horror".to_string(),
        version: "1".to_string(),
    })
}

async fn get_move() {}
