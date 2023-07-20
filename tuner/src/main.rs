use std::{fs, path::Path};

use eval::area_eval::AreaEval;
use nalgebra::SVector;
use serde::{Deserialize, Serialize};
use snake_tuner::{activation::functions::Sigmoid, database, evaluation::evaluations::Linear};
use treesearch::Search;

#[derive(Deserialize, Serialize)]
struct DB {
    inputs: Vec<SVector<f64, 5>>,
    outputs: Vec<f64>,
}

#[derive(Deserialize)]
struct Config {
    weights: [f64; 5],
    db_path: String,
}

fn main() {
    // let mut weights = [
    //     0.02887396891287721,
    //     -0.024065560310748118,
    //     -0.00024165487017368143,
    //     0.038723174814708515,
    //     0.00470536898375267,
    // ];

    let config = fs::read_to_string("config.toml").expect("Unable to read file");
    let config: Config = toml::from_str(&config).expect("Config was not well-formatted");
    let mut eval = AreaEval {
        eval: Linear::<5, Sigmoid>::from_weights(SVector::from(config.weights), Sigmoid),
    };
    println!("Opening DB");
    let database;
    if Path::new("database.json").exists() {
        println!("found old");
        let db = fs::read_to_string("database.json").unwrap();
        database = serde_json::from_str(&db).unwrap();
    } else {
        println!("scanning in from sql");
        let db = combat_adapter::DB::new(config.db_path, 2);
        let x = serde_json::to_string(&db).unwrap();
        fs::write("database.json", x).unwrap();
        database = db;
    }

    // let mut io = vec![];
    println!("Starting iteration loop");
    for x in 0..1000 {
        // println!("iteration {x}");
        let frame = database.positions[x].clone();
        let mut search = Search::new(frame, eval.clone());
        for y in 0..300 {
            // println!("iterating search {y}");
            search.iterate()
        }
        if (0.5 - search.root_score()).abs() > 0.1 {
            println!("e");
            println!("{x}");
        }
        // println!("{:?}", (1.0 - search.root_score()).abs() > 0.05);
    }
}
