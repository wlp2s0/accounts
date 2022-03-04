mod routes;
mod utils;

use actix_web::{App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use routes::ping;

use paperclip::actix::{
    api_v2_operation,
    delete,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    get,
    post,
    put,
    // use this instead of actix_web::web
    web::{self, Json},
    Apiv2Schema,
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};

async fn initialize_db() -> Result<mongodb::Database, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    // Get a handle to a database.
    let db = client.database("mydb");
    Ok(db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = initialize_db().await.expect("Database connection error");
    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .data(db.clone())
            .service(
                web::scope("/api")
                    .service(ping)
                    .service(signup)
                    .service(login),
            )
            .with_json_spec_v3_at("/docs")
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
