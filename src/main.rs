use axum::{
    extract::Path, response::Html, routing::get, Json, Router
};
use serde::Deserialize;

use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(
    paths(
        handler_hello,
        handler_hello_name
    ),
    components(schemas(HelloParams))
)]
pub struct ApiDoc;

#[derive(Deserialize, utoipa::IntoParams, ToSchema)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_hello))
        .route("/:name", get(handler_hello_name))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
async fn handler_hello() -> Html<String> {
    Html(format!("<h1>Hello, World!</h1>"))
}

#[utoipa::path(
    get,
    path = "/:name",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
async fn handler_hello_name(Path(params): Path<HelloParams>) -> Html<String> {
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("<h1>Hello, {name}</h1>"))
}

