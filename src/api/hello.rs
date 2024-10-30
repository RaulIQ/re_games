use axum::{response::Html, extract::Path};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa::OpenApi;


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
pub struct HelloParams {
    pub name: Option<String>,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
pub async fn handler_hello() -> Html<String> {
    Html(format!("<h1>Hello, World!</h1>"))
}

#[utoipa::path(
    get,
    path = "/:name",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
pub async fn handler_hello_name(Path(params): Path<HelloParams>) -> Html<String> {
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("<h1>Hello, {name}</h1>"))
}
