use combat_adapter::DB;

fn main() {
    let db = DB::new(
        "../../Downloads/combat-reptile_dump_2023-06-27.sqlite".to_string(),
        2,
    );
}
