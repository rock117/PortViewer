use crate::models::{ConnectionInfo, Protocol, ConnectionState};
use crate::platform::{NetworkProvider, NetworkError};
use crate::platform::lsof_parser::LsofParser;

/// macOS-specific network provider using unified lsof parser
/// 
/// This implementation uses the shared LsofParser for consistent behavior
/// across macOS and Linux platforms
pub struct MacOSNetworkProvider {
    parser: LsofParser,
}

impl MacOSNetworkProvider {
    pub fn new() -> Self {
        Self {
            parser: LsofParser::new(),
        }
    }

}

impl NetworkProvider for MacOSNetworkProvider {
    fn get_all_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        self.parser.parse_all_connections()
    }

    fn get_tcp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        self.parser.parse_tcp_connections()
    }

    fn get_udp_connections(&self) -> Result<Vec<ConnectionInfo>, NetworkError> {
        self.parser.parse_udp_connections()
    }

    fn platform_name(&self) -> &'static str {
        "macOS"
    }

    fn is_supported(&self) -> bool {
        self.parser.is_supported()
    }
}
