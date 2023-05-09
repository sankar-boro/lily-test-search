mod route;
mod error;
mod auth;
mod db;
mod builder;
mod utils;
mod v2;

use log::info;
use std::fs::create_dir;
use lily_types::FlatNamedDocument;
use v2::IndexCatalog;
use std::env;
use std::path::Path;
use std::sync::Arc;
use time::Duration;
use actix_cors::Cors;
use actix_web::{App as ActixApp, HttpServer, web, cookie};
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};
use v2::{settings, Settings};
use lily_types::Catalog;

pub type Result<T> = std::result::Result<T, lily_types::Error>;
pub type AddDocument = lily_types::AddDocument<serde_json::Value>;
pub type SearchResults = lily_types::SearchResults<FlatNamedDocument>;
pub type SharedCatalog = Arc<IndexCatalog>;

pub use builder::Connections;

async fn start_server(app: Connections) -> std::io::Result<()> {
    let lp_host = env::var("LP_HOST").unwrap();
    let lp_port = env::var("LP_PORT").unwrap();
    let lp_port: u16 = lp_port.parse().unwrap();
    let pkey = env::var("PRIVATE_KEY").unwrap();
    let redis_uri = env::var("REDIS_URI").unwrap();

    let private_key = cookie::Key::from(pkey.as_bytes());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&redis_uri),
                    private_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(Duration::days(5))
                )
                .build()
            )
            .app_data(web::Data::new(app.clone()))
            .configure(route::routes)
    })
    .bind((lp_host, lp_port))?
    .run()
    .await?;

    Ok(())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let pool = db::get_pg_connection().await;
    let session = db::get_scylla_connection().await;
    
    // lily-search
    let settings = settings();
    if !Path::new(&settings.path).exists() {
        info!("Base data path {} does not exist, creating it...", settings.path);
        create_dir(settings.path.clone()).expect("Unable to create data directory");
    }
    let index_catalog = setup_catalog(&settings).await.unwrap();
    let app = Connections::new(pool, session, index_catalog);
    start_server(app).await.unwrap();
    Ok(())
}

async fn setup_catalog(settings: &Settings) -> Result<IndexCatalog> {
    let mut index_catalog = match IndexCatalog::new(settings.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error creating IndexCatalog from path {} - {:?}", settings.path, e);
            std::process::exit(1);
        }
    };
    index_catalog.refresh_catalog().await?;
    info!("{} Indexes loaded...", index_catalog.get_collection().len());
    Ok(index_catalog)
}