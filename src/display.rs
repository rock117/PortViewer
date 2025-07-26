use crate::models::{ConnectionInfo, Protocol, ConnectionState};

pub fn display_connections(connections: &[ConnectionInfo]) {
    println!("Windows Port Usage:");
    println!("{:<8} {:<22} {:<22} {:<12} {:<8} {:<20}", "Protocol", "Local Address", "Remote Address", "State", "PID", "Process Name");
    println!("{}", "-".repeat(110));
    
    for conn in connections {
        let local_addr = format!("{}:{}", conn.local_address, conn.local_port);
        let remote_addr = if conn.remote_port == 0 {
            "*:*".to_string()
        } else {
            format!("{}:{}", conn.remote_address, conn.remote_port)
        };
        
        println!(
            "{:<8} {:<22} {:<22} {:<12} {:<8} {:<20}",
            conn.protocol,
            local_addr,
            remote_addr,
            conn.state,
            conn.pid,
            conn.process_name
        );
    }
}

pub fn display_summary(connections: &[ConnectionInfo]) {
    let tcp_count = connections.iter().filter(|c| c.protocol == Protocol::TCP).count();
    let udp_count = connections.iter().filter(|c| c.protocol == Protocol::UDP).count();
    
    println!("\nSummary:");
    println!("TCP connections: {}", tcp_count);
    println!("UDP connections: {}", udp_count);
    println!("Total connections: {}", connections.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_functions_dont_panic() {
        let connections = vec![
            ConnectionInfo::new(
                Protocol::TCP,
                "127.0.0.1".to_string(),
                80,
                "192.168.1.1".to_string(),
                8080,
                ConnectionState::Established,
                1234,
                "test.exe".to_string(),
            ),
            ConnectionInfo::new(
                Protocol::UDP,
                "0.0.0.0".to_string(),
                53,
                "*".to_string(),
                0,
                ConnectionState::Listening,
                5678,
                "dns.exe".to_string(),
            ),
        ];

        // These should not panic
        display_connections(&connections);
        display_summary(&connections);
    }
}
