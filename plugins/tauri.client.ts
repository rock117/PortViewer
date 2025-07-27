export interface ConnectionInfo {
  protocol: string
  local_address: string
  local_port: number
  remote_address: string
  remote_port: number
  state: string
  pid: number
  process_name: string
}

// Check if we're running in Tauri environment
const isTauri = () => {
  return typeof window !== 'undefined' && 
         window.__TAURI__ !== undefined
}

// Mock data for browser development
const mockConnections: ConnectionInfo[] = [
  {
    protocol: 'TCP',
    local_address: '127.0.0.1',
    local_port: 8080,
    remote_address: '192.168.1.100',
    remote_port: 54321,
    state: 'ESTABLISHED',
    pid: 1234,
    process_name: 'chrome.exe'
  },
  {
    protocol: 'TCP',
    local_address: '0.0.0.0',
    local_port: 80,
    remote_address: '',
    remote_port: 0,
    state: 'LISTENING',
    pid: 4,
    process_name: 'System'
  },
  {
    protocol: 'UDP',
    local_address: '127.0.0.1',
    local_port: 53,
    remote_address: '',
    remote_port: 0,
    state: 'LISTENING',
    pid: 2048,
    process_name: 'dns.exe'
  }
]

export class TauriAPI {
  static async getConnections(): Promise<ConnectionInfo[]> {
    if (!isTauri()) {
      // Return mock data in browser mode
      console.log('Running in browser mode, returning mock data')
      return Promise.resolve(mockConnections)
    }

    try {
      const { invoke } = await import('@tauri-apps/api/core')
      return await invoke('get_connections')
    } catch (error) {
      console.error('Failed to get connections:', error)
      throw error
    }
  }

  static async getFilteredConnections(protocol?: string, port?: number): Promise<ConnectionInfo[]> {
    if (!isTauri()) {
      // Return filtered mock data in browser mode
      console.log('Running in browser mode, returning filtered mock data')
      let filtered = [...mockConnections]
      
      if (protocol && protocol !== 'all') {
        filtered = filtered.filter(conn => 
          conn.protocol.toLowerCase() === protocol.toLowerCase()
        )
      }
      
      if (port) {
        filtered = filtered.filter(conn => 
          conn.local_port === port || conn.remote_port === port
        )
      }
      
      return Promise.resolve(filtered)
    }

    try {
      const { invoke } = await import('@tauri-apps/api/core')
      return await invoke('get_filtered_connections', { protocol, port })
    } catch (error) {
      console.error('Failed to get filtered connections:', error)
      throw error
    }
  }
}

export default defineNuxtPlugin(() => {
  return {
    provide: {
      tauri: TauriAPI
    }
  }
})
