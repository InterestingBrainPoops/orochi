use std::{
    fs,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Instant,
};

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};

use board::incoming_board::Request;
use eval::area_eval::AreaEval;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

#[derive(Deserialize)]
struct Config {
    port: u32,
    weights: [f64; 5],
}
#[tokio::main]
async fn main() {
    let config = fs::read_to_string("config.toml").expect("Unable to read file");
    let config: Config = toml::from_str(&config).expect("JSON was not well-formatted");
    // build our application with a single route

    let app = Router::new()
        .route("/", get(get_data))
        .route("/move", post(get_move))
        .route("/start", post(start));

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{}", config.port).parse().unwrap())
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
async fn start() {}
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

#[derive(Serialize)]
struct OutMove {
    #[serde(rename = "move")]
    out_move: String,
    shout: String,
}
async fn get_move(Json(request): Json<Request>) -> Json<OutMove> {
    let mut usable = request.into_usable();
    let t0 = Instant::now();
    // let (returned_move, score) = state
    //     .write()
    //     .unwrap()
    //     .search
    //     .iterative_deepen(&mut usable, 2);
    // let t1 = Instant::now();
    // let stats = state.read().unwrap().search.statistics;
    // println!("search time {:?}", (t1 - t0));
    // let tdiff = (t1 - t0).as_secs_f64();
    // let move_string = match returned_move.move_type {
    //     MoveType::MoveSquare(square) => {
    //         let diff = usable.board.snakes[usable.you_id].body[0].trailing_zeros() as i32
    //             - square.trailing_zeros() as i32;
    //         match diff {
    //             -1 => "right".to_string(),
    //             1 => "left".to_string(),
    //             -11 => "up".to_string(),
    //             11 => "down".to_string(),
    //             _ => {
    //                 panic!("Invalid move");
    //             }
    //         }
    //     }
    //     MoveType::Death => String::from_str("up").unwrap(),
    // };
    // println!("Score: {}", score);
    // println!("Move direction : {}", move_string);
    // println!(
    //     "NPS: {}, TT Hits: {}, Nodes: {}",
    //     stats.node_count as f64 / tdiff,
    //     stats.tt_hits,
    //     stats.node_count
    // );
    Json(OutMove {
        out_move: "up".to_string(),
        shout: String::from_str("hello").unwrap(),
    })
}
