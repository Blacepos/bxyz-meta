use std::{net::SocketAddr, path::Path, str::FromStr};

use axum::{response::Html, routing::get, Router};
use init::initialize;
use tokio::fs;
use tower_http::services::ServeDir;

pub mod cli;
pub mod init;

const MODULE_NAME: &str = "meta";
const CONTENT_DIR: &str = "content";

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
        let module_name =
            slot_client::protocol::ValidName::from_str(MODULE_NAME)
                .expect("The constant module name is valid");

        slot_client::client_impl::run_client(
            slot_port,
            module_name,
            listener.local_addr().expect("HTTP socket is bound").port(),
        );
    }

    // set up webserver
    let routes = Router::new()
        .route(
            "/meta/index",
            get(async || {
                Html(
                    fs::read(Path::new(CONTENT_DIR).join("pages/index.html"))
                        .await
                        .expect("index.html exists"),
                )
            }),
        )
        .nest_service("/meta/content", ServeDir::new(CONTENT_DIR));
    axum::serve(listener, routes).await.unwrap();
}
