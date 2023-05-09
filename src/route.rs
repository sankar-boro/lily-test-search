use actix_web::web;
use crate::v2;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/v2")
        .route("/create_index", web::post().to(v2::create_index))
        .route("/add_document", web::post().to(v2::add_document))
        .route("/add_users", web::post().to(v2::add_users))
        .route("/search_docs/{index}/{field}/{query}", web::get().to(v2::search_docs))
        .route("/delete_index/{name}", web::post().to(v2::delete_index))
    );
}