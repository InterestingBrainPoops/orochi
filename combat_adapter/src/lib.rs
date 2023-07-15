use std::io::Read;

use board::{incoming_board::Request, useful_board::Game};
use rusqlite::Connection;
use serde::Deserialize;
use zstd::Decoder;

pub struct DB {
    positions: Vec<Game>,
}
struct ParticipantsEntry {
    game_id: String,
}
#[derive(Deserialize, Debug, Clone)]
struct Record {
    turns: Vec<IRecord>,
}
#[derive(Deserialize, Debug, Clone)]
struct IRecord {
    request: Request,
}
impl DB {
    pub fn new(path: String, num_snakes: usize) -> Self {
        let conn = Connection::open(path).unwrap();
        let mut stmt = conn.prepare("SELECT gid FROM participants").unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(ParticipantsEntry {
                    game_id: row.get(0)?,
                })
            })
            .unwrap();
        let mut thing = vec![];
        let mut counter = 0;
        let mut current = String::new();
        for x in person_iter {
            let x = x.unwrap();
            if current.is_empty() {
                current = x.game_id;
                counter = 1;
                continue;
            }
            if current == x.game_id {
                counter += 1;
            } else {
                thing.push((current, counter));
                current = x.game_id;
                counter = 1;
            }
        }
        let game_ids = thing
            .iter()
            .filter(|&x| x.1 == num_snakes)
            .map(|x| x.0.clone())
            .collect::<Vec<String>>();
        // let mut games = vec![];
        let mut stmt = conn.prepare("SELECT gid, record FROM games").unwrap();
        let games_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, Vec<u8>>(1).unwrap(),
            ))
        });
        let mut out = vec![];
        for row in games_iter.unwrap() {
            let row = row.unwrap();
            if game_ids.contains(&row.0) {
                let mut x = Decoder::new(&row.1[..]).unwrap();
                let mut buf = String::new();
                x.read_to_string(&mut buf).unwrap();
                let game: Record = serde_json::from_str(&buf).unwrap();
                for frame in &game.turns {
                    out.push(frame.request.clone().into_usable());
                }
            }
        }

        DB { positions: out }
    }
}
