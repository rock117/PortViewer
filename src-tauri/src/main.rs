// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod network;
mod process;
mod filter;

use models::ConnectionInfo;
use network::get_all_connections;
use filter::filter_connections;
use serde::{Deserialize, Serialize};

// Serializable version of ConnectionInfo for Tauri commands
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfoSerde {
    protocol: String,
    local_address: String,
    local_port: u16,
    remote_address: String,
    remote_port: u16,
    state: String,
    pid: u32,
    process_name: String,
}

impl From<ConnectionInfo> for ConnectionInfoSerde {
    fn from(conn: ConnectionInfo) -> Self {
        ConnectionInfoSerde {
            protocol: conn.protocol.to_string(),
            local_address: conn.local_address,
            local_port: conn.local_port,
            remote_address: conn.remote_address,
            remote_port: conn.remote_port,
            state: conn.state.to_string(),
            pid: conn.pid,
            process_name: conn.process_name,
        }
    }
}

// Tauri command to get all connections
#[tauri::command]
fn get_connections() -> Vec<ConnectionInfoSerde> {
    let connections = get_all_connections();
    println!("Backend: Retrieved {} total connections", connections.len());
    
    let tcp_count = connections.iter().filter(|c| matches!(c.protocol, crate::models::Protocol::TCP)).count();
    let udp_count = connections.iter().filter(|c| matches!(c.protocol, crate::models::Protocol::UDP)).count();
    println!("Backend: TCP connections: {}, UDP connections: {}", tcp_count, udp_count);
    
    let result: Vec<ConnectionInfoSerde> = connections.into_iter().map(ConnectionInfoSerde::from).collect();
    println!("Backend: Returning {} serialized connections", result.len());
    result
}

// Tauri command to get filtered connections
#[tauri::command]
fn get_filtered_connections(protocol: String, port: Option<u16>) -> Vec<ConnectionInfoSerde> {
    let all_connections = get_all_connections();
    let filtered = filter_connections(&all_connections, &protocol, port);
    filtered.into_iter().map(ConnectionInfoSerde::from).collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_connections, get_filtered_connections])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
