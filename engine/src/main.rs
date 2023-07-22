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

use board::{incoming_board::Request, movegen::MoveType};
use eval::area_eval::AreaEval;
use minimax::Search;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

#[derive(Deserialize)]
struct Config {
    port: u32,
    weights: [f64; 5],
}
type SharedState = Arc<RwLock<EngineState>>;

struct EngineState {
    search: Search<AreaEval>,
}
#[tokio::main]
async fn main() {
    let config = fs::read_to_string("config.toml").expect("Unable to read file");
    let config: Config = toml::from_str(&config).expect("JSON was not well-formatted");
    // build our application with a single route

    let app = Router::new()
        .route("/", get(get_data))
        .route("/move", post(get_move))
        .route("/start", post(start))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(Arc::new(RwLock::new(EngineState {
                    search: Search::new(100000, AreaEval::new(config.weights)),
                }))))
                .into_inner(),
        );

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
async fn start(Extension(state): Extension<SharedState>) {
    state.write().unwrap().search.reset();
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

#[derive(Serialize)]
struct OutMove {
    #[serde(rename = "move")]
    out_move: String,
    shout: String,
}
async fn get_move(
    Extension(state): Extension<SharedState>,
    Json(request): Json<Request>,
) -> Json<OutMove> {
    let mut usable = request.into_usable();
    let t0 = Instant::now();
    let (returned_move, score) = state
        .write()
        .unwrap()
        .search
        .iterative_deepen(&mut usable, 4);
    let t1 = Instant::now();
    let stats = state.read().unwrap().search.statistics;
    println!("search time {:?}", (t1 - t0));
    let tdiff = (t1 - t0).as_secs_f64();
    let move_string = if let MoveType::MoveSquare(square) = returned_move.move_type {
        let diff = usable.board.snakes[usable.you_id].body[0].trailing_zeros() as i32
            - square.trailing_zeros() as i32;
        match diff {
            -1 => "right".to_string(),
            1 => "left".to_string(),
            -11 => "up".to_string(),
            11 => "down".to_string(),
            _ => {
                panic!("Invalid move");
            }
        }
    } else {
        String::from_str("up").unwrap()
    };

    println!("Score: {}", score);
    println!("Move direction : {}", move_string);
    println!(
        "NPS: {}, TT Hits: {}, Nodes: {}",
        stats.node_count as f64 / tdiff,
        stats.tt_hits,
        stats.node_count
    );
    Json(OutMove {
        out_move: move_string,
        shout: String::from_str("hello").unwrap(),
    })
}
