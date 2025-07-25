use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, Router};
use init::initialize;

pub mod cli;
pub mod init;

const MODULE_NAME: &str = "meta";

#[tokio::main]
async fn main() {
    let (args, _logger_handle) = initialize();
    log::debug!("Completed initialization");

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
            args.http_port,
        );
    }

    // set up webserver
    let routes = Router::new().route("/meta/index", get(test_route));

    let addr = SocketAddr::new(args.web_addr, args.http_port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}

async fn test_route() -> &'static str {
    "Hello from the meta module!"
}
