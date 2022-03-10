mod plugins;
mod utils;

use plugins::auth::routes::AuthApi;
// use plugins::auth::AuthApi;
// use plugins::auth::routes::AuthApi;
use poem::{
    listener::TcpListener,
    session::{CookieConfig, RedisStorage, ServerSession},
    EndpointExt, Route, Server,
};
use poem_openapi::OpenApiService;
use redis::{aio::ConnectionManager, Client as RedisClient};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=trace");
    }
    tracing_subscriber::fmt::init();

    let client = RedisClient::open("redis://127.0.0.1/").unwrap();

    let api_service =
        OpenApiService::new(AuthApi, "Hello World", "1.0").server("http://localhost:6001");
    utils::db::get_db().await;
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .with(ServerSession::new(
            CookieConfig::default().secure(false),
            RedisStorage::new(ConnectionManager::new(client).await.unwrap()),
        ));

    Server::new(TcpListener::bind("127.0.0.1:6001"))
        .run(app)
        .await
}
