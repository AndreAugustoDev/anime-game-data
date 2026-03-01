use std::io::{Write, stdout};

use anime_game_data::AnimeGameData;

#[tokio::main]
async fn main() {
    let mut data = AnimeGameData::new().unwrap();

    print!("Updating game data... ");
    stdout().flush().unwrap();

    if let Err(e) = data.update().await {
        println!("error: {e}");
    } else {
        println!("done");
    }
}
