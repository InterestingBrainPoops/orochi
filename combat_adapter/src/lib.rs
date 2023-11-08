use std::io::Read;

use board::{incoming_board::Request, useful_board::Game};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use zstd::Decoder;

#[derive(Serialize, Deserialize)]
pub struct DB {
    pub positions: Vec<(Game, String)>,
}
struct ParticipantsEntry {
    game_id: String,
    snake_id: String,
    winner: u8,
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
        assert!(num_snakes > 1);

        // open a connection to the database and grab all the game, snake, and winner values for each row.
        let conn = Connection::open(path).unwrap();
        let mut stmt = conn
            .prepare("SELECT gid, sid, won FROM participants")
            .unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(ParticipantsEntry {
                    game_id: row.get(0)?,
                    snake_id: row.get(1)?,
                    winner: row.get(2)?,
                })
            })
            .unwrap();

        //Find all the game ids and their respective winners that match the given size requirement
        let mut thing = vec![];
        let mut num_snake_counter = 0;
        let mut current_game_id = String::new();
        let mut current_winner_id = String::new();
        for x in person_iter {
            let x = x.unwrap();
            // if the current row is the winner, update the winner.
            if x.winner == 1 {
                current_winner_id = x.snake_id;
            }
            // condition for the first entry, detected by current_game_id being empty.
            if current_game_id.is_empty() {
                current_game_id = x.game_id;
                num_snake_counter = 1;
                continue;
            }
            // if it matches the current game id, increase the number of snakes
            if current_game_id == x.game_id {
                num_snake_counter += 1;
            } else {
                // if it doesnt, then we have moved onto a new game
                if num_snake_counter == num_snakes {
                    // store the game id and the winner id.
                    thing.push((current_game_id, current_winner_id.clone()));
                }
                // update the current game id and reset the snake counter
                current_game_id = x.game_id;
                num_snake_counter = 1;
            }
        }
        // Using the found game ids from above, grab the games and convert them
        // grab all the records and game id's from the games table
        let mut stmt = conn.prepare("SELECT gid, record FROM games").unwrap();
        let games_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, Vec<u8>>(1).unwrap(),
            ))
        });
        // convert all the scraped games that match the participant count into the internal representation
        let mut out = vec![];
        for row in games_iter.unwrap() {
            let row = row.unwrap();
            if let Some(idx) = thing.iter().position(|(id, _)| id == &row.0) {
                let winner = thing[idx].1.clone();
                // unwrap the zstd compressed blob
                let mut x = Decoder::new(&row.1[..]).unwrap();
                let mut buf = String::new();
                x.read_to_string(&mut buf).unwrap();
                // unwrap the JSON into a game record
                let game: Record = serde_json::from_str(&buf).unwrap();
                for frame in &game.turns {
                    // convert it into a usable format.
                    out.push((frame.request.clone().into_usable(), winner.clone()));
                }
            }
        }

        DB { positions: out }
    }
}
