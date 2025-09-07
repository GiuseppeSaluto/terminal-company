use crate::models::entities::GameState;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{Client, Collection, options::ClientOptions};
use std::env;
use std::io;

pub async fn get_game_state_collection() -> io::Result<Collection<GameState>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get DATABASE_URL: {}", e),
        )
    })?;

    let client_options = ClientOptions::parse(database_url).await.map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to parse connection string: {}", e),
        )
    })?;

    let client = Client::with_options(client_options).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create client: {}", e),
        )
    })?;

    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to connect to MongoDB: {}", e),
            )
        })?;

    let db = client.database("terminal_company");

    Ok(db.collection("game_state"))
}

pub async fn save_game_state(game_state: &GameState) -> io::Result<()> {
    let collection = get_game_state_collection().await?;

    let filter = doc! { "_id": "game_state" };

    collection
        .replace_one(filter, game_state)
        .await
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to save game state: {}", e),
            )
        })?;
    Ok(())
}

pub async fn load_game_state() -> io::Result<Option<GameState>> {
    let collection = get_game_state_collection().await?;
    let filter = doc! { "_id": "game_state" };
    let result = collection.find_one(filter).await.map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to load game state: {}", e),
        )
    })?;
    if let Some(state) = result {
        println!("Game state loaded.");
        Ok(Some(state))
    } else {
        Ok(None)
    }
}
