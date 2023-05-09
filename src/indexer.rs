// use crate::error::Error;
// use crate::auth::AuthSession;
// use crate::Connections;

// use actix_session::Session;
// use actix_web::{HttpResponse, web};
// use serde::{ Serialize, Deserialize };

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CreateIndexRequest {
//     index: String
// }

// #[derive(Serialize, Deserialize)]
// pub struct DeleteIndexRequest {
//     index: String
// }

// #[derive(Serialize, Deserialize)]
// pub struct AddCategoryRequest {
//     index: String,
//     name: String
// }

// #[derive(Serialize, Deserialize)]
// pub struct AddCategoryPg {
//     id: i32,
//     name: String
// }

// #[derive(Serialize, Deserialize)]
// pub struct SearchableAttributes {
//     index: String,
//     attrs: Vec<String>
// }

// pub async fn create_index(app: web::Data<Connections>, request: web::Json<CreateIndexRequest>, session: Session) -> Result<HttpResponse, Error> {
//     let auth = session.user_info()?;
//     if auth.userId == 1 {
//         app.indexer.create_index(&request.index, None).await?;
//         return Ok(HttpResponse::Ok().body(format!("created index: {}", &request.index)));
//     }
//     Ok(HttpResponse::Ok().body(format!("Could not create index: {}", &request.index)))
// }

// pub async fn set_searchable_attributes(app: web::Data<Connections>, request: web::Json<SearchableAttributes>, session: Session) -> Result<HttpResponse, Error> {
//     let auth = session.user_info()?;
//     if auth.userId == 1 {
//         let _ = app.indexer.index(&request.index).set_searchable_attributes(&request.attrs).await?;
//         return Ok(HttpResponse::Ok().body(format!("searchable atrributes set to")));
//     }
//     Ok(HttpResponse::Ok().body(format!("Could not create index")))
// }

// pub async fn delete_index(app: web::Data<Connections>, request: web::Json<DeleteIndexRequest>, session: Session) -> Result<HttpResponse, Error> {
//     let auth = session.user_info()?;
//     if auth.userId == 1 {
//         app.indexer.delete_index(&request.index).await?;
//         return Ok(HttpResponse::Ok().body(format!("deleted index: {}", &request.index)));
//     }
//     Ok(HttpResponse::Ok().body(format!("Could not delete index: {}", &request.index)))
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct GetCategory {
//     id: String,
//     name: String,
// }
// pub async fn get_categories(app: web::Data<Connections>, query_string: web::Path<(String,String)>) -> Result<HttpResponse, Error> {
//     let index = app.indexer.index(&query_string.0);

//     let results: SearchResults<GetCategory> = index.search()
//     .with_query(&query_string.1)
//     .with_limit(5)
//     .execute()
//     .await?;

//     let doc_res: Vec<GetCategory> = results.hits.iter().map(|d| d.result.clone()).collect();

//     Ok(HttpResponse::Ok().json(doc_res))
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct GetUsers {
//     user_id: i32,
//     fname: String,
//     lname: String
// }

// pub async fn get_users(app: web::Data<Connections>, query_string: web::Path<(String,String)>) -> Result<HttpResponse, Error> {
//     let index = app.indexer.index(&query_string.0);

//     let results: SearchResults<GetUsers> = index.search()
//     .with_query(&query_string.1)
//     .with_limit(5)
//     .execute()
//     .await?;

//     let doc_res: Vec<GetUsers> = results.hits.iter().map(|d| d.result.clone()).collect();

//     Ok(HttpResponse::Ok().json(doc_res))
// }

// pub async fn get_stats(app: web::Data<Connections>) -> Result<HttpResponse, Error> {
//     let stats: ClientStats = app.indexer
//     .get_stats()
//     .await
//     .unwrap();

//     let x: Vec<&String> = stats.indexes.keys().map(|f| {f}).collect();
//     Ok(HttpResponse::Ok().json(x))
// }

// #[derive(Serialize, Deserialize, Clone)]
// #[allow(non_snake_case)]
// struct GetDocs {
//     docId: String,
//     title: String,
//     body: String,
//     createdAt: String
// }

// pub async fn get_docs(app: web::Data<Connections>, query_string: web::Path<(String,String)>) -> Result<HttpResponse, Error> {
//     let index = app.indexer.index(&query_string.0);

//     let results: SearchResults<GetDocs> = index.search()
//     .with_query(&query_string.1)
//     .with_limit(5)
//     .execute()
//     .await?;

//     let doc_res: Vec<GetDocs> = results.hits.iter().map(|d| d.result.clone()).collect();

//     Ok(HttpResponse::Ok().json(doc_res))
// }