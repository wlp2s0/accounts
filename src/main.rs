mod plugins;
mod utils;

use mongodb::{options::ClientOptions, Client};
use plugins::auth::routes::AuthApi;
// use plugins::auth::AuthApi;
// use plugins::auth::routes::AuthApi;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=trace");
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(AuthApi, "Hello World", "1.0").server("http://localhost:3000");
    utils::db::get_db().await;
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
