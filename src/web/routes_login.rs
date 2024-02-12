use crate::{web, Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

// only one body extractor per API, needs to be last param
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    // TOD : database logic for athentocation
    if payload.username != "toto" || payload.password != "123456" {
        return Err(Error::LoginFail);
    }
    // Set cookies

    // Create success body
    let body = Json(json!({

    "result" : {
    "success" : true
    }

    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
