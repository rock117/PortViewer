import { logger } from '../utils/logger'

export interface ConnectionInfo {
  id: string
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
const isTauri = (): boolean => {
  // Check for Tauri API availability
  const hasWindow: boolean = typeof window !== 'undefined'
  const hasTauriAPI: boolean = hasWindow && (window as any).__TAURI__ !== undefined
  const hasTauriInvoke: boolean = hasWindow && (window as any).__TAURI__?.core?.invoke !== undefined
  
  // Also check for Tauri-specific user agent or other indicators
  const userAgent: string = hasWindow ? navigator.userAgent : ''
  const isTauriUserAgent: boolean = userAgent.includes('Tauri') || userAgent.includes('tauri')
  
  const isInTauri: boolean = hasTauriAPI && hasTauriInvoke
  
  logger.debug('Tauri environment detection:', {
    hasWindow,
    hasTauriAPI,
    hasTauriInvoke,
    isTauriUserAgent,
    userAgent,
    isInTauri,
    finalDecision: isInTauri ? 'Using Tauri backend' : 'Using mock data'
  })
  
  return isInTauri
}

// Generate unique ID for connections using UUID v4
const generateConnectionId = (): string => {
  // Simple UUID v4 implementation for browser compatibility
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0
    const v = c == 'x' ? r : (r & 0x3 | 0x8)
    return v.toString(16)
  })
}

// Mock data for browser development
const mockConnections: ConnectionInfo[] = [
  // TCP Connections
  {
    id: generateConnectionId(),
    protocol: 'TCP',
    local_address: '192.168.1.50',
    local_port: 22,
    remote_address: '192.168.1.100',
    remote_port: 54322,
    state: 'ESTABLISHED',
    pid: 9012,
    process_name: 'sshd.exe'
  },
  {
    id: generateConnectionId(),
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
    id: generateConnectionId(),
    protocol: 'TCP',
    local_address: '0.0.0.0',
    local_port: 80,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 4,
    process_name: 'System'
  },
  {
    id: generateConnectionId(),
    protocol: 'TCP',
    local_address: '0.0.0.0',
    local_port: 443,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 4,
    process_name: 'System'
  },
  {
    id: generateConnectionId(),
    protocol: 'TCP',
    local_address: '127.0.0.1',
    local_port: 3000,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 5678,
    process_name: 'node.exe'
  },
  {
    id: generateConnectionId(),
    protocol: 'TCP',
    local_address: '192.168.1.10',
    local_port: 49152,
    remote_address: '142.250.191.14',
    remote_port: 443,
    state: 'ESTABLISHED',
    pid: 1234,
    process_name: 'chrome.exe'
  },
  {
    id: generateConnectionId(),
    protocol: 'TCP',
    local_address: '127.0.0.1',
    local_port: 1420,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 9876,
    process_name: 'nuxt.exe'
  },
  // UDP Connections
  {
    id: generateConnectionId(),
    protocol: 'UDP',
    local_address: '127.0.0.1',
    local_port: 53,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 2048,
    process_name: 'dns.exe'
  },
  {
    id: generateConnectionId(),
    protocol: 'UDP',
    local_address: '0.0.0.0',
    local_port: 67,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 1024,
    process_name: 'dhcp.exe'
  },
  {
    id: generateConnectionId(),
    protocol: 'UDP',
    local_address: '192.168.1.10',
    local_port: 137,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 4,
    process_name: 'System'
  },
  {
    id: generateConnectionId(),
    protocol: 'UDP',
    local_address: '192.168.1.10',
    local_port: 138,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 4,
    process_name: 'System'
  },
  {
    id: generateConnectionId(),
    protocol: 'UDP',
    local_address: '0.0.0.0',
    local_port: 5353,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 3456,
    process_name: 'mdnsresponder.exe'
  },
  {
    id: generateConnectionId(),
    protocol: 'UDP',
    local_address: '127.0.0.1',
    local_port: 1900,
    remote_address: '*',
    remote_port: 0,
    state: 'LISTENING',
    pid: 7890,
    process_name: 'svchost.exe'
  }
]

export class TauriAPI {
  static async getConnections(): Promise<ConnectionInfo[]> {
    logger.debug('TauriAPI.getConnections() called')
    
    // Always try Tauri first, regardless of environment detection
    try {
      logger.debug('Attempting to import Tauri API...')
      const { invoke } = await import('@tauri-apps/api/core')
      logger.debug('Tauri API imported successfully, calling get_connections...')
      const result: ConnectionInfo[] = await invoke('get_connections') as ConnectionInfo[]
      logger.log('Tauri backend returned:', result.length + ' connections')
      return result
    } catch (error: unknown) {
      logger.error('Tauri API failed, falling back to mock data. Error:', error)
      
      // Only use mock data if Tauri is completely unavailable
      if (!isTauri()) {
        logger.debug('Environment detection confirms browser mode, using mock data')
        return Promise.resolve(mockConnections)
      }
      
      // If we're in Tauri but invoke failed, still try mock data as fallback
      logger.debug('In Tauri environment but invoke failed, using mock data as fallback')
      return Promise.resolve(mockConnections)
    }
  }
  
  // Keep the old logic as a separate method for comparison
  static async getConnectionsOld(): Promise<ConnectionInfo[]> {
    if (!isTauri()) {
      // Return mock data in browser mode
      logger.debug('Running in browser mode, returning mock data')
      return Promise.resolve(mockConnections)
    }

    try {
      const { invoke } = await import('@tauri-apps/api/core')
      return await invoke('get_connections') as ConnectionInfo[]
    } catch (error: unknown) {
      logger.error('Failed to get connections:', error)
      throw error
    }
  }

  static async getFilteredConnections(protocol?: string, port?: number): Promise<ConnectionInfo[]> {
    if (!isTauri()) {
      // Return filtered mock data in browser mode
      logger.debug('Running in browser mode, returning filtered mock data')
      let filtered: ConnectionInfo[] = [...mockConnections]
      
      if (protocol && protocol !== 'all') {
        filtered = filtered.filter((conn: ConnectionInfo) => 
          conn.protocol.toLowerCase() === protocol.toLowerCase()
        )
      }
      
      if (port) {
        filtered = filtered.filter((conn: ConnectionInfo) => 
          conn.local_port === port || conn.remote_port === port
        )
      }
      
      return Promise.resolve(filtered)
    }

    try {
      const { invoke } = await import('@tauri-apps/api/core')
      return await invoke('get_filtered_connections', { protocol, port }) as ConnectionInfo[]
    } catch (error: unknown) {
      logger.error('Failed to get filtered connections:', error)
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
