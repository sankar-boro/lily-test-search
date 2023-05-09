#![allow(non_snake_case)]

use crate::error::Error;
use crate::Connections;
use crate::{AddDocument};

use scylla::{macros::FromRow};
use actix_web::{HttpResponse, web};
use serde::{ Serialize, Deserialize };
use lily_types::{Catalog, IndexHandle, IndexOptions};

static GET_ALL_TABLE_USERS: &str = "SELECT userId, fname, lname from users";

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[allow(non_snake_case)]
struct GetUser {
    userId: i32,
    fname: String,
    lname: String,
}

pub async fn add_users(app: web::Data<Connections>) -> Result<HttpResponse, Error> {
    let client = app.pgpool.get().await?;
    let stmt = client.prepare_cached(GET_ALL_TABLE_USERS).await?;
    let rows = client.query(&stmt, &[]).await?;

    let index_name = "users";
    let local_index = app.catalog.get_index(&index_name).unwrap();
    if !app.catalog.exists(&index_name) {
        return Err(Error::from("NOT EXISTS: INDEX").into());
    }

    // let mut users = Vec::new();
    for (x, _) in rows.iter().enumerate() {
        let userId: i32 = rows[x].get(0);
        let fname: String = rows[x].get(1);
        let lname: String = rows[x].get(2);
        let user = GetUser {
            userId,
            fname,
            lname
        };
        let user = serde_json::json!(user);
        
            let doc_ = AddDocument {
                options: Some(IndexOptions {
                    commit: true,
                }),
                document: user,
            };
            local_index.add_document(doc_).await.unwrap();
    }

    Ok(HttpResponse::Ok().body("Indexed users"))
}
