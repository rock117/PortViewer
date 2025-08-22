use crate::models::{ConnectionInfo, Protocol, ConnectionState};
use crate::platform::{NetworkProvider, NetworkError};
use procfs::net::{tcp, tcp6, udp, udp6, TcpState};
use procfs::process::{all_processes, FDTarget};
use std::collections::HashMap;

/// Linux-specific network provider using procfs crate
/// 
/// This implementation uses the procfs crate to avoid manual parsing
/// of /proc filesystem files, providing better reliability and maintainability
pub struct LinuxNetworkProvider;

impl LinuxNetworkProvider {
    pub fn new() -> Self {
        Self
    }

    /// Build a map of socket inodes to process PIDs using procfs
    fn build_process_inode_map(&self) -> Result<HashMap<u32, u32>, NetworkError> {
        let mut map = HashMap::new();
        
        match all_processes() {
            Ok(processes) => {
                for process in processes {
                    if let Ok(process) = process {
                        if let Ok(fds) = process.fd() {
                            for fd in fds {
                                if let FDTarget::Socket(inode) = fd.target {
                                    map.insert(inode, process.stat.pid as u32);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                return Err(NetworkError::SystemError(format!("Failed to read processes: {}", e)));
            }
        }
        
        Ok(map)
    }

    /// Get process name from PID
    fn get_process_name(&self, pid: u32) -> String {
        match procfs::process::Process::new(pid as i32) {
            Ok(process) => {
                process.stat.comm
            }
            Err(_) => "unknown".to_string()
        }
    }

    /// Convert procfs TcpState to our ConnectionState
    fn convert_tcp_state(&self, state: TcpState) -> ConnectionState {
        match state {
            TcpState::Established => ConnectionState::Established,
            TcpState::SynSent => ConnectionState::SynSent,
            TcpState::SynRecv => ConnectionState::SynRecv,
            TcpState::FinWait1 => ConnectionState::FinWait1,
            TcpState::FinWait2 => ConnectionState::FinWait2,
            TcpState::TimeWait => ConnectionState::TimeWait,
            TcpState::Close => ConnectionState::Close,
            TcpState::CloseWait => ConnectionState::CloseWait,
            TcpState::LastAck => ConnectionState::LastAck,
            TcpState::Listen => ConnectionState::Listening,
            TcpState::Closing => ConnectionState::Closing,
            _ => ConnectionState::Unknown,
        }
    }

    /// Parse /proc/net/tcp format
    /// Format: sl local_address rem_address st tx_queue rx_queue tr tm->when retrnsmt uid timeout inode
    fn parse_tcp_line(&self, line: &str) -> Result<Option<ConnectionInfo>, NetworkError> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 {
            return Ok(None); // Skip header or malformed lines
        }

        // Skip the first line (header)
        if fields[0] == "sl" {
            return Ok(None);
        }

        // Parse local address (format: XXXXXXXX:XXXX)
        let local_parts: Vec<&str> = fields[1].split(':').collect();
        if local_parts.len() != 2 {
            return Err(NetworkError::ParseError(format!("Invalid local address format: {}", fields[1])));
        }

        let local_addr = self.parse_hex_ip(local_parts[0])?;
        let local_port = u16::from_str_radix(local_parts[1], 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid local port: {}", e)))?;

        // Parse remote address
        let remote_parts: Vec<&str> = fields[2].split(':').collect();
        if remote_parts.len() != 2 {
            return Err(NetworkError::ParseError(format!("Invalid remote address format: {}", fields[2])));
        }

        let remote_addr = self.parse_hex_ip(remote_parts[0])?;
        let remote_port = u16::from_str_radix(remote_parts[1], 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid remote port: {}", e)))?;

        // Parse connection state
        let state_hex = fields[3];
        let state_num = u8::from_str_radix(state_hex, 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid state: {}", e)))?;
        
        let state = match state_num {
            0x01 => ConnectionState::Established,
            0x02 => ConnectionState::SynSent,
            0x03 => ConnectionState::SynRecv,
            0x04 => ConnectionState::FinWait1,
            0x05 => ConnectionState::FinWait2,
            0x06 => ConnectionState::TimeWait,
            0x07 => ConnectionState::Close,
            0x08 => ConnectionState::CloseWait,
            0x09 => ConnectionState::LastAck,
            0x0A => ConnectionState::Listening,
            0x0B => ConnectionState::Closing,
            _ => ConnectionState::Unknown,
        };

        // Parse UID (we'll use this to get PID via /proc/net/tcp)
        let uid = fields[7].parse::<u32>()
            .map_err(|e| NetworkError::ParseError(format!("Invalid UID: {}", e)))?;

        // Parse inode
        let inode = fields[9].parse::<u64>()
            .map_err(|e| NetworkError::ParseError(format!("Invalid inode: {}", e)))?;

        // Find PID by inode (this is the tricky part)
        let pid = self.find_pid_by_inode(inode).unwrap_or(0);
        let process_name = if pid > 0 {
            self.get_process_name(pid)
        } else {
            "unknown".to_string()
        };

        Ok(Some(ConnectionInfo::new(
            Protocol::TCP,
            local_addr,
            local_port,
            remote_addr,
            remote_port,
            state,
            pid,
            process_name,
        )))
    }

    /// Parse hexadecimal IP address to dotted decimal
    fn parse_hex_ip(&self, hex_ip: &str) -> Result<String, NetworkError> {
        if hex_ip.len() != 8 {
            return Err(NetworkError::ParseError(format!("Invalid IP hex length: {}", hex_ip)));
        }

        let ip_num = u32::from_str_radix(hex_ip, 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid IP hex: {}", e)))?;

        // Convert from little-endian network byte order
        let a = (ip_num & 0xFF) as u8;
        let b = ((ip_num >> 8) & 0xFF) as u8;
        let c = ((ip_num >> 16) & 0xFF) as u8;
        let d = ((ip_num >> 24) & 0xFF) as u8;

        Ok(format!("{}.{}.{}.{}", a, b, c, d))
    }

    /// Find PID by socket inode (expensive operation, should be optimized)
    fn find_pid_by_inode(&self, target_inode: u64) -> Option<u32> {
        // This is a simplified implementation
        // In practice, you'd want to cache this or use a more efficient approach
        if let Ok(proc_entries) = fs::read_dir("/proc") {
            for entry in proc_entries.flatten() {
                if let Ok(pid_str) = entry.file_name().into_string() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if let Ok(fd_entries) = fs::read_dir(format!("/proc/{}/fd", pid)) {
                            for fd_entry in fd_entries.flatten() {
                                if let Ok(link) = fs::read_link(fd_entry.path()) {
                                    if let Some(link_str) = link.to_str() {
                                        if link_str.starts_with("socket:[") && link_str.ends_with(']') {
                                            let inode_str = &link_str[8..link_str.len()-1];
                                            if let Ok(inode) = inode_str.parse::<u64>() {
                                                if inode == target_inode {
                                                    return Some(pid);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Build a map of process inodes to PIDs for efficient lookup
    fn build_process_inode_map(&self) -> Result<HashMap<u64, u32>, NetworkError> {
        let mut process_map = HashMap::new();

        if let Ok(proc_entries) = fs::read_dir("/proc") {
            for entry in proc_entries.flatten() {
                if let Ok(pid_str) = entry.file_name().into_string() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if let Ok(fd_entries) = fs::read_dir(format!("/proc/{}/fd", pid)) {
                            for fd_entry in fd_entries.flatten() {
                                if let Ok(link) = fs::read_link(fd_entry.path()) {
                                    if let Some(link_str) = link.to_str() {
                                        if link_str.starts_with("socket:[") && link_str.ends_with(']') {
                                            let inode_str = &link_str[8..link_str.len()-1];
                                            if let Ok(inode) = inode_str.parse::<u64>() {
                                                process_map.insert(inode, pid);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(process_map)
    }

    /// Parse /proc/net/tcp format with process inode map
    fn parse_tcp_line_with_procfs(&self, line: &str, process_map: &HashMap<u64, u32>) -> Result<Option<ConnectionInfo>, NetworkError> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 {
            return Ok(None); // Skip header or malformed lines
        }

        // Skip the first line (header)
        if fields[0] == "sl" {
            return Ok(None);
        }

        // Parse local address (format: XXXXXXXX:XXXX)
        let local_parts: Vec<&str> = fields[1].split(':').collect();
        if local_parts.len() != 2 {
            return Err(NetworkError::ParseError(format!("Invalid local address format: {}", fields[1])));
        }

        let local_addr = self.parse_hex_ip(local_parts[0])?;
        let local_port = u16::from_str_radix(local_parts[1], 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid local port: {}", e)))?;

        // Parse remote address
        let remote_parts: Vec<&str> = fields[2].split(':').collect();
        if remote_parts.len() != 2 {
            return Err(NetworkError::ParseError(format!("Invalid remote address format: {}", fields[2])));
        }

        let remote_addr = self.parse_hex_ip(remote_parts[0])?;
        let remote_port = u16::from_str_radix(remote_parts[1], 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid remote port: {}", e)))?;

        // Parse connection state
        let state_hex = fields[3];
        let state_num = u8::from_str_radix(state_hex, 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid state: {}", e)))?;
        
        let state = match state_num {
            0x01 => ConnectionState::Established,
            0x02 => ConnectionState::SynSent,
            0x03 => ConnectionState::SynRecv,
            0x04 => ConnectionState::FinWait1,
            0x05 => ConnectionState::FinWait2,
            0x06 => ConnectionState::TimeWait,
            0x07 => ConnectionState::Close,
            0x08 => ConnectionState::CloseWait,
            0x09 => ConnectionState::LastAck,
            0x0A => ConnectionState::Listening,
            0x0B => ConnectionState::Closing,
            _ => ConnectionState::Unknown,
        };

        // Parse inode
        let inode = fields[9].parse::<u64>()
            .map_err(|e| NetworkError::ParseError(format!("Invalid inode: {}", e)))?;

        // Find PID by inode using the process map
        let pid = *process_map.get(&inode).unwrap_or(&0);
        let process_name = if pid > 0 {
            self.get_process_name(pid)
        } else {
            "unknown".to_string()
        };

        Ok(Some(ConnectionInfo::new(
            Protocol::TCP,
            local_addr,
            local_port,
            remote_addr,
            remote_port,
            state,
            pid,
            process_name,
        )))
    }

    /// Get TCP connections (IPv4 and IPv6) using /proc/net/tcp and /proc/net/tcp6
    fn get_tcp_connections_with_procfs(&self, process_map: &HashMap<u64, u32>) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();

        // Parse /proc/net/tcp
        let content = fs::read_to_string("/proc/net/tcp")
            .map_err(|e| NetworkError::FileSystemError(format!("Failed to read /proc/net/tcp: {}", e)))?;
        for line in content.lines() {
            if let Some(conn) = self.parse_tcp_line_with_procfs(line, process_map)? {
                connections.push(conn);
            }
        }

        // Parse /proc/net/tcp6
        let content = fs::read_to_string("/proc/net/tcp6")
            .map_err(|e| NetworkError::FileSystemError(format!("Failed to read /proc/net/tcp6: {}", e)))?;
        for line in content.lines() {
            if let Some(conn) = self.parse_tcp_line_with_procfs(line, process_map)? {
                connections.push(conn);
            }
        }

        Ok(connections)
    }

    /// Parse /proc/net/udp format with process inode map
    fn parse_udp_line_with_procfs(&self, line: &str, process_map: &HashMap<u64, u32>) -> Result<Option<ConnectionInfo>, NetworkError> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 8 {
            return Ok(None); // Skip header or malformed lines
        }

        // Skip the first line (header)
        if fields[0] == "sl" {
            return Ok(None);
        }

        // Parse local address (format: XXXXXXXX:XXXX)
        let local_parts: Vec<&str> = fields[1].split(':').collect();
        if local_parts.len() != 2 {
            return Err(NetworkError::ParseError(format!("Invalid local address format: {}", fields[1])));
        }

        let local_addr = self.parse_hex_ip(local_parts[0])?;
        let local_port = u16::from_str_radix(local_parts[1], 16)
            .map_err(|e| NetworkError::ParseError(format!("Invalid local port: {}", e)))?;

        // Parse inode
        let inode = fields[9].parse::<u64>()
            .map_err(|e| NetworkError::ParseError(format!("Invalid inode: {}", e)))?;

        // Find PID by inode using the process map
        let pid = *process_map.get(&inode).unwrap_or(&0);
        let process_name = if pid > 0 {
            self.get_process_name(pid)
        } else {
            "unknown".to_string()
        };

        Ok(Some(ConnectionInfo::new(
            Protocol::UDP,
            local_addr,
            local_port,
            "*".to_string(),
            0,
            ConnectionState::Listening,
            pid,
            process_name,
        )))
    }

    /// Get UDP connections (IPv4 and IPv6) using /proc/net/udp and /proc/net/udp6
    fn get_udp_connections_with_procfs(&self, process_map: &HashMap<u64, u32>) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();

        // Parse /proc/net/udp
        let content = fs::read_to_string("/proc/net/udp")
            .map_err(|e| NetworkError::FileSystemError(format!("Failed to read /proc/net/udp: {}", e)))?;
        for line in content.lines() {
            if let Some(conn) = self.parse_udp_line_with_procfs(line, process_map)? {
                connections.push(conn);
            }
        }

        // Parse /proc/net/udp6
        let content = fs::read_to_string("/proc/net/udp6")
            .map_err(|e| NetworkError::FileSystemError(format!("Failed to read /proc/net/udp6: {}", e)))?;
        for line in content.lines() {
            if let Some(conn) = self.parse_udp_line_with_procfs(line, process_map)? {
                connections.push(conn);
            }
        }

        Ok(connections)
    }
}

impl NetworkProvider for LinuxNetworkProvider {
    fn get_all_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        // Use procfs crate for cleaner implementation
        let process_map = self.build_process_inode_map()?;
        
        // Get TCP connections using procfs
        match tcp() {
            Ok(tcp_entries) => {
                for entry in tcp_entries {
                    let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                    let process_name = if pid > 0 {
                        self.get_process_name(pid)
                    } else {
                        "unknown".to_string()
                    };
                    
                    connections.push(ConnectionInfo::new(
                        Protocol::TCP,
                        entry.local_address.to_string(),
                        entry.local_address.port(),
                        entry.remote_address.to_string(),
                        entry.remote_address.port(),
                        self.convert_tcp_state(entry.state),
                        pid,
                        process_name,
                    ));
                }
            }
            Err(e) => {
                println!("Failed to read TCP connections with procfs: {}", e);
            }
        }
        
        // Get TCP6 connections
        match tcp6() {
            Ok(tcp6_entries) => {
                for entry in tcp6_entries {
                    let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                    let process_name = if pid > 0 {
                        self.get_process_name(pid)
                    } else {
                        "unknown".to_string()
                    };
                    
                    connections.push(ConnectionInfo::new(
                        Protocol::TCP,
                        entry.local_address.to_string(),
                        entry.local_address.port(),
                        entry.remote_address.to_string(),
                        entry.remote_address.port(),
                        self.convert_tcp_state(entry.state),
                        pid,
                        process_name,
                    ));
                }
            }
            Err(e) => {
                println!("Failed to read TCP6 connections with procfs: {}", e);
            }
        }
        
        // Get UDP connections
        match udp() {
            Ok(udp_entries) => {
                for entry in udp_entries {
                    let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                    let process_name = if pid > 0 {
                        self.get_process_name(pid)
                    } else {
                        "unknown".to_string()
                    };
                    
                    connections.push(ConnectionInfo::new(
                        Protocol::UDP,
                        entry.local_address.to_string(),
                        entry.local_address.port(),
                        "*".to_string(),
                        0,
                        ConnectionState::Listening,
                        pid,
                        process_name,
                    ));
                }
            }
            Err(e) => {
                println!("Failed to read UDP connections with procfs: {}", e);
            }
        }
        
        // Get UDP6 connections
        match udp6() {
            Ok(udp6_entries) => {
                for entry in udp6_entries {
                    let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                    let process_name = if pid > 0 {
                        self.get_process_name(pid)
                    } else {
                        "unknown".to_string()
                    };
                    
                    connections.push(ConnectionInfo::new(
                        Protocol::UDP,
                        entry.local_address.to_string(),
                        entry.local_address.port(),
                        "*".to_string(),
                        0,
                        ConnectionState::Listening,
                        pid,
                        process_name,
                    ));
                }
            }
            Err(e) => {
                println!("Failed to read UDP6 connections with procfs: {}", e);
            }
        }
        
        Ok(connections)
    }

    fn get_tcp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        let process_map = self.build_process_inode_map()?;
        
        // TCP IPv4
        if let Ok(tcp_entries) = tcp() {
            for entry in tcp_entries {
                let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                let process_name = if pid > 0 {
                    self.get_process_name(pid)
                } else {
                    "unknown".to_string()
                };
                
                connections.push(ConnectionInfo::new(
                    Protocol::TCP,
                    entry.local_address.to_string(),
                    entry.local_address.port(),
                    entry.remote_address.to_string(),
                    entry.remote_address.port(),
                    self.convert_tcp_state(entry.state),
                    pid,
                    process_name,
                ));
            }
        }
        
        // TCP IPv6
        if let Ok(tcp6_entries) = tcp6() {
            for entry in tcp6_entries {
                let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                let process_name = if pid > 0 {
                    self.get_process_name(pid)
                } else {
                    "unknown".to_string()
                };
                
                connections.push(ConnectionInfo::new(
                    Protocol::TCP,
                    entry.local_address.to_string(),
                    entry.local_address.port(),
                    entry.remote_address.to_string(),
                    entry.remote_address.port(),
                    self.convert_tcp_state(entry.state),
                    pid,
                    process_name,
                ));
            }
        }
        
        Ok(connections)
    }

    fn get_udp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        let process_map = self.build_process_inode_map()?;
        
        // UDP IPv4
        if let Ok(udp_entries) = udp() {
            for entry in udp_entries {
                let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                let process_name = if pid > 0 {
                    self.get_process_name(pid)
                } else {
                    "unknown".to_string()
                };
                
                connections.push(ConnectionInfo::new(
                    Protocol::UDP,
                    entry.local_address.to_string(),
                    entry.local_address.port(),
                    "*".to_string(),
                    0,
                    ConnectionState::Listening,
                    pid,
                    process_name,
                ));
            }
        }
        
        // UDP IPv6
        if let Ok(udp6_entries) = udp6() {
            for entry in udp6_entries {
                let pid = process_map.get(&entry.inode).copied().unwrap_or(0);
                let process_name = if pid > 0 {
                    self.get_process_name(pid)
                } else {
                    "unknown".to_string()
                };
                
                connections.push(ConnectionInfo::new(
                    Protocol::UDP,
                    entry.local_address.to_string(),
                    entry.local_address.port(),
                    "*".to_string(),
                    0,
                    ConnectionState::Listening,
                    pid,
                    process_name,
                ));
            }
        }
        
        Ok(connections)
    }

    fn platform_name(&self) -> &'static str {
        "Linux"
    }

    fn is_supported(&self) -> bool {
        Path::new("/proc/net/tcp").exists() && Path::new("/proc/net/udp").exists()
    }
}
