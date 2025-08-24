// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod filter;
mod platform;

use models::ConnectionInfo;
use platform::create_network_provider;
use filter::filter_connections;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Serializable version of ConnectionInfo for Tauri commands
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfoSerde {
    id: String,
    protocol: String,
    local_address: String,
    local_port: u16,
    remote_address: String,
    remote_port: u16,
    state: String,
    pid: u32,
    process_name: String,
}

// Generate unique ID for connections using UUID
fn generate_connection_id() -> String {
    Uuid::new_v4().to_string()
}

impl From<ConnectionInfo> for ConnectionInfoSerde {
    fn from(conn: ConnectionInfo) -> Self {
        ConnectionInfoSerde {
            id: generate_connection_id(),
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

// Tauri command for frontend logging
#[tauri::command]
fn log_message(level: String, message: String, data: Option<String>) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
    match level.as_str() {
        "error" => eprintln!("[{}] [FRONTEND ERROR] {}", timestamp, message),
        "debug" => println!("[{}] [FRONTEND DEBUG] {}", timestamp, message),
        _ => println!("[{}] [FRONTEND] {}", timestamp, message),
    }
    
    if let Some(data_str) = data {
        println!("[{}] [FRONTEND DATA] {}", timestamp, data_str);
    }
}

// Tauri command to get all connections using cross-platform provider
#[tauri::command]
fn get_connections() -> Vec<ConnectionInfoSerde> {
    let provider = create_network_provider();
    println!("Backend: Using {} network provider", provider.platform_name());
    
    match provider.get_all_connections() {
        Ok(connections) => {
            println!("Backend: Retrieved {} total connections", connections.len());
            
            let tcp_count = connections.iter().filter(|c| matches!(c.protocol, crate::models::Protocol::TCP)).count();
            let udp_count = connections.iter().filter(|c| matches!(c.protocol, crate::models::Protocol::UDP)).count();
            println!("Backend: TCP connections: {}, UDP connections: {}", tcp_count, udp_count);
            
            let result: Vec<ConnectionInfoSerde> = connections.into_iter().map(ConnectionInfoSerde::from).collect();
            println!("Backend: Returning {} serialized connections", result.len());
            result
        }
        Err(e) => {
            eprintln!("Backend Error: Failed to get connections: {}", e);
            Vec::new()
        }
    }
}

// Tauri command to get filtered connections
#[tauri::command]
fn get_filtered_connections(protocol: String, port: Option<u16>) -> Vec<ConnectionInfoSerde> {
    let provider = create_network_provider();
    
    match provider.get_all_connections() {
        Ok(all_connections) => {
            let filtered = filter_connections(&all_connections, &protocol, port);
            filtered.into_iter().map(ConnectionInfoSerde::from).collect()
        }
        Err(e) => {
            eprintln!("Backend Error: Failed to get connections for filtering: {}", e);
            Vec::new()
        }
    }
}

// Tauri command to get platform information
#[tauri::command]
fn get_platform_info() -> serde_json::Value {
    let provider = create_network_provider();
    serde_json::json!({
        "platform": provider.platform_name(),
        "supported": provider.is_supported(),
        "architecture": std::env::consts::ARCH,
        "os": std::env::consts::OS
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_connections, get_filtered_connections, log_message, get_platform_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
