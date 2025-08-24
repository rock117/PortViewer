use crate::models::{ConnectionInfo, Protocol};

pub fn filter_connections(connections: &[ConnectionInfo], protocol: &str, filter_port: Option<u16>) -> Vec<ConnectionInfo> {
    connections.iter()
        .filter(|conn| {
            // Filter by protocol
            let protocol_match = match protocol {
                "tcp" => conn.protocol == Protocol::TCP,
                "udp" => conn.protocol == Protocol::UDP,
                "all" => true,
                _ => false,
            };
            
            // Filter by port if specified (using string prefix matching)
            let port_match = if let Some(port) = filter_port {
                let port_str = port.to_string();
                conn.local_port.to_string().starts_with(&port_str) || 
                conn.remote_port.to_string().starts_with(&port_str)
            } else {
                true
            };
            
            protocol_match && port_match
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_by_protocol() {
        let connections = vec![
            ConnectionInfo::new(
                Protocol::TCP,
                "127.0.0.1".to_string(),
                80,
                "0.0.0.0".to_string(),
                0,
                ConnectionState::Listening,
                1234,
                "test.exe".to_string(),
            ),
            ConnectionInfo::new(
                Protocol::UDP,
                "127.0.0.1".to_string(),
                53,
                "*".to_string(),
                0,
                ConnectionState::Listening,
                5678,
                "dns.exe".to_string(),
            ),
        ];

        let tcp_filtered = filter_connections(&connections, "tcp", None);
        assert_eq!(tcp_filtered.len(), 1);
        assert_eq!(tcp_filtered[0].protocol, Protocol::TCP);

        let udp_filtered = filter_connections(&connections, "udp", None);
        assert_eq!(udp_filtered.len(), 1);
        assert_eq!(udp_filtered[0].protocol, Protocol::UDP);

        let all_filtered = filter_connections(&connections, "all", None);
        assert_eq!(all_filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_port() {
        let connections = vec![
            ConnectionInfo::new(
                Protocol::TCP,
                "127.0.0.1".to_string(),
                80,
                "0.0.0.0".to_string(),
                0,
                ConnectionState::Listening,
                1234,
                "test.exe".to_string(),
            ),
            ConnectionInfo::new(
                Protocol::UDP,
                "127.0.0.1".to_string(),
                53,
                "*".to_string(),
                0,
                ConnectionState::Listening,
                5678,
                "dns.exe".to_string(),
            ),
        ];

        let port_80_filtered = filter_connections(&connections, "all", Some(80));
        assert_eq!(port_80_filtered.len(), 1);
        assert_eq!(port_80_filtered[0].local_port, 80);

        let port_53_filtered = filter_connections(&connections, "all", Some(53));
        assert_eq!(port_53_filtered.len(), 1);
        assert_eq!(port_53_filtered[0].local_port, 53);

        let port_999_filtered = filter_connections(&connections, "all", Some(999));
        assert_eq!(port_999_filtered.len(), 0);
    }
}
