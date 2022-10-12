use log::{info, error};
use std::sync::{Arc};

use std::env;

use std::net::SocketAddr;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};

#[macro_use]
pub mod utils;
pub mod logging;

pub mod error;
pub mod router;
pub mod proxy;
pub mod cache;
pub mod config;
pub mod session;
pub mod services;
pub mod security;
pub mod dash;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler.");
    
    println!();
    info!("Heimdall closing. Goodbye!");
    std::process::exit(0);
}

#[tokio::main]
async fn main() {
    // prometheus apm rust tracing
    // rust sentry
    // rethinkdb
    // tera templates
    // websocket
    // gRPC

    println!("{}", env::current_dir().unwrap().display());

    match logging::init_logging() {
        Ok(_) => {
            info!("/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
            info!("/\\/\\/\\/\\/ Heimdall \\/\\/\\/\\/");
            info!("/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
            println!()
        }
        Err(_) => {}
    }

    let services = Arc::new(services::new(dash::get_routes()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));

    let make_svc = make_service_fn(move |_| {
        let services = services.clone();

        async {
            Ok::<_, error::Error>(service_fn(move |req| {
                proxy::handle(req, services.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    info!("Heimdall listening on port: {}", 8888);

    if let Err(e) = graceful.await {
        error!("Server error: {}", e);
    }
}
