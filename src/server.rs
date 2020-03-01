use std::convert::Infallible;
use std::error::Error;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use hyper::{Body, Request, Server, Method, Response};
use hyper::service::{make_service_fn, service_fn};

use crate::views;


const PORT:u16 = 7878;

async fn graceful_shutdown() {
    tokio::signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler")
}

async fn router(req: Request<Body>) -> views::Result<Response<Body>> {
    debug!("Router got incoming request");

    match req.uri().path() {
        "/health/check" => {
            match req.method() {
                &Method::GET => views::health_check().await,
                _ => views::method_not_allowed().await
            }

        },
        _ => views::not_found().await,
    }
}

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn Error + Send + Sync >>{
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT);

    let make_service = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(router)) }
    });

    let server = Server::bind(&socket).serve(make_service);
    info!("Listening on port: {}", PORT);

    let graceful_shutdown = server.with_graceful_shutdown(graceful_shutdown());

    graceful_shutdown.await?;

    Ok(())
}
