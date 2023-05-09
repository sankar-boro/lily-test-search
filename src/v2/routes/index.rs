use crate::error::Error;
use crate::{Connections};

use tantivy::schema::Schema;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use lily_types::{Catalog};

#[derive(Serialize, Deserialize)]
pub struct IndexSchema {
    index_name: String,
    schema: String,
}

pub async fn create_index(app: web::Data<Connections>, payload: web::Json<IndexSchema>) -> Result<HttpResponse, Error> {
    if app.catalog.exists(&payload.index_name) {
        return Err(Error::from("INDEX EXISTS").into());
    }
    let sc: Schema = serde_json::from_slice::<Schema>(&payload.schema.as_bytes()).unwrap();
    app.catalog.add_index(&payload.index_name, sc).await.unwrap();
    Ok(HttpResponse::Ok().body(format!("Created index.")))
}


pub async fn delete_index(app: web::Data<Connections>, payload: web::Path<String>) -> Result<HttpResponse, Error> {
    if !app.catalog.exists(&payload) {
        return Err(Error::from("INDEX DOES NOT EXISTS").into());
    }
    app.catalog.delete_index(&payload).await.unwrap();
    Ok(HttpResponse::Ok().body(format!("Deleted index.")))
}
