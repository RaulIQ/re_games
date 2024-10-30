use api::hello::ApiDoc;
use axum::{
    extract::Path, response::Html, routing::{get, post}, Json, Router
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use serde::Deserialize;

use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod config;
mod db;
mod api;
mod docs;

#[tokio::main]
async fn main() {
    config::load_env();

    let pool = db::create_pool().expect("Failed to create pool.");
    
    let app = Router::new()
        .route("/", get(api::hello::handler_hello))
        .route("/:name", get(api::hello::handler_hello_name))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


