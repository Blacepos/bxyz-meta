use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    init::initialize,
    paths::CONTENT_DIR,
    routes::{route_audio, route_index},
};

mod cli;
mod init;
mod paths;
mod routes;

const MODULE_NAME: &str = "meta";

#[tokio::main]
async fn main() {
    let (args, _logger_handle) = initialize();
    log::debug!("Completed initialization");

    let addr = SocketAddr::new(args.web_addr, args.http_port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // if defined, register with the slot server
    if let Some(slot_port) = args.slot_port {
        // TODO: enforce localhost in args while there's no support for other
        // interfaces?
        let module_name = slot_client::protocol::ValidName::from_str(MODULE_NAME)
            .expect("The constant module name is valid");

        slot_client::client_impl::run_client(
            slot_port,
            module_name,
            listener.local_addr().expect("HTTP socket is bound").port(),
        );
    }

    // create tera template engine instance to pass into routes
    let tera_eng = match tera::Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Failed to parse templates: \"{e}\"");
            std::process::exit(1);
        }
    };

    // set up webserver
    let routes = Router::new()
        .route("/meta/index", get(route_index))
        .route("/meta/audio", get(route_audio))
        .nest_service("/meta/content", ServeDir::new(CONTENT_DIR))
        .layer(TraceLayer::new_for_http())
        .with_state(tera_eng);
    axum::serve(listener, routes).await.unwrap();
}
