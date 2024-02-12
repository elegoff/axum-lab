#![allow(unused)] //just for beginning

use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello().fallback_service(routes_static()));

    // Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LITENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
// Routes hello
//
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// e.g /hello?name=John
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}

//e.g /hello/Jane
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}</strong>"))
}
