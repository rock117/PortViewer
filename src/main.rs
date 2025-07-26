use std::ffi::c_void;
use windows::Win32::NetworkManagement::IpHelper::*;
use windows::Win32::Foundation::*;
use windows::Win32::Networking::WinSock::*;
use windows::Win32::System::Threading::*;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("windows-port-viewer")
        .about("Windows Port Usage Viewer")
        .arg(
            Arg::new("protocol")
                .short('p')
                .long("protocol")
                .value_name("PROTOCOL")
                .help("Specify protocol type (tcp, udp, all)")
                .default_value("all")
        )
        .arg(
            Arg::new("port")
                .short('P')
                .long("port")
                .value_name("PORT")
                .help("Filter by specific port")
        )
        .get_matches();

    let protocol = matches.get_one::<String>("protocol").unwrap();
    let filter_port = matches.get_one::<String>("port").map(|s| s.parse::<u16>().ok()).flatten();

    println!("Windows Port Usage:");
    println!("{:<8} {:<22} {:<22} {:<12} {:<8} {:<20}", "Protocol", "Local Address", "Remote Address", "State", "PID", "Process Name");
    println!("{}", "-".repeat(110));

    match protocol.as_str() {
        "tcp" => list_tcp_connections(filter_port),
        "udp" => list_udp_connections(filter_port),
        "all" => {
            list_tcp_connections(filter_port);
            list_udp_connections(filter_port);
        }
        _ => {
            eprintln!("Unsupported protocol type: {}", protocol);
            std::process::exit(1);
        }
    }
}

fn list_tcp_connections(filter_port: Option<u16>) {
    unsafe {
        let mut size = 0u32;
        
        // First get the required buffer size
        GetExtendedTcpTable(
            None,
            &mut size,
            FALSE,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if size == 0 {
            return;
        }

        // allocate buffer
        let mut buffer = vec![0u8; size as usize];
        let result = GetExtendedTcpTable(
            Some(buffer.as_mut_ptr() as *mut c_void),
            &mut size,
            FALSE,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result != 0 {
            eprintln!("Failed to obtain TCP connection table: {}", result);
            return;
        }

        let table_ptr = buffer.as_ptr() as *const u32;
        let num_entries = *table_ptr;
        
        // Skip the dwNumEntries field and get to the actual entries
        let entries_ptr = table_ptr.add(1) as *const MIB_TCPROW_OWNER_PID;
        
        for i in 0..num_entries {
            let entry = &*entries_ptr.add(i as usize);
            
            let local_addr = format_ip_address(entry.dwLocalAddr);
            let local_port = u16::from_be(entry.dwLocalPort as u16);
            let remote_addr = format_ip_address(entry.dwRemoteAddr);
            let remote_port = u16::from_be(entry.dwRemotePort as u16);
            let state = format_tcp_state(entry.dwState);
            let pid = entry.dwOwningPid;
            let process_name = get_process_name(pid);

            // apply port filter
            if let Some(port) = filter_port {
                if local_port != port && remote_port != port {
                    continue;
                }
            }

            println!(
                "{:<8} {:<22} {:<22} {:<12} {:<8} {:<20}",
                "TCP",
                format!("{}:{}", local_addr, local_port),
                if remote_port == 0 { "*:*".to_string() } else { format!("{}:{}", remote_addr, remote_port) },
                state,
                pid,
                process_name
            );
        }
    }
}

fn list_udp_connections(filter_port: Option<u16>) {
    unsafe {
        let mut size = 0u32;
        
        // get required buffer size
        GetExtendedUdpTable(
            None,
            &mut size,
            FALSE,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if size == 0 {
            return;
        }

        // allocate buffer
        let mut buffer = vec![0u8; size as usize];
        let result = GetExtendedUdpTable(
            Some(buffer.as_mut_ptr() as *mut c_void),
            &mut size,
            FALSE,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if result != 0 {
            eprintln!("Failed to obtain UDP connection table: {}", result);
            return;
        }

        let table_ptr = buffer.as_ptr() as *const u32;
        let num_entries = *table_ptr;
        
        // Skip the dwNumEntries field and get to the actual entries
        let entries_ptr = table_ptr.add(1) as *const MIB_UDPROW_OWNER_PID;
        
        for i in 0..num_entries {
            let entry = &*entries_ptr.add(i as usize);
            
            let local_addr = format_ip_address(entry.dwLocalAddr);
            let local_port = u16::from_be(entry.dwLocalPort as u16);
            let pid = entry.dwOwningPid;
            let process_name = get_process_name(pid);

            // apply port filter
            if let Some(port) = filter_port {
                if local_port != port {
                    continue;
                }
            }

            println!(
                "{:<8} {:<22} {:<22} {:<12} {:<8} {:<20}",
                "UDP",
                format!("{}:{}", local_addr, local_port),
                "*:*",
                "LISTENING",
                pid,
                process_name
            );
        }
    }
}

fn format_ip_address(addr: u32) -> String {
    let bytes = addr.to_le_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}

fn get_process_name(pid: u32) -> String {
    unsafe {
        // try to open process
        let process_handle = match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid) {
            Ok(handle) => handle,
            Err(_) => return "<Unknown>".to_string(),
        };

        // get process executable file path
        let mut buffer = [0u16; 260]; // MAX_PATH
        let mut size = buffer.len() as u32;
        
        let result = QueryFullProcessImageNameW(
            process_handle, 
            PROCESS_NAME_WIN32, 
            windows::core::PWSTR(buffer.as_mut_ptr()), 
            &mut size
        );
        let _ = CloseHandle(process_handle);
        
        if result.is_ok() && size > 0 {
            // convert UTF-16 to String and extract filename
            let path = String::from_utf16_lossy(&buffer[..size as usize]);
            if let Some(filename) = path.split('\\').last() {
                filename.to_string()
            } else {
                path
            }
        } else {
            "<Unknown>".to_string()
        }
    }
}

fn format_tcp_state(state: u32) -> String {
    match state {
        1 => "CLOSED".to_string(),
        2 => "LISTEN".to_string(),
        3 => "SYN_SENT".to_string(),
        4 => "SYN_RCVD".to_string(),
        5 => "ESTABLISHED".to_string(),
        6 => "FIN_WAIT1".to_string(),
        7 => "FIN_WAIT2".to_string(),
        8 => "CLOSE_WAIT".to_string(),
        9 => "CLOSING".to_string(),
        10 => "LAST_ACK".to_string(),
        11 => "TIME_WAIT".to_string(),
        12 => "DELETE_TCB".to_string(),
        _ => format!("UNKNOWN({})", state),
    }
}
