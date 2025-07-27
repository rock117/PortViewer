use std::ffi::c_void;
use windows::Win32::Foundation::FALSE;
use windows::Win32::NetworkManagement::IpHelper::{GetExtendedTcpTable, GetExtendedUdpTable, MIB_TCPROW_OWNER_PID, MIB_UDPROW_OWNER_PID, TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID};
use windows::Win32::Networking::WinSock::AF_INET;

use crate::models::{ConnectionInfo, Protocol, ConnectionState};
use crate::process::{get_process_name, format_ip_address};

pub fn get_all_connections() -> Vec<ConnectionInfo> {
    let mut connections = Vec::new();
    
    // Get TCP connections
    connections.extend(get_tcp_connections());
    
    // Get UDP connections
    connections.extend(get_udp_connections());
    
    connections
}

pub fn get_tcp_connections() -> Vec<ConnectionInfo> {
    let mut connections = Vec::new();
    println!("[DEBUG] Starting TCP connection retrieval...");
    
    unsafe {
        let mut size = 0u32;
        
        // First get the required buffer size
        let initial_result = GetExtendedTcpTable(
            None,
            &mut size,
            FALSE,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
        
        println!("[DEBUG] TCP table size query result: {}, required size: {} bytes", initial_result, size);

        if size == 0 {
            println!("[DEBUG] TCP table size is 0, returning empty connections");
            return connections;
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
            eprintln!("[ERROR] Failed to obtain TCP connection table: {}", result);
            return connections;
        }

        let table_ptr = buffer.as_ptr() as *const u32;
        let num_entries = *table_ptr;
        println!("[DEBUG] TCP table retrieved successfully, found {} entries", num_entries);
        
        // Skip the dwNumEntries field and get to the actual entries
        let entries_ptr = table_ptr.add(1) as *const MIB_TCPROW_OWNER_PID;
        
        for i in 0..num_entries {
            let entry = &*entries_ptr.add(i as usize);
            
            let local_addr = format_ip_address(entry.dwLocalAddr);
            let local_port = u16::from_be(entry.dwLocalPort as u16);
            let remote_addr = format_ip_address(entry.dwRemoteAddr);
            let remote_port = u16::from_be(entry.dwRemotePort as u16);
            let state = ConnectionState::from(entry.dwState);
            let pid = entry.dwOwningPid;
            let process_name = get_process_name(pid);

            connections.push(ConnectionInfo::new(
                Protocol::TCP,
                local_addr,
                local_port,
                remote_addr,
                remote_port,
                state,
                pid,
                process_name,
            ));
        }
    }
    
    connections
}

pub fn get_udp_connections() -> Vec<ConnectionInfo> {
    let mut connections = Vec::new();
    
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
            return connections;
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
            return connections;
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

            connections.push(ConnectionInfo::new(
                Protocol::UDP,
                local_addr,
                local_port,
                "*".to_string(),
                0,
                ConnectionState::Listening,
                pid,
                process_name,
            ));
        }
    }
    
    connections
}
