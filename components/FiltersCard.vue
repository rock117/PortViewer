<template>
  <div class="card mb-6">
    <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
      <!-- Filter Controls -->
      <div class="flex flex-col md:flex-row gap-4 flex-1">
        <!-- Protocol Filter -->
        <div class="flex flex-col">
          <label class="text-sm font-medium text-gray-700 mb-1">Protocol</label>
          <select 
            :value="filters.protocol" 
            @change="updateFilter('protocol', ($event.target as HTMLSelectElement).value)"
            class="input-field"
          >
            <option value="all">All</option>
            <option value="tcp">TCP</option>
            <option value="udp">UDP</option>
          </select>
        </div>

        <!-- Port Filter -->
        <div class="flex flex-col">
          <label class="text-sm font-medium text-gray-700 mb-1">Port</label>
          <input 
            type="text" 
            :value="filters.port"
            @input="updateFilter('port', ($event.target as HTMLInputElement).value)"
            placeholder="Enter port number"
            class="input-field"
          />
        </div>

        <!-- Process Filter -->
        <div class="flex flex-col">
          <label class="text-sm font-medium text-gray-700 mb-1">Process</label>
          <input 
            type="text" 
            :value="filters.process"
            @input="updateFilter('process', ($event.target as HTMLInputElement).value)"
            placeholder="Enter process name"
            class="input-field"
          />
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-3">
        <!-- Auto Refresh Toggle -->
        <div class="flex items-center">
          <input 
            type="checkbox" 
            id="autoRefresh"
            :checked="autoRefresh"
            @change="toggleAutoRefresh"
            class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
          />
          <label for="autoRefresh" class="ml-2 text-sm text-gray-700">
            Auto Refresh
          </label>
        </div>

        <!-- Refresh Button -->
        <button 
          @click="fetchConnections" 
          :disabled="isLoading"
          class="btn-primary flex items-center"
        >
          <svg 
            class="w-4 h-4 mr-2"
            :class="{ 'animate-spin': isLoading }"
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          {{ isLoading ? 'Loading...' : 'Refresh' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  filters: {
    protocol: string
    port: string
    process: string
  }
  autoRefresh: boolean
  isLoading: boolean
  updateFilter: (key: string, value: string) => void
  toggleAutoRefresh: () => void
  fetchConnections: () => void
}

defineProps<Props>()
</script>
