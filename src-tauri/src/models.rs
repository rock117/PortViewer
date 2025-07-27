#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

impl From<&str> for Protocol {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TCP" => Protocol::TCP,
            "UDP" => Protocol::UDP,
            _ => Protocol::TCP, // Default fallback
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    // TCP states
    Closed,
    Listen,
    SynSent,
    SynRcvd,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
    DeleteTcb,
    // UDP state
    Listening,
    // Unknown state
    Unknown(u32),
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Closed => write!(f, "CLOSED"),
            ConnectionState::Listen => write!(f, "LISTEN"),
            ConnectionState::SynSent => write!(f, "SYN_SENT"),
            ConnectionState::SynRcvd => write!(f, "SYN_RCVD"),
            ConnectionState::Established => write!(f, "ESTABLISHED"),
            ConnectionState::FinWait1 => write!(f, "FIN_WAIT1"),
            ConnectionState::FinWait2 => write!(f, "FIN_WAIT2"),
            ConnectionState::CloseWait => write!(f, "CLOSE_WAIT"),
            ConnectionState::Closing => write!(f, "CLOSING"),
            ConnectionState::LastAck => write!(f, "LAST_ACK"),
            ConnectionState::TimeWait => write!(f, "TIME_WAIT"),
            ConnectionState::DeleteTcb => write!(f, "DELETE_TCB"),
            ConnectionState::Listening => write!(f, "LISTENING"),
            ConnectionState::Unknown(code) => write!(f, "UNKNOWN({})", code),
        }
    }
}

impl From<u32> for ConnectionState {
    fn from(state: u32) -> Self {
        match state {
            1 => ConnectionState::Closed,
            2 => ConnectionState::Listen,
            3 => ConnectionState::SynSent,
            4 => ConnectionState::SynRcvd,
            5 => ConnectionState::Established,
            6 => ConnectionState::FinWait1,
            7 => ConnectionState::FinWait2,
            8 => ConnectionState::CloseWait,
            9 => ConnectionState::Closing,
            10 => ConnectionState::LastAck,
            11 => ConnectionState::TimeWait,
            12 => ConnectionState::DeleteTcb,
            _ => ConnectionState::Unknown(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub protocol: Protocol,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: String,
    pub remote_port: u16,
    pub state: ConnectionState,
    pub pid: u32,
    pub process_name: String,
}

impl ConnectionInfo {
    pub fn new(
        protocol: Protocol,
        local_address: String,
        local_port: u16,
        remote_address: String,
        remote_port: u16,
        state: ConnectionState,
        pid: u32,
        process_name: String,
    ) -> Self {
        Self {
            protocol,
            local_address,
            local_port,
            remote_address,
            remote_port,
            state,
            pid,
            process_name,
        }
    }
}
