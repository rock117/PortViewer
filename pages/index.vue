<template>
  <div class="h-screen bg-gray-50 flex flex-col overflow-hidden">
    <!-- Header -->
    <header class="bg-white shadow-sm border-b border-gray-200 flex-shrink-0">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <h1 class="text-2xl font-bold text-gray-900">Windows Port Viewer</h1>
            </div>
            <div class="ml-4">
              <p class="text-sm text-gray-600">Monitor TCP/UDP port usage and process information</p>
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
            Windows Port Viewer - Built with Nuxt 3 and Tauri 2
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
import useConnections from '~/composables/useConnections'
import { logger } from '~/utils/logger'
export interface FilterState {
  protocol: 'all' | 'tcp' | 'udp'
  port: string
  process: string
}

// Set page meta
useHead({
  title: 'Windows Port Viewer',
  meta: [
    { name: 'description', content: 'Monitor Windows TCP/UDP port usage with process information' }
  ]
})
const filters = ref({
  protocol: 'all',
  port: '',
  process: ''
})
// Use connections composable
const {
  fetchConnections,
  applyFilters
} = useConnections

const isLoading = ref(false)
const error = ref(null)
const autoRefresh = ref(false)
const refreshIntervalSeconds = ref(5)
const sortConfig = ref({
  column: null,
  direction: 'asc'
})
const sortBy = ref({
  column: null,
  direction: 'asc'
})


const allConnections = ref([])
const filteredConnections = ref([])
const statistics = computed(() => {
  return  {
      total: allConnections.value.length,
      tcp: 0,
      udp: 0,
      listening: 0,
      established: 0
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


// Initialize on mount
onMounted(async () => {
  // Add keyboard event listener
  window.addEventListener('keydown', handleKeydown)
  
  allConnections.value = await fetchConnections()
  updateFilterConnections()
})

// Cleanup on unmount
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

</script>
