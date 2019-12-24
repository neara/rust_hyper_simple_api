use std::collections::HashMap;

use hyper::{StatusCode, header, Response, Body};

use super::types::Result;


pub fn to_json(payload: &HashMap<&str, &str>, status_code: Option<StatusCode>) -> Result<Response<Body>> {
    debug!("building response");
    let json_payload = serde_json::to_string(&payload)?;
    let status_code = status_code.unwrap_or(StatusCode::OK);

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status_code)
        .body(Body::from(json_payload))?;

    Ok(response)
}
