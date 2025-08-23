<template>
  <div class="h-screen bg-gray-50 flex flex-col overflow-hidden">
    <!-- Header -->
    <header class="bg-white shadow-sm border-b border-gray-200 flex-shrink-0">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <h1 class="text-2xl font-bold text-gray-900">
                Port Viewer
              </h1>
            </div>
            <div class="ml-4">
              <p class="text-sm text-gray-600">Monitor TCP/UDP port usage and process information</p>
              <p v-if="platformInfo" class="text-xs text-gray-500">
                {{ platformInfo.os }}/{{ platformInfo.architecture }} 
                <span :class="platformInfo.supported ? 'text-green-600' : 'text-red-600'">
                  {{ platformInfo.supported ? '✓ Supported' : '✗ Not Supported' }}
                </span>
              </p>
            </div>
          </div>
          
          <!-- Status Indicator -->
          <div class="flex items-center space-x-2">
            <div class="flex items-center">
              <div 
                class="w-2 h-2 rounded-full mr-2"
                :class="isLoading ? 'bg-yellow-400' : error ? 'bg-red-400' : 'bg-green-400'"
              ></div>
              <span class="text-sm text-gray-600">
                {{ isLoading ? 'Loading...' : error ? 'Error' : 'Connected' }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 overflow-hidden">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 h-full flex flex-col space-y-6">
      <!-- Statistics Cards -->
      <StatisticsCard :statistics="statistics" />

      <!-- Filters -->
      <FiltersCard 
        :filters="filters"
        :auto-refresh="autoRefresh"
        :refresh-interval-seconds="refreshIntervalSeconds"
        :is-loading="isLoading"
        :update-filter="updateFilter"
        :refresh-connections="refreshConnections"
        :toggle-auto-refresh="toggleAutoRefresh"
        :set-refresh-interval="setRefreshInterval"
      />

        <!-- Connections Table -->
        <div class="flex-1 min-h-0">
          <ConnectionsTable 
            :connections="allConnections"
            :filtered-connections="filteredConnections"
            :is-loading="isLoading"
            :error="error"
            :sort-config="sortConfig"
            :sort-by="sortBy"
          />
        </div>
      </div>
    </main>

    <!-- Footer -->
    <footer class="bg-white border-t border-gray-200 flex-shrink-0">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex items-center justify-between">
          <p class="text-sm text-gray-600">
            Port Viewer - Built with Nuxt 3 and Tauri 2
          </p>
          <div class="flex items-center space-x-4 text-sm text-gray-500">
            <span>Last updated: {{ lastUpdated }}</span>
            <kbd class="px-2 py-1 bg-gray-100 rounded text-xs">F5</kbd>
            <span>Refresh</span>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { logger } from '~/utils/logger'
import type { ConnectionInfo } from '../plugins/tauri.client'
export interface FilterState {
  protocol: 'all' | 'tcp' | 'udp'
  port: string
  process: string
}

// Set page meta
useHead({
  title: 'Port Viewer',
  meta: [
    { name: 'description', content: 'Monitor Windows TCP/UDP port usage with process information' }
  ]
})
const filters = ref<FilterState>({
  protocol: 'all',
  port: '',
  process: ''
})
// State management
const isLoading = ref(false)
const error = ref<string | null>(null)
const autoRefresh = ref(false)
const refreshInterval = ref<NodeJS.Timeout | null>(null)
const platformInfo = ref<any>(null)
const sortConfig = ref({
  column: null,
  direction: 'asc'
})
const sortBy = ref({
  column: null,
  direction: 'asc'
})

let internalFetchConnectionId: NodeJS.Timeout | null = null
const allConnections = ref<ConnectionInfo[]>([])
const filteredConnections = ref<ConnectionInfo[]>([])
const refreshIntervalSeconds = ref(5)

// Filter connections based on current filters
const applyFilters = (connections: ConnectionInfo[], filters: FilterState): ConnectionInfo[] => {
  return connections.filter(conn => {
    // Protocol filter
    if (filters.protocol !== 'all' && conn.protocol.toLowerCase() !== filters.protocol) {
      return false
    }
    
    // Port filter
    if (filters.port && !conn.local_port.toString().includes(filters.port) && !conn.remote_port.toString().includes(filters.port)) {
      return false
    }
    
    // Process filter
    if (filters.process && !conn.process_name.toLowerCase().includes(filters.process.toLowerCase())) {
      return false
    }
    
    return true
  })
}

const statistics = computed(() => {
  const tcp = allConnections.value.filter(conn => conn.protocol.toLowerCase() === 'tcp').length
  const udp = allConnections.value.filter(conn => conn.protocol.toLowerCase() === 'udp').length
  const listening = allConnections.value.filter(conn => conn.state.toLowerCase() === 'listen').length
  const established = allConnections.value.filter(conn => conn.state.toLowerCase() === 'established').length
  
  return {
    total: allConnections.value.length,
    filtered: filteredConnections.value.length,
    tcp,
    udp,
    listening,
    established
  }
})
// Last updated timestamp
const lastUpdated = ref('')

watch(filters, () => {
  logger.debug('🔄 Filters changed, updating filter-connections, filters:', filters.value)
  updateFilterConnections()
}, { deep: true })


// Update timestamp when connections are fetched
watch(allConnections, () => {
  lastUpdated.value = new Date().toLocaleTimeString()
  updateFilterConnections()
})

// Keyboard shortcuts
const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'F5') {
    event.preventDefault()
    fetchConnections()
  }
}

