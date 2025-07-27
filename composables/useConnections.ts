import { ref, computed, watch, onUnmounted, readonly } from 'vue'
import type { ConnectionInfo } from '~/plugins/tauri.client'

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
    const stats = {
      total: connections.value.length,
      tcp: 0,
      udp: 0,
      listening: 0,
      established: 0
    }

    connections.value.forEach(conn => {
      if (conn.protocol.toLowerCase() === 'tcp') stats.tcp++
      else if (conn.protocol.toLowerCase() === 'udp') stats.udp++

      if (conn.state.toLowerCase() === 'listening') stats.listening++
      else if (conn.state.toLowerCase() === 'established') stats.established++
    })

    return stats
  })

  // Fetch connections from Tauri backend
  const fetchConnections = async () => {
    try {
      isLoading.value = true
      error.value = null
      
      const { $tauri } = useNuxtApp()
      const data = await $tauri.getConnections()
      
      connections.value = data
      applyFilters()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch connections'
      console.error('Error fetching connections:', err)
    } finally {
      isLoading.value = false
    }
  }

  // Apply filters to connections
  const applyFilters = () => {
    let filtered = [...connections.value]

    // Protocol filter
    if (filters.value.protocol !== 'all') {
      filtered = filtered.filter(conn => 
        conn.protocol.toLowerCase() === filters.value.protocol
      )
    }

    // Port filter (using string prefix matching)
    if (filters.value.port) {
      const portStr = filters.value.port.trim()
      if (portStr) {
        filtered = filtered.filter(conn => 
          conn.local_port.toString().startsWith(portStr) || 
          conn.remote_port.toString().startsWith(portStr)
        )
      }
    }

    // Process filter
    if (filters.value.process) {
      const processFilter = filters.value.process.toLowerCase()
      filtered = filtered.filter(conn =>
        conn.process_name.toLowerCase().includes(processFilter)
      )
    }

    // Apply sorting
    if (sortConfig.value.column) {
      filtered.sort((a, b) => {
        const aVal = getNestedValue(a, sortConfig.value.column!)
        const bVal = getNestedValue(b, sortConfig.value.column!)
        
        let comparison = 0
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
  const getNestedValue = (obj: any, path: string) => {
    return path.split('.').reduce((current, key) => current?.[key], obj)
  }

  // Sort connections by column
  const sortBy = (column: string) => {
    if (sortConfig.value.column === column) {
      sortConfig.value.direction = sortConfig.value.direction === 'asc' ? 'desc' : 'asc'
    } else {
      sortConfig.value.column = column
      sortConfig.value.direction = 'asc'
    }
    applyFilters()
  }

  // Update filters
  const updateFilter = (key: keyof FilterState, value: string) => {
    filters.value[key] = value as any
    applyFilters()
  }

  // Auto refresh functionality
  const startAutoRefresh = () => {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value)
    }
    
    refreshInterval.value = setInterval(() => {
      if (autoRefresh.value) {
        fetchConnections()
      }
    }, refreshIntervalSeconds.value * 1000) // Use configurable interval
  }

  const stopAutoRefresh = () => {
    if (refreshInterval.value) {
      clearInterval(refreshInterval.value)
      refreshInterval.value = null
    }
  }

  const toggleAutoRefresh = () => {
    autoRefresh.value = !autoRefresh.value
    if (autoRefresh.value) {
      startAutoRefresh()
    } else {
      stopAutoRefresh()
    }
  }

  const setRefreshInterval = (seconds: number) => {
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
