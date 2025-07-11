//! Defines the command line interface
//!
//! Adding attributes to this structure will add CLI options

use clap::Parser;
use std::net::SocketAddr;

const DEFAULT_LOG_LEVEL: &str = "INFO";
const DEFAULT_BIND: &str = "127.0.0.1:8000";

#[derive(Parser, Debug)]
#[command(version, about = "blacepos.xyz webserver \"meta\" module")]
pub struct Args {
    /// Log level (ERROR, WARN, INFO, DEBUG, TRACE)
    #[arg(short='l', long="log", default_value=DEFAULT_LOG_LEVEL)]
    pub log_level: log::LevelFilter,

    /// The socket address bind the webserver to e.g., "127.0.0.1:8000"
    #[arg(short='w', long="web-bind", default_value=DEFAULT_BIND)]
    pub bind_addr: SocketAddr,

    /// The port of the slot server on localhost e.g., "7568"
    #[arg(short = 's', long = "slot-addr")]
    pub slot_port: Option<u16>,
}
