use crate::models::entities::GameState;
use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::{Client, Collection, options::ClientOptions};
use std::env;
use std::io;
use std::sync::Arc;

pub async fn init_db() -> io::Result<Arc<Client>> {
    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client_options = ClientOptions::parse(&uri).await.map_err(|e| {
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

    ensure_collection_exists(&client, "terminal_company", "game_state").await?;

    Ok(Arc::new(client))
}

async fn ensure_collection_exists(
    client: &Client,
    db_name: &str,
    coll_name: &str,
) -> io::Result<()> {
    let db = client.database(db_name);
    let names: Vec<String> = db.list_collection_names().await.map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to list collections: {}", e),
        )
    })?;

    if !names.iter().any(|n| n == coll_name) {
        db.create_collection(coll_name).await.map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create collection: {}", e),
            )
        })?;
    }
    Ok(())
}

pub async fn save_game_state(client: &Client, game_state: &GameState) -> io::Result<()> {
    let collection = client.database("terminal_company").collection::<GameState>("game_state");
    
    // Per un'applicazione a giocatore singolo, possiamo usare un ID fisso per sovrascrivere.
    let filter = doc! { "_id": "game_state" };
    let opts = ReplaceOptions::builder().upsert(true).build();
    
    collection
        .replace_one(filter, game_state)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to save game state: {}", e)))?;
        
    Ok(())
}

pub async fn load_game_state(client: &Client) -> io::Result<Option<GameState>> {
    let collection = client.database("terminal_company").collection::<GameState>("game_state");
    let filter = doc! { "_id": "game_state" };
    
    let result = collection
        .find_one(filter)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to load game state: {}", e)))?;
        
    Ok(result)
}
