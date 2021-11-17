use std::convert::Infallible;
use std::net::SocketAddr;

use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use crate::globals::CONFIG;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    log::info!("Shutting down the HTTP server");
}

pub async fn init_http_server() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.http_port));
    let server = Server::bind(&addr).serve(make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(http_routes))
    }));
    log::info!(
        "HTTP server is listening on http://0.0.0.0:{}",
        &CONFIG.http_port
    );
    if let Err(e) = server.with_graceful_shutdown(shutdown_signal()).await {
        log::error!("HTTP server error: {}", e);
    };
    Ok(())
}

async fn http_routes(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/healthz") => *response.body_mut() = Body::from("ok"),
        _ => *response.status_mut() = StatusCode::NOT_FOUND,
    }
    Ok(response)
}
