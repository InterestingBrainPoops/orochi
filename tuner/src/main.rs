use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use eval::area_eval::AreaEval;
use nalgebra::SVector;
use pretty_assertions::{assert_eq, assert_ne};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use snake_tuner::{
    activation::{functions::Sigmoid, ActivationFunction},
    database::{self, Database, Entry},
    dataloader::DataLoader,
    evaluation::{evaluations::Linear, Eval},
    optimizer::{optimizers::SGD, Optimizer},
};
#[derive(Deserialize, Serialize, Clone)]
struct DB {
    entries: Vec<ComputedEntry>,
}
#[derive(Deserialize, Serialize, Clone, Copy)]
struct ComputedEntry {
    input: SVector<f64, 6>,
    output: f64,
}
impl Entry<6> for ComputedEntry {
    fn get_inputs(&self) -> SVector<f64, 6> {
        self.input
    }

    fn get_expected_output(&self) -> f64 {
        self.output
    }
}
impl Database<6, ComputedEntry> for DB {
    fn size(&self) -> usize {
        self.entries.len()
    }

    fn get(&self, idx: usize) -> ComputedEntry {
        self.entries[idx]
    }
}
#[derive(Deserialize)]
struct Config {
    weights: [f64; 6],
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
        eval: Linear::<6, Sigmoid>::from_weights(SVector::from(config.weights), Sigmoid),
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
        let database_bar = Arc::new(Mutex::new(pbr::ProgressBar::new(db.positions.len() as u64)));
        let o_bar = database_bar.clone();
        let t0 = Instant::now();
        let entries = db
            .positions
            .par_iter()
            .map(move |(x, y)| {
                let input = AreaEval::label(x);
                let output = if &x.you_id == y { 1.0 } else { 0.0 };
                database_bar.lock().unwrap().inc();
                ComputedEntry { input, output }
            })
            .collect();
        o_bar.lock().unwrap().finish();
        println!("{:?}", t0.elapsed());
        database = DB { entries };
        let x = serde_json::to_string(&database).unwrap();
        fs::write("database.json", x).unwrap();
    }
    println!("{:?}", database.entries.len());
    let mut dataloader = DataLoader::new(database.clone(), 150, true);
    let mut grad = SGD::new(0.01);
    let mut sum = 0.001;

    for entry in &database.entries {
        let error = (entry.output
            - eval
                .eval
                .activation_fn()
                .evaluate(eval.eval.forward(entry.input)))
        .powf(2.0);
        sum += error;
    }
    println!("{}", sum / database.entries.len() as f64);
    for x in 0..100000 {
        // println!("{x}");
        // println!("{:?}", eval.eval);
        grad.step(&mut eval.eval, dataloader.sample());
    }
    let mut sum = 0.0;
    for entry in &database.entries {
        let error = (entry.output
            - eval
                .eval
                .activation_fn()
                .evaluate(eval.eval.forward(entry.input)))
        .powf(2.0);
        sum += error;
    }
    println!("{}", sum / database.entries.len() as f64);
    // // let mut io = vec![];
    // println!("Starting iteration loop");
    // let mut accum = 0.0;
    // let mut node_accum = 0;
    // let mut time_accum = Duration::from_secs_f64(0.0);
    // let mut hit_accum = 0;
    // let mut search = Search::new(100000, eval.clone());

    // for x in 0..50 {
    //     println!("iteration {x}");

    //     let mut frame = database.positions[x].clone();
    //     // dbg!(frame.board.snakes.clone());
    //     let board0 = frame.clone();
    //     let t0 = Instant::now();
    //     let (mov, score) = search.iterative_deepen(&mut frame, 3);
    //     println!("{mov:?}, {score}");
    //     let t1 = Instant::now();
    //     // println!("{:?}", (t1 - t0).as_secs_f64() * 1000.0);
    //     // println!(
    //     //     "{:?} NPS",
    //     //     search.statistics.node_count as f64 / (t1 - t0).as_secs_f64()
    //     // );

    //     time_accum += t1 - t0;
    //     node_accum += search.statistics.node_count;
    //     hit_accum += search.statistics.tt_hits;
    //     accum += search.statistics.node_count as f64 / (t1 - t0).as_secs_f64();
    //     // println!("{score}");
    //     // assert_eq!(board0, frame);
    //     // println!("{score}, {pv_table:?}");
    //     // assert!((0.5 - score).abs() > 0.2);
    // }

    // println!("Average NPS: {}", accum / 50.0);
    // println!("Average nodes searched : {}", node_accum as f64 / 50.0);
    // println!("Average search time : {:?}", time_accum / 50);
    // println!("Average hits : {}", hit_accum as f64 / 50.0);
}
