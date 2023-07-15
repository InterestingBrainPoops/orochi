use board::useful_board::Game;
use rusqlite::Connection;

pub struct DB {
    positions: Vec<Game>,
}
struct ParticipantsEntry {
    game_id: String,
}
impl DB {
    pub fn new(path: String, num_snakes: usize) -> Self {
        let conn = Connection::open(path).unwrap();
        let mut stmt = conn
            .prepare("SELECT gid compressed_frames_json FROM snake_games")
            .unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(ParticipantsEntry {
                    game_id: row.get(1)?,
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

        todo!()
    }
}
