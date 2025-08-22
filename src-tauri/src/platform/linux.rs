use crate::models::{ConnectionInfo, Protocol, ConnectionState};
use crate::platform::{NetworkProvider, NetworkError};
use procfs::net::{tcp, tcp6, udp, udp6, TcpState};
use procfs::process::{all_processes, FDTarget};
use std::collections::HashMap;
use std::path::Path;

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
}

impl NetworkProvider for LinuxNetworkProvider {
    fn get_all_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        let mut connections = Vec::new();
        
        // Use procfs crate for cleaner implementation
        let process_map = self.build_process_inode_map()?;
        
        // Get TCP connections using procfs
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
        
        // Get TCP6 connections
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
        
        // Get UDP connections
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
        
        // Get UDP6 connections
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
