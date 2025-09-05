import { ref, computed, watch, readonly, onUnmounted, nextTick } from 'vue'
import type { ConnectionInfo } from '~/plugins/tauri.client'
import { logger } from '~/utils/logger'



export interface SortConfig {
  column: string | null
  direction: 'asc' | 'desc'
}

// Fetch connections from Tauri backend
const fetchConnections = async (): Promise<ConnectionInfo[]> => {
  // Only show loading state if we don't have existing data (first load)
  logger.debug('Begin fetching connections...')
  let conns: ConnectionInfo[] = []
  try {
    const { $tauri } = useNuxtApp()
    conns = await $tauri.getConnections()
  } catch (err: unknown) {
    const error = err instanceof Error ? err.message : 'Failed to fetch connections'
    logger.error('Error fetching connections:', err)
    
    // Check if it's a lsof not found error
    if (error.includes('lsof command not found') || error.includes('Command not found')) {
      throw new Error('LSOF_NOT_FOUND: ' + error)
    }
    
    throw err
  } finally {
  }
  return conns
}

// Apply filters to connections
const applyFilters = (connections: ConnectionInfo[], filters): ConnectionInfo[] => {
  let filtered: ConnectionInfo[] = connections
  // Protocol filter
  if (filters.protocol !== 'all') {
    filtered = filtered.filter((conn: ConnectionInfo) =>
      conn.protocol.toLowerCase() === filters.protocol
    )
  }

  // Port filter (using string prefix matching)
  if (filters.port) {
    const portStr: string = filters.port.trim()
    if (portStr) {
      filtered = filtered.filter((conn: ConnectionInfo) => {
        const localMatch: boolean = conn.local_port.toString().startsWith(portStr)
        const remoteMatch: boolean = conn.remote_port.toString().startsWith(portStr)
        const result: boolean = localMatch || remoteMatch
        return result
      })
    }
  }

  // Process filter
  if (filters.process) {
    const processFilter: string = filters.process.toLowerCase()
    filtered = filtered.filter((conn: ConnectionInfo) =>
      conn.process_name.toLowerCase().includes(processFilter)
    )
  }

  // Debug: Log final filter results
  logger.debug(`🎯 Final filter results: ${connections.length} → ${filtered.length} connections`)
  return filtered
}

export default {
  fetchConnections,
  applyFilters
}
