use crate::error::Error;
use crate::{AddDocument, Connections};

use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use lily_types::{Catalog, IndexHandle, IndexOptions};

#[derive(Serialize, Deserialize)]
pub struct AddDocumentPayload {
    index_name: String,
    data: String,
}

pub async fn add_document(app: web::Data<Connections>, payload: web::Json<AddDocumentPayload>) -> Result<HttpResponse, Error> {
    let local_index = app.catalog.get_index(&payload.index_name)?;
    if !app.catalog.exists(&payload.index_name) {
        return Err(Error::from("NOT EXISTS: INDEX").into());
    }
    let user = serde_json::from_str(&payload.data)?;

    let doc_ = AddDocument {
        options: Some(IndexOptions {
            commit: true,
        }),
        document: user,
    };
    local_index.add_document(doc_).await?;
    Ok(HttpResponse::Ok().body(format!("Document added.")))
}
