use crate::models::{ConnectionInfo, Protocol, ConnectionState};
use crate::platform::{NetworkProvider, NetworkError};
use std::process::Command;
use std::collections::HashMap;

/// macOS-specific network provider using netstat and lsof commands
/// 
/// This implementation uses system commands to retrieve network connection information
/// on macOS systems, providing comprehensive TCP/UDP connection monitoring
pub struct MacOSNetworkProvider;

impl MacOSNetworkProvider {
    pub fn new() -> Self {
        Self
    }

    /// Execute netstat command to get network connections
    fn execute_netstat(&self, protocol: &str) -> Result<String, NetworkError> {
        let output = Command::new("netstat")
            .args(&["-anv", "-p", protocol])
            .output()
            .map_err(|e| NetworkError::CommandFailed(format!("Failed to execute netstat: {}", e)))?;

        if !output.status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "netstat command failed: {}", 
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute lsof command to get process information for network connections
    fn execute_lsof(&self) -> Result<HashMap<String, (u32, String)>, NetworkError> {
        let output = Command::new("lsof")
            .args(&["-i", "-n", "-P"])
            .output()
            .map_err(|e| NetworkError::CommandFailed(format!("Failed to execute lsof: {}", e)))?;

        if !output.status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "lsof command failed: {}", 
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let mut process_map = HashMap::new();
        let lsof_output = String::from_utf8_lossy(&output.stdout);
        
        for line in lsof_output.lines().skip(1) { // Skip header
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 9 {
                let process_name = fields[0].to_string();
                if let Ok(pid) = fields[1].parse::<u32>() {
                    let connection_info = fields[8]; // Network connection info
                    if connection_info.contains(':') {
                        process_map.insert(connection_info.to_string(), (pid, process_name));
                    }
                }
            }
        }

        Ok(process_map)
    }

    /// Parse netstat output line for TCP connections
    fn parse_tcp_line(&self, line: &str, process_map: &HashMap<String, (u32, String)>) -> Option<ConnectionInfo> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 6 {
            return None;
        }

        // Skip non-TCP lines
        if !fields[0].starts_with("tcp") {
            return None;
        }

        let local_addr = fields[3];
        let remote_addr = fields[4];
        let state_str = fields[5];

        // Parse local address and port
        let (local_ip, local_port) = self.parse_address(local_addr)?;
        
        // Parse remote address and port
        let (remote_ip, remote_port) = self.parse_address(remote_addr)?;

        // Parse connection state
        let state = self.parse_tcp_state(state_str);

        // Try to find process info
        let (pid, process_name) = process_map
            .get(local_addr)
            .or_else(|| process_map.get(&format!("{}:{}", local_ip, local_port)))
            .copied()
            .unwrap_or((0, "unknown".to_string()));

        Some(ConnectionInfo::new(
            Protocol::TCP,
            local_ip,
            local_port,
            remote_ip,
            remote_port,
            state,
            pid,
            process_name,
        ))
    }

    /// Parse netstat output line for UDP connections
    fn parse_udp_line(&self, line: &str, process_map: &HashMap<String, (u32, String)>) -> Option<ConnectionInfo> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 4 {
            return None;
        }

        // Skip non-UDP lines
        if !fields[0].starts_with("udp") {
            return None;
        }

        let local_addr = fields[3];

        // Parse local address and port
        let (local_ip, local_port) = self.parse_address(local_addr)?;

        // Try to find process info
        let (pid, process_name) = process_map
            .get(local_addr)
            .or_else(|| process_map.get(&format!("{}:{}", local_ip, local_port)))
            .copied()
            .unwrap_or((0, "unknown".to_string()));

        Some(ConnectionInfo::new(
            Protocol::UDP,
            local_ip,
            local_port,
            "*".to_string(),
            0,
            ConnectionState::Listening,
            pid,
            process_name,
        ))
    }

    /// Parse address string in format "ip:port" or "ip.port"
    fn parse_address(&self, addr: &str) -> Option<(String, u16)> {
        // Handle IPv6 addresses [::1]:port or IPv4 addresses ip:port
        if addr.starts_with('[') {
            // IPv6 format [::1]:8080
            if let Some(bracket_end) = addr.find("]:") {
                let ip = addr[1..bracket_end].to_string();
                let port_str = &addr[bracket_end + 2..];
                if let Ok(port) = port_str.parse::<u16>() {
                    return Some((ip, port));
                }
            }
        } else {
            // IPv4 format 127.0.0.1:8080 or 127.0.0.1.8080
            let separator = if addr.contains(':') { ':' } else { '.' };
            if let Some(sep_pos) = addr.rfind(separator) {
                let ip = addr[..sep_pos].to_string();
                let port_str = &addr[sep_pos + 1..];
                if let Ok(port) = port_str.parse::<u16>() {
                    return Some((ip, port));
                }
            }
        }
        None
    }

    /// Parse TCP connection state from netstat output
    fn parse_tcp_state(&self, state_str: &str) -> ConnectionState {
        match state_str.to_uppercase().as_str() {
            "ESTABLISHED" => ConnectionState::Established,
            "SYN_SENT" => ConnectionState::SynSent,
            "SYN_RCVD" => ConnectionState::SynRecv,
            "FIN_WAIT_1" => ConnectionState::FinWait1,
            "FIN_WAIT_2" => ConnectionState::FinWait2,
            "TIME_WAIT" => ConnectionState::TimeWait,
            "CLOSED" => ConnectionState::Close,
            "CLOSE_WAIT" => ConnectionState::CloseWait,
            "LAST_ACK" => ConnectionState::LastAck,
            "LISTEN" => ConnectionState::Listening,
            "CLOSING" => ConnectionState::Closing,
            _ => ConnectionState::Unknown,
        }
    }
}

impl NetworkProvider for MacOSNetworkProvider {
    fn get_all_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        // Get process information first
        let process_map = self.execute_lsof().unwrap_or_else(|_| HashMap::new());
        
        // Get TCP connections
        if let Ok(tcp_output) = self.execute_netstat("tcp") {
            for line in tcp_output.lines() {
                if let Some(conn) = self.parse_tcp_line(line, &process_map) {
                    connections.push(conn);
                }
            }
        }
        
        // Get UDP connections
        if let Ok(udp_output) = self.execute_netstat("udp") {
            for line in udp_output.lines() {
                if let Some(conn) = self.parse_udp_line(line, &process_map) {
                    connections.push(conn);
                }
            }
        }
        
        Ok(connections)
    }

    fn get_tcp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        // Get process information first
        let process_map = self.execute_lsof().unwrap_or_else(|_| HashMap::new());
        
        // Get TCP connections
        let tcp_output = self.execute_netstat("tcp")?;
        for line in tcp_output.lines() {
            if let Some(conn) = self.parse_tcp_line(line, &process_map) {
                connections.push(conn);
            }
        }
        
        Ok(connections)
    }

    fn get_udp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        // Get process information first
        let process_map = self.execute_lsof().unwrap_or_else(|_| HashMap::new());
        
        // Get UDP connections
        let udp_output = self.execute_netstat("udp")?;
        for line in udp_output.lines() {
            if let Some(conn) = self.parse_udp_line(line, &process_map) {
                connections.push(conn);
            }
        }
        
        Ok(connections)
    }

    fn platform_name(&self) -> &'static str {
        "macOS"
    }

    fn is_supported(&self) -> bool {
        // Check if netstat and lsof commands are available
        Command::new("netstat").arg("--help").output().is_ok() &&
        Command::new("lsof").arg("-v").output().is_ok()
    }
}
