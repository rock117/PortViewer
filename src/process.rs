use windows::Win32::Foundation::{CloseHandle, FALSE};
use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub fn get_process_name(pid: u32) -> String {
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

pub fn format_ip_address(addr: u32) -> String {
    let bytes = addr.to_le_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}


