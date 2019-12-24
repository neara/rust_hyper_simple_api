use std::convert::Infallible;
use std::error::Error;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use hyper::{Body, Request, Server, Method, StatusCode, Response};
use hyper::service::{make_service_fn, service_fn};

use crate::views;


const PORT:u16 = 7878;

fn http_not_found_error() -> views::Result<Response<Body>>{
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

fn http_method_not_allowed() -> views::Result<Response<Body>> {
    let mut resp = Response::default();
    *resp.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
    Ok(resp)
}


async fn router(req: Request<Body>) -> views::Result<Response<Body>> {
    debug!("Router got incoming request");

    match req.uri().path() {
        "/health/check" => {
            match req.method() {
                &Method::GET => views::health_check().await,
                _ => http_method_not_allowed()
            }

        },
        _ => http_not_found_error(),
    }
}

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn Error + Send + Sync >>{
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), PORT);

    let make_service = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(router)) }
    });

    let server = Server::bind(&socket).serve(make_service);
    info!("Listening on port: {}", PORT);

    server.await?;

    Ok(())
}
