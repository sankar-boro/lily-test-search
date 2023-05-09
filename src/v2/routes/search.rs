use crate::error::Error;
use crate::{Connections};

use actix_web::{web, HttpResponse};
use lily_types::{Catalog, IndexHandle, Search, FuzzyQuery};

pub async fn search_docs(app: web::Data<Connections>, path: web::Path<(String, String, String)>) -> Result<HttpResponse, Error> {
    let index_name = &path.0;
    let field_name = &path.1;
    let query_name = &path.2;

    let search: Search = Search {
        query: Some(FuzzyQuery::builder().for_field(field_name).with_distance(1).with_value(query_name).build()),
        facets: None,
        limit: 10,
        sort_by: None,
    };
    if app.catalog.exists(index_name) {
        let index = app.catalog.get_index(index_name).unwrap(); // If this unwrap fails, this is a bug.
        let res = index.search_index(search).await.unwrap();
        return Ok(HttpResponse::Ok().json(res));
    }    

    Ok(HttpResponse::Ok().body(format!("Document not found.")))
}