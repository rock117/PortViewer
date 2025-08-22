use crate::models::ConnectionInfo;
use std::error::Error;
use std::fmt;

/// Cross-platform network connection provider trait
/// 
/// This trait abstracts the platform-specific implementation details
/// for retrieving network connections, following Rust's zero-cost abstraction principle.
pub trait NetworkProvider {
    /// Get all active network connections (TCP + UDP)
    fn get_all_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError>;
    
    /// Get TCP connections only
    fn get_tcp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError>;
    
    /// Get UDP connections only  
    fn get_udp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError>;
    
    /// Get platform name for display purposes
    fn platform_name(&self) -> &'static str;
    
    /// Check if the platform is supported on current system
    fn is_supported(&self) -> bool;
}

/// Network-related errors that can occur across platforms
#[derive(Debug, Clone)]
pub enum NetworkError {
    /// Platform not supported
    UnsupportedPlatform(String),
    /// Permission denied (e.g., need root/admin privileges)
    PermissionDenied(String),
    /// System API call failed
    SystemCallFailed(String),
    /// File system access failed (Linux /proc)
    FileSystemError(String),
    /// Command execution failed (macOS/FreeBSD netstat)
    CommandFailed(String),
    /// Data parsing error
    ParseError(String),
    /// Generic I/O error
    IoError(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::UnsupportedPlatform(msg) => write!(f, "Unsupported platform: {}", msg),
            NetworkError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            NetworkError::SystemCallFailed(msg) => write!(f, "System call failed: {}", msg),
            NetworkError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            NetworkError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            NetworkError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            NetworkError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl Error for NetworkError {}

/// Factory function to create the appropriate NetworkProvider for current platform
/// 
/// Uses unified implementation with platform-specific optimizations and fallbacks
/// This ensures "开箱即用" (works out of the box) on all supported platforms
pub fn create_network_provider() -> Box<dyn NetworkProvider> {
    Box::new(crate::platform::unified::UnifiedNetworkProvider::new())
}

// Unified cross-platform implementation
pub mod unified;

// Platform-specific modules (for optimized implementations)
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "freebsd")]
pub mod freebsd;
