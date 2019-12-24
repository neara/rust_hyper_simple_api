use std::collections::HashMap;

use hyper::{Response, Body};

use super::types::Result;
use super::response::to_json;


pub async fn health_check() -> Result<Response<Body>>{
    info!("Checking server health");
    let mut payload:HashMap<&str, &str> = HashMap::new();
    payload.insert("status", "ok");

    to_json(&payload, None)
}

