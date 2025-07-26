# Windows Port Viewer

A powerful command-line tool for viewing network port usage on Windows systems, built with Rust.

## Features

- **Comprehensive Port Monitoring**: Display all TCP and UDP connections with detailed information
- **Process Name Resolution**: Shows the executable name for each process using network ports
- **Protocol Filtering**: Filter connections by protocol type (TCP, UDP, or both)
- **Port Filtering**: Filter connections by specific port numbers
- **Real-time Information**: Shows current connection states and process details
- **Clean Output Format**: Well-formatted table output for easy reading

## Installation

### Prerequisites

- Windows operating system
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Building from Source

1. Clone or download this repository
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

4. The executable will be available at `target/release/windows-tool.exe`

## Usage

### Basic Usage

Display all network connections:
```bash
cargo run
```

### Command Line Options

```bash
cargo run -- [OPTIONS]
```

**Options:**
- `-p, --protocol <PROTOCOL>`: Specify protocol type (tcp, udp, all) [default: all]
- `-P, --port <PORT>`: Filter by specific port number
- `-h, --help`: Print help information

### Examples

**Show all connections:**
```bash
cargo run
```

**Show only TCP connections:**
```bash
cargo run -- --protocol tcp
```

**Show only UDP connections:**
```bash
cargo run -- --protocol udp
```

**Filter by specific port (e.g., MySQL port 3306):**
```bash
cargo run -- --port 3306
```

**Show TCP connections on port 80:**
```bash
cargo run -- --protocol tcp --port 80
```

## Output Format

The tool displays network connections in a table format with the following columns:

| Column | Description |
|--------|-------------|
| Protocol | Connection protocol (TCP/UDP) |
| Local Address | Local IP address and port |
| Remote Address | Remote IP address and port (or *:* for listening ports) |
| State | Connection state (LISTEN, ESTABLISHED, etc.) |
| PID | Process ID using the port |
| Process Name | Executable name of the process |

### Example Output

```
Windows Port Usage:
Protocol Local Address          Remote Address         State        PID      Process Name        
--------------------------------------------------------------------------------------------------------------
TCP      0.0.0.0:135            *:*                    LISTEN       1908     svchost.exe         
TCP      127.0.0.1:3306         *:*                    LISTEN       9684     mysqld.exe          
TCP      192.168.1.100:443      74.125.224.108:443     ESTABLISHED  5432     chrome.exe          
UDP      0.0.0.0:53             *:*                    LISTENING    5040     dns.exe             
UDP      127.0.0.1:1900         *:*                    LISTENING    20704    svchost.exe         
```

## Technical Details

### Windows APIs Used

- **GetExtendedTcpTable**: Retrieves TCP connection information with process IDs
- **GetExtendedUdpTable**: Retrieves UDP connection information with process IDs
- **OpenProcess**: Opens process handles for querying process information
- **QueryFullProcessImageNameW**: Gets the full path of process executables

### Dependencies

- `windows`: Windows API bindings for Rust
- `clap`: Command-line argument parsing

### Connection States

The tool displays various TCP connection states:

- **LISTEN**: Port is listening for incoming connections
- **ESTABLISHED**: Active connection established
- **SYN_SENT**: Connection request sent
- **SYN_RCVD**: Connection request received
- **FIN_WAIT1/FIN_WAIT2**: Connection termination in progress
- **CLOSE_WAIT**: Waiting for connection termination
- **TIME_WAIT**: Waiting for network to clear duplicate packets
- **CLOSED**: Connection closed

## Permissions

Some processes may show as `<Unknown>` in the Process Name column. This typically occurs when:

- The process requires elevated privileges to access
- The process has terminated but the connection is still being cleaned up
- System-level processes that restrict access

To see more process names, consider running the tool with administrator privileges.

## Use Cases

- **Network Debugging**: Identify which processes are using specific ports
- **Security Monitoring**: Monitor for unexpected network connections
- **System Administration**: Manage services and applications using network resources
- **Development**: Debug network-related issues in applications
- **Performance Analysis**: Identify network-intensive processes

## Troubleshooting

### Common Issues

**"Access Denied" or `<Unknown>` Process Names:**
- Run the tool as Administrator for better process name resolution

**No Output or Empty Results:**
- Ensure Windows Firewall is not blocking the application
- Check if you have necessary permissions to query network information

**Build Errors:**
- Ensure you have the latest Rust toolchain installed
- Verify Windows SDK is available for the `windows` crate

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

## License

This project is open source. Please refer to the license file for details.

## Changelog

### Version 1.0.0
- Initial release with TCP/UDP connection monitoring
- Process name resolution
- Protocol and port filtering
- Clean table output format
