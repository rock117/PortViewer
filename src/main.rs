use std::ffi::c_void;
use windows::Win32::NetworkManagement::IpHelper::*;
use windows::Win32::Foundation::*;
use windows::Win32::Networking::WinSock::*;
use clap::{Arg, Command};

#[derive(Debug)]
struct ConnectionInfo {
    protocol: String,
    local_address: String,
    local_port: u16,
    remote_address: String,
    remote_port: u16,
    state: String,
    pid: u32,
}

fn main() {
    let matches = Command::new("windows-tool")
        .about("Windows端口占用查看工具")
        .arg(
            Arg::new("protocol")
                .short('p')
                .long("protocol")
                .value_name("PROTOCOL")
                .help("指定协议类型 (tcp, udp, all)")
                .default_value("all")
        )
        .arg(
            Arg::new("port")
                .short('P')
                .long("port")
                .value_name("PORT")
                .help("过滤指定端口")
        )
        .get_matches();

    let protocol = matches.get_one::<String>("protocol").unwrap();
    let filter_port = matches.get_one::<String>("port").map(|s| s.parse::<u16>().ok()).flatten();

    println!("Windows 端口占用情况:");
    println!("{:<8} {:<22} {:<22} {:<12} {:<8}", "协议", "本地地址", "远程地址", "状态", "PID");
    println!("{}", "-".repeat(80));

    match protocol.as_str() {
        "tcp" => list_tcp_connections(filter_port),
        "udp" => list_udp_connections(filter_port),
        "all" => {
            list_tcp_connections(filter_port);
            list_udp_connections(filter_port);
        }
        _ => {
            eprintln!("不支持的协议类型: {}", protocol);
            std::process::exit(1);
        }
    }
}

fn list_tcp_connections(filter_port: Option<u16>) {
    unsafe {
        let mut size = 0u32;
        
        // 首先获取所需的缓冲区大小
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

        // 分配缓冲区
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
            eprintln!("获取TCP连接表失败: {}", result);
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

            // 应用端口过滤
            if let Some(port) = filter_port {
                if local_port != port && remote_port != port {
                    continue;
                }
            }

            println!(
                "{:<8} {:<22} {:<22} {:<12} {:<8}",
                "TCP",
                format!("{}:{}", local_addr, local_port),
                if remote_port == 0 { "*:*".to_string() } else { format!("{}:{}", remote_addr, remote_port) },
                state,
                pid
            );
        }
    }
}

fn list_udp_connections(filter_port: Option<u16>) {
    unsafe {
        let mut size = 0u32;
        
        // 首先获取所需的缓冲区大小
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

        // 分配缓冲区
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
            eprintln!("获取UDP连接表失败: {}", result);
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

            // 应用端口过滤
            if let Some(port) = filter_port {
                if local_port != port {
                    continue;
                }
            }

            println!(
                "{:<8} {:<22} {:<22} {:<12} {:<8}",
                "UDP",
                format!("{}:{}", local_addr, local_port),
                "*:*",
                "LISTENING",
                pid
            );
        }
    }
}

fn format_ip_address(addr: u32) -> String {
    let bytes = addr.to_le_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
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
