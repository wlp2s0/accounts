mod routes;

use mongodb::{options::ClientOptions, Client};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use routes::auth_routes::AuthApi;

async fn initialize_db() -> Result<mongodb::Database, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    // Get a handle to a database.
    let db = client.database("mydb");
    Ok(db)
}



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
let api_service = OpenApiService::new(AuthApi, "Hello World", "1.0").server("http://localhost:3000");
let ui = api_service.swagger_ui();
let app = Route::new().nest("/", api_service).nest("/docs", ui);

Server::new(TcpListener::bind("127.0.0.1:3000"))
    .run(app)
    .await
}
