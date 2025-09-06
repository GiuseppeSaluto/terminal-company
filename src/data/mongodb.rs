use crate::entities::GameState;
use mongodb::bson::doc;
use mongodb::{Client, Database, options::ClientOptions};


// Function to get the MongoDB collection for GameState
pub async fn get_game_state_collection() -> io::Result<Collection<GameState>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to parse connection string: {}", e)))?;
        
    let client = Client::with_options(client_options)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to create client: {}", e)))?;
        
    // Ping Check
    client.database("admin").run_command(mongodb::bson::doc! {"ping": 1}, None).await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to connect to MongoDB: {}", e)))?;

    let db = client.database("terminal_company");
    
    Ok(db.collection("game_state"))
}
// Function to save the game state from MongoDB
pub async fn save_game_state(game_state: &GameState) {
    let collection = get_game_state_collection().await;
    // Clear existing state and insert the new one
    collection.delete_many(doc! {}, None).await.expect("Failed to delete.");
    collection.insert_one(game_state, None).await.expect("Failed to insert.");
    println!("Game state saved successfully.");
}

// Function to load the game state from MongoDB
pub async fn load_game_state() -> Option<GameState> {
    let collection = get_game_state_collection().await;
    let result = collection.find_one(None, None).await.expect("Failed to load.");
    if let Some(state) = result {
        println!("Game state loaded.");
        Some(state)
    } else {
        None
    }
}
