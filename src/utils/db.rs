use mongodb::options::ClientOptions;
use mongodb::Client;
use tokio::sync::OnceCell;

static DATABASE: OnceCell<mongodb::Database> = OnceCell::const_new();

async fn initialize_db() -> Result<mongodb::Database, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    // Get a handle to a database.
    let db = client.database("mydb");
    Ok(db)
}

// async fn get_global_integer() -> &'static u32 {
//     ONCE.get_or_init(|| async { 1 + 1 }).await
// }

pub async fn get_db() -> &'static mongodb::Database {
    DATABASE
        .get_or_init(|| async { initialize_db().await.expect("Could not initialize DB") })
        .await
}
