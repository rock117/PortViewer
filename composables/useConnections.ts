import { ref, computed, watch, readonly, onUnmounted } from 'vue'
import type { ConnectionInfo } from '~/plugins/tauri.client'
import { logger } from '~/utils/logger'

export interface FilterState {
  protocol: 'all' | 'tcp' | 'udp'
  port: string
  process: string
}

export interface SortConfig {
  column: string | null
  direction: 'asc' | 'desc'
}

export const useConnections = () => {
  const connections = ref<ConnectionInfo[]>([])
  const filteredConnections = ref<ConnectionInfo[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const autoRefresh = ref(false)
  const refreshInterval = ref<NodeJS.Timeout | null>(null)
  const refreshIntervalSeconds = ref(5) // Default 5 seconds

  const filters = ref<FilterState>({
    protocol: 'all',
    port: '',
    process: ''
  })

  const sortConfig = ref<SortConfig>({
    column: null,
    direction: 'asc'
  })

  // Statistics computed properties
  const statistics = computed(() => {
    const stats: {
      total: number;
      tcp: number;
      udp: number;
      listening: number;
      established: number;
    } = {
      total: connections.value.length,
      tcp: 0,
      udp: 0,
      listening: 0,
      established: 0
    }

    connections.value.forEach((conn: ConnectionInfo) => {
      if (conn.protocol.toLowerCase() === 'tcp') stats.tcp++
      else if (conn.protocol.toLowerCase() === 'udp') stats.udp++

      if (conn.state.toLowerCase() === 'listening') stats.listening++
      else if (conn.state.toLowerCase() === 'established') stats.established++
    })

    return stats
  })

  // Fetch connections from Tauri backend
  const fetchConnections = async (): Promise<void> => {
    // Only show loading state if we don't have existing data (first load)
    if (connections.value.length === 0) {
      isLoading.value = true
    }
    
    try {
      error.value = null
      
      const { $tauri } = useNuxtApp()
      const data: ConnectionInfo[] = await $tauri.getConnections()
      
      // Smooth data update to prevent jitter
      connections.value = data
      applyFilters()
    } catch (err: unknown) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch connections'
      logger.error('Error fetching connections:', err)
    } finally {
      isLoading.value = false
    }
  }

  // Apply filters to connections
  const applyFilters = (): void => {
    let filtered: ConnectionInfo[] = [...connections.value]

    // Protocol filter
    if (filters.value.protocol !== 'all') {
      filtered = filtered.filter((conn: ConnectionInfo) => 
        conn.protocol.toLowerCase() === filters.value.protocol
      )
    }

    // Port filter (using string prefix matching)
    if (filters.value.port) {
      const portStr: string = filters.value.port.trim()
      logger.debug('🔍 Port filter search:', portStr)
      
      if (portStr) {
        const beforeCount: number = filtered.length
        filtered = filtered.filter((conn: ConnectionInfo) => {
          const localMatch: boolean = conn.local_port.toString().startsWith(portStr)
          const remoteMatch: boolean = conn.remote_port.toString().startsWith(portStr)
          const result: boolean = localMatch || remoteMatch
          
          // Debug specific cases
          if (result) {
            logger.debug(`✅ Match found: ${conn.local_port}/${conn.remote_port} matches "${portStr}"`, {
              local_port: conn.local_port,
              remote_port: conn.remote_port,
              localMatch,
              remoteMatch
            })
          }
          
          return result
        })
        
        logger.debug(`📊 Port filter: ${beforeCount} → ${filtered.length} connections`)
      }
    }

    // Process filter
    if (filters.value.process) {
      const processFilter: string = filters.value.process.toLowerCase()
      filtered = filtered.filter((conn: ConnectionInfo) =>
        conn.process_name.toLowerCase().includes(processFilter)
      )
    }

    // Apply sorting
    if (sortConfig.value.column) {
      filtered.sort((a: ConnectionInfo, b: ConnectionInfo) => {
        const aVal: any = getNestedValue(a, sortConfig.value.column!)
        const bVal: any = getNestedValue(b, sortConfig.value.column!)
        
        let comparison: number = 0
        if (typeof aVal === 'number' && typeof bVal === 'number') {
          comparison = aVal - bVal
        } else {
          comparison = String(aVal).localeCompare(String(bVal))
        }
        
        return sortConfig.value.direction === 'desc' ? -comparison : comparison
      })
    }

    filteredConnections.value = filtered
  }

  // Helper function to get nested object values
  const getNestedValue = (obj: any, path: string): any => {
    return path.split('.').reduce((current: any, key: string) => current?.[key], obj)
  }

  // Sort connections by column
  const sortBy = (column: string): void => {
    if (sortConfig.value.column === column) {
      sortConfig.value.direction = sortConfig.value.direction === 'asc' ? 'desc' : 'asc'
    } else {
      sortConfig.value.column = column
      sortConfig.value.direction = 'asc'
    }
    applyFilters()
  }

  // Update filters
  const updateFilter = (key: keyof FilterState, value: string): void => {
    filters.value[key] = value as any
    applyFilters()
  }

  // Auto refresh functionality
  const startAutoRefresh = (): void => {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value)
    }
    
    refreshInterval.value = setInterval((): void => {
      if (autoRefresh.value) {
        fetchConnections()
      }
    }, refreshIntervalSeconds.value * 1000) // Use configurable interval
  }

  const stopAutoRefresh = (): void => {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value)
      refreshInterval.value = null
    }
  }

  const toggleAutoRefresh = (): void => {
    autoRefresh.value = !autoRefresh.value
    if (autoRefresh.value) {
      startAutoRefresh()
    } else {
      stopAutoRefresh()
    }
  }

  const setRefreshInterval = (seconds: number): void => {
    refreshIntervalSeconds.value = seconds
    if (autoRefresh.value) {
      // Restart with new interval
      startAutoRefresh()
    }
  }

  // Cleanup on unmount
  onUnmounted(() => {
    stopAutoRefresh()
  })

  // Watch for filter changes
  watch(filters, () => {
    applyFilters()
  }, { deep: true })

  return {
    connections: readonly(connections),
    filteredConnections: readonly(filteredConnections),
    isLoading: readonly(isLoading),
    error: readonly(error),
    statistics,
    filters,
    sortConfig: readonly(sortConfig),
    autoRefresh,
    refreshIntervalSeconds: readonly(refreshIntervalSeconds),
    fetchConnections,
    sortBy,
    updateFilter,
    toggleAutoRefresh,
    setRefreshInterval,
    startAutoRefresh,
    stopAutoRefresh
  }
}
