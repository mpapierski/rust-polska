extern crate hyper;
#[macro_use]
extern crate log;

use std::env;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};

static TEXT: &str = "Hello, Rust Polska!";

fn main() -> Result<(), Box<std::error::Error>> {
    pretty_env_logger::init();

    // Heroku passes $PORT
    let port = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        // For local development port 3000 is used if no $PORT is found.
        .unwrap_or(3000);

    let addr = ([0, 0, 0, 0], port).into();

    let new_svc = || service_fn_ok(|_req| Response::new(Body::from(TEXT)));

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e));

    hyper::rt::run(server);

    Ok(())
}
