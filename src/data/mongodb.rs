use crate::models::types::{CollectConfig, GameState, Bestiary};
use log::{error, info};
use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn init_db() -> Result<Client, Box<dyn std::error::Error>> {
    let uri = env::var("MONGODB_URI")?;
    let options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(options)?;

    ensure_collection_exists(&client, "terminal_company", "game_state").await?;
    //ensure_collect_config_exists(&client).await?;
    load_bestiary(&client).await?;

    info!("✅Connection to MongoDB established on {}", uri);
    Ok(client)
}

async fn ensure_collection_exists(
    client: &Client,
    db_name: &str,
    coll_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database(db_name);
    let collections = db.list_collection_names().await?;

    if !collections.contains(&coll_name.to_string()) {
        db.create_collection(coll_name).await?;
        info!(
            "📦 Collection '{}' created in database '{}'",
            coll_name, db_name
        );

        let coll = db.collection::<GameState>(coll_name);
        coll.insert_one(GameState::default()).await?;
        info!("Inserted initial game_state document.");
    }

    Ok(())
}

pub async fn save_game_state(
    client: &Client,
    game_state: &GameState,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database("terminal_company");
    let collection = db.collection::<GameState>("game_state");

    let filter = doc! { "_id": "game_state" };
    let options = ReplaceOptions::builder().upsert(true).build();

    match collection
        .replace_one(filter, game_state)
        .with_options(options)
        .await
    {
        Ok(result) => {
            if result.matched_count == 0 && result.upserted_id.is_some() {
                info!("New game_state inserted (upsert).");
            } else {
                info!("game_state updated.");
            }
            Ok(())
        }
        Err(e) => {
            error!("Error saving game_state: {:?}", e);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error saving game_state",
            )))
        }
    }
}

pub async fn load_game_state(
    client: &Client,
) -> Result<Option<GameState>, Box<dyn std::error::Error>> {
    let db = client.database("terminal_company");
    let collection = db.collection::<GameState>("game_state");

    match collection.find_one(doc! { "_id": "game_state" }).await {
        Ok(game_state) => Ok(game_state),
        Err(e) => {
            error!("❌Error loading game_state: {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn delete_game_state(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database("terminal_company");
    let collection = db.collection::<GameState>("game_state");

    match collection.delete_one(doc! { "_id": "game_state" }).await {
        Ok(_) => {
            info!("🗑️ game_state deleted.");
            Ok(())
        }
        Err(e) => {
            error!("❌ Error deleting game_state: {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn load_collect_config(
    client: &Client,
) -> Result<CollectConfig, Box<dyn std::error::Error>> {
    let db = client.database("terminal_company");
    let coll = db.collection::<CollectConfig>("game_config");
    let filter = doc! { "_id": "collect_config" };

    if let Some(config) = coll.find_one(filter.clone()).await? {
        Ok(config)
    } else {
        let mut default_config = CollectConfig::default();
        
        default_config.id = Some("collect_config".to_string());
        coll.insert_one(default_config.clone()).await?;
        Ok(default_config)
    }
}

pub async fn load_bestiary(
    client: &Client,
) -> Result<Bestiary, Box<dyn std::error::Error>> {
    let db = client.database("terminal_company");
    let coll = db.collection::<Bestiary>("bestiary");
    let filter = doc! { "_id": "bestiary" };

    if let Some(bestiary) = coll.find_one(filter.clone()).await? {
        Ok(bestiary)
    } else {
        let mut default_bestiary = Bestiary::default();
        default_bestiary.monsters = crate::models::lists::BESTIARY.to_vec();
        default_bestiary.id = Some("bestiary".to_string());
        coll.insert_one(default_bestiary.clone()).await?;
        Ok(default_bestiary)
    }
}
