use std::collections::HashMap;

use hyper::{Response, Body, StatusCode};

use super::types::Result;
use super::response::to_json;


pub async fn not_found() -> Result<Response<Body>>{
    info!("Requested url not found");
    let mut payload:HashMap<&str, &str> = HashMap::new();
    payload.insert("error", "not found");

    to_json(&payload, Some(StatusCode::NOT_FOUND))
}


pub async fn method_not_allowed() -> Result<Response<Body>>{
    info!("Requested method not allowed");
    let mut payload:HashMap<&str, &str> = HashMap::new();
    payload.insert("error", "method not allowed");

    to_json(&payload, Some(StatusCode::METHOD_NOT_ALLOWED))
}


