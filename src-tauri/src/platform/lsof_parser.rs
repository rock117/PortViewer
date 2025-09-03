use crate::models::{ConnectionInfo, Protocol, ConnectionState};
use crate::platform::NetworkError;
use std::process::Command;

/// Unified lsof parser for macOS and Linux
/// 
/// Since lsof command format and output are identical on both platforms,
/// this provides a shared implementation to avoid code duplication
pub struct LsofParser;

impl LsofParser {
    pub fn new() -> Self {
        Self
    }

    /// Execute lsof command to get network connections by protocol
    pub fn execute_lsof_by_protocol(&self, protocol: &str) -> Result<String, NetworkError> {
        let protocol_arg = format!("-i{}", protocol);
        let output = Command::new("lsof")
            .args(&[&protocol_arg, "-n", "-P"])
            .output()
            .map_err(|e| NetworkError::CommandFailed(format!("Failed to execute lsof: {}", e)))?;

        if !output.status.success() {
            return Err(NetworkError::CommandFailed(format!(
                "lsof command failed: {}", 
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute lsof command to get all network connections
    pub fn execute_lsof_all(&self) -> Result<String, NetworkError> {
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

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Parse lsof output to get all connections
    pub fn parse_all_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        let lsof_output = self.execute_lsof_all()?;
        
        for line in lsof_output.lines().skip(1) { // Skip header
            // Try parsing as TCP connection
            if let Some(conn) = self.parse_lsof_tcp_line(line) {
                connections.push(conn);
            }
            // Try parsing as UDP connection
            else if let Some(conn) = self.parse_lsof_udp_line(line) {
                connections.push(conn);
            }
        }
        
        Ok(connections)
    }

    /// Parse lsof output to get TCP connections
    pub fn parse_tcp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        let lsof_output = self.execute_lsof_by_protocol("tcp")?;
        
        for line in lsof_output.lines().skip(1) { // Skip header
            if let Some(conn) = self.parse_lsof_tcp_line(line) {
                connections.push(conn);
            }
        }
        
        Ok(connections)
    }

    /// Parse lsof output to get UDP connections
    pub fn parse_udp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        let lsof_output = self.execute_lsof_by_protocol("udp")?;
        
        for line in lsof_output.lines().skip(1) { // Skip header
            if let Some(conn) = self.parse_lsof_udp_line(line) {
                connections.push(conn);
            }
        }
        
        Ok(connections)
    }

    /// Parse lsof output line for TCP connections
    fn parse_lsof_tcp_line(&self, line: &str) -> Option<ConnectionInfo> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 9 {
            return None;
        }

        let process_name = fields[0].to_string();
        let pid = fields[1].parse::<u32>().ok()?;
        let connection_info = fields[8];

        // Skip non-TCP connections
        if !connection_info.contains("TCP") {
            return None;
        }

        // Parse connection info: format like "*:8080 (LISTEN)" or "127.0.0.1:8080->192.168.1.1:80 (ESTABLISHED)"
        let connection_part = if let Some(paren_pos) = connection_info.find('(') {
            &connection_info[..paren_pos]
        } else {
            connection_info
        };

        let state = self.extract_tcp_state_from_lsof(connection_info);

        if connection_part.contains("->") {
            // Established connection: local->remote
            let parts: Vec<&str> = connection_part.split("->").collect();
            if parts.len() == 2 {
                let (local_ip, local_port) = self.parse_address(parts[0])?;
                let (remote_ip, remote_port) = self.parse_address(parts[1])?;
                
                return Some(ConnectionInfo::new(
                    Protocol::TCP,
                    local_ip,
                    local_port,
                    remote_ip,
                    remote_port,
                    state,
                    pid,
                    process_name,
                ));
            }
        } else {
            // Listening connection: *:port or ip:port
            let (local_ip, local_port) = self.parse_address(connection_part)?;
            
            return Some(ConnectionInfo::new(
                Protocol::TCP,
                local_ip,
                local_port,
                "*".to_string(),
                0,
                state,
                pid,
                process_name,
            ));
        }

        None
    }

    /// Parse lsof output line for UDP connections
    fn parse_lsof_udp_line(&self, line: &str) -> Option<ConnectionInfo> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 9 {
            return None;
        }

        let process_name = fields[0].to_string();
        let pid = fields[1].parse::<u32>().ok()?;
        let connection_info = fields[8];

        // Skip non-UDP connections
        if !connection_info.contains("UDP") {
            return None;
        }

        // Parse connection info: format like "*:8080" or "127.0.0.1:8080"
        let connection_part = if let Some(paren_pos) = connection_info.find('(') {
            &connection_info[..paren_pos]
        } else {
            connection_info
        };

        let (local_ip, local_port) = self.parse_address(connection_part)?;

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

    /// Extract TCP connection state from lsof output
    fn extract_tcp_state_from_lsof(&self, connection_info: &str) -> ConnectionState {
        if let Some(start) = connection_info.find('(') {
            if let Some(end) = connection_info.find(')') {
                let state_str = &connection_info[start + 1..end];
                return self.parse_tcp_state(state_str);
            }
        }
        ConnectionState::Unknown(0)
    }

    /// Parse TCP connection state from state string
    fn parse_tcp_state(&self, state_str: &str) -> ConnectionState {
        match state_str.to_uppercase().as_str() {
            "ESTABLISHED" => ConnectionState::Established,
            "SYN_SENT" => ConnectionState::SynSent,
            "SYN_RCVD" => ConnectionState::SynRcvd,
            "FIN_WAIT_1" => ConnectionState::FinWait1,
            "FIN_WAIT_2" => ConnectionState::FinWait2,
            "TIME_WAIT" => ConnectionState::TimeWait,
            "CLOSED" => ConnectionState::Closed,
            "CLOSE_WAIT" => ConnectionState::CloseWait,
            "LAST_ACK" => ConnectionState::LastAck,
            "LISTEN" => ConnectionState::Listening,
            "CLOSING" => ConnectionState::Closing,
            _ => ConnectionState::Unknown(0),
        }
    }

    /// Check if lsof command is available
    pub fn is_supported(&self) -> bool {
        Command::new("lsof").arg("-v").output().is_ok()
    }
}
