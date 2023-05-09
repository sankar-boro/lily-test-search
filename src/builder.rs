use std::sync::Arc;
use scylla::Session;
use scylla::batch::Batch;
use scylla::query::Query;
use deadpool_postgres::Pool;
use crate::v2::IndexCatalog;
use scylla::{QueryResult, BatchResult};
use scylla::transport::errors::QueryError;
use scylla::prepared_statement::PreparedStatement;
use scylla::frame::value::{ValueList, BatchValues};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Connections {
    pub pgpool: Pool,
    pub session: Arc<Session>,
    pub catalog: Arc<IndexCatalog>,
}

impl Connections {
    pub fn new(pgpool: Pool, session: Session, catalog: IndexCatalog) -> Self {
        Self {
            pgpool,
            session: Arc::new(session),
            catalog: Arc::new(catalog)
        }
    }

    pub async fn query(&self, query: impl Into<Query>, values: impl ValueList) -> Result<QueryResult, QueryError> {
        self.session.query(query, values).await
    }

    pub async fn query_paged(&self, query: impl Into<Query>, values: impl ValueList, page: Vec<u8>) -> Result<QueryResult, QueryError>{
        let pagedata = Some(scylla::Bytes::from(page));
        self.session.query_paged(query, values, pagedata).await
    }

    pub async fn batch(&self, query: &Batch, values: impl BatchValues) -> Result<BatchResult, QueryError> {
        self.session.batch(query, values).await
    }

    pub async fn execute(&self, query: &PreparedStatement, values: impl ValueList) -> Result<QueryResult, QueryError> {
        self.session.execute(&query, values).await
    }
}