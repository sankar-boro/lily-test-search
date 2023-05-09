use std::env;
use tokio_postgres::NoTls;
use scylla::{Session, SessionBuilder};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};

pub async fn get_pg_connection() -> Pool {

    let pg_db_name = env::var("PG_DB_NAME").unwrap();
    let pg_db_uname = env::var("PG_DB_UNAME").unwrap();
    let pg_db_pwd = env::var("PG_DB_PWD").unwrap();

    let mut cfg = Config::new();
    cfg.dbname = Some(pg_db_name);
    cfg.user = Some(pg_db_uname);
    cfg.password = Some(pg_db_pwd);
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    pool
}

// pub async fn get_indexer_connection() -> Client {
//     let indexer_uri = env::var("INDEXER_URI").unwrap();
//     let indexer = Client::new(indexer_uri, Some("authUser"));
//     indexer
// }

pub async fn get_scylla_connection() -> Session {
    let db_uri = env::var("DB_URI").unwrap();
    let session = SessionBuilder::new().known_node(db_uri).build().await.unwrap();
    session
}
