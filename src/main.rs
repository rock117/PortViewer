use clap::{Arg, Command};

mod models;
mod network;
mod filter;
mod display;
mod process;

use models::ConnectionInfo;
use network::get_all_connections;
use filter::filter_connections;
use display::display_connections;

fn main() {
    let matches = Command::new("windows-port-viewer")
        .about("Windows Port Usage Viewer")
        .arg(
            Arg::new("protocol")
                .short('p')
                .long("protocol")
                .value_name("PROTOCOL")
                .help("Specify protocol type (tcp, udp, all)")
                .default_value("all")
        )
        .arg(
            Arg::new("port")
                .short('P')
                .long("port")
                .value_name("PORT")
                .help("Filter by specific port")
        )
        .get_matches();

    let protocol = matches.get_one::<String>("protocol").unwrap();
    let filter_port = matches.get_one::<String>("port").map(|s| s.parse::<u16>().ok()).flatten();

    // Get all connections
    let all_connections = get_all_connections();
    
    // Filter connections based on protocol and port
    let filtered_connections = filter_connections(&all_connections, protocol, filter_port);
    
    // Display filtered connections
    display_connections(&filtered_connections);
}