const updateFilter = (key: keyof FilterState, value: string): void => {
    filters.value[key] = value as any
}

const updateFilterConnections = () => {
  logger.debug(`🔄 Filter connections begin update, filtered-connections num, ${filteredConnections.value.length}`)
  filteredConnections.value = applyFilters(allConnections.value, filters.value)
  logger.debug(`🔄 Filter connections complete update, filtered-connections num, ${filteredConnections.value.length}, filters = ${JSON.stringify(filters.value)}`)
}

// Fetch connections from Tauri backend
const fetchConnections = async (): Promise<ConnectionInfo[]> => {
  try {
    isLoading.value = true
    error.value = null
    
    const { invoke } = await import('@tauri-apps/api/core')
    const connections = await invoke('get_connections') as ConnectionInfo[]
    
    logger.debug('✅ Fetched connections:', connections.length)
    return connections
  } catch (err) {
    logger.error('❌ Failed to fetch connections:', err)
    error.value = 'Failed to fetch connections'
    return []
  } finally {
    isLoading.value = false
  }
}

const refreshConnections = async () => {
  logger.debug('🔄 Refresh connections begin')
  allConnections.value = await fetchConnections()
  updateFilterConnections()
  logger.debug('🔄 Refresh connections complete')
}

const toggleAutoRefresh = () => {
  autoRefresh.value = !autoRefresh.value
}

const setRefreshInterval = (seconds: number) => {
  refreshIntervalSeconds.value = seconds
}

 

watch(autoRefresh, () => {
  if (autoRefresh.value) {
    internalFetchConnectionId = setInterval(async () => {
      await refreshConnections()
    }, refreshIntervalSeconds.value * 1000)
  } else {
    clearInterval(internalFetchConnectionId)
    internalFetchConnectionId = null
  }
}, { immediate: true })


// Fetch platform information
const fetchPlatformInfo = async () => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    platformInfo.value = await invoke('get_platform_info')
    logger.debug('Platform info:', platformInfo.value)
  } catch (err) {
    logger.error('Failed to fetch platform info:', err)
  }
}

// Initialize on mount
onMounted(async () => {
  // Add keyboard event listener
  window.addEventListener('keydown', handleKeydown)
  
  // Fetch platform info and connections
  await fetchPlatformInfo()
  allConnections.value = await fetchConnections()
  updateFilterConnections()
})

// Cleanup on unmount
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

</script>
