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
        :toggle-auto-refresh="toggleAutoRefresh"
        :set-refresh-interval="setRefreshInterval"
        :fetch-connections="fetchConnections"
      />

        <!-- Connections Table -->
        <div class="flex-1 min-h-0">
          <ConnectionsTable 
            :connections="connections"
            :filtered-connections="filteredConnections"
            :is-loading="isLoading"
            :error="error"
            :sort-config="sortConfig"
            :fetch-connections="fetchConnections"
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
// Set page meta
useHead({
  title: 'Windows Port Viewer',
  meta: [
    { name: 'description', content: 'Monitor Windows TCP/UDP port usage with process information' }
  ]
})

// Use connections composable
const {
  connections,
  filteredConnections,
  isLoading,
  error,
  statistics,
  filters,
  sortConfig,
  autoRefresh,
  refreshIntervalSeconds,
  fetchConnections,
  sortBy,
  updateFilter,
  toggleAutoRefresh,
  setRefreshInterval,
  startAutoRefresh
} = useConnections()

// Last updated timestamp
const lastUpdated = ref('')

// Update timestamp when connections are fetched
watch(connections, () => {
  lastUpdated.value = new Date().toLocaleTimeString()
})

// Keyboard shortcuts
const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'F5') {
    event.preventDefault()
    fetchConnections()
  }
}

// Initialize on mount
onMounted(async () => {
  // Add keyboard event listener
  window.addEventListener('keydown', handleKeydown)
  
  // Initial data fetch
  await fetchConnections()
  
  // Start auto refresh
  startAutoRefresh()
})

// Cleanup on unmount
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>
