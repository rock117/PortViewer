<template>
  <div class="bg-white rounded-lg shadow-md overflow-hidden flex flex-col h-full">
    <!-- Table Header - Fixed -->
    <div class="px-6 py-4 bg-gray-50 border-b border-gray-200 flex-shrink-0">
      <h3 class="text-lg font-semibold text-gray-900">Active Connections</h3>
      <p class="text-sm text-gray-600 mt-1">
        Showing {{ filteredConnections.length }} of {{ connections.length }} connections
      </p>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="flex-1 overflow-hidden">
      <div class="overflow-auto h-full">
        <table class="min-w-full divide-y divide-gray-200 table-fixed">
        <colgroup>
          <col class="w-20"> <!-- Protocol -->
          <col class="w-32"> <!-- Local Address -->
          <col class="w-20"> <!-- Local Port -->
          <col class="w-32"> <!-- Remote Address -->
          <col class="w-20"> <!-- Remote Port -->
          <col class="w-24"> <!-- State -->
          <col class="w-16"> <!-- PID -->
          <col class="w-auto"> <!-- Process -->
        </colgroup>
        <thead class="table-header">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Protocol</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Local Address</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Local Port</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Remote Address</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Remote Port</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">State</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">PID</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Process</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <!-- Skeleton rows -->
          <tr v-for="n in 8" :key="n" class="animate-pulse">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-12"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-24"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-16"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-24"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-16"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-6 bg-gray-200 rounded-full w-20"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-12"></div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="h-4 bg-gray-200 rounded w-32"></div>
            </td>
          </tr>
        </tbody>
        </table>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="flex-1 flex items-center justify-center px-6 py-12">
      <div class="text-red-600 mb-2">
        <svg class="w-12 h-12 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
        </svg>
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Error Loading Connections</h3>
      <p class="text-gray-600 mb-4">{{ error }}</p>
      <button @click="fetchConnections" class="btn-primary">
        Try Again
      </button>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredConnections.length === 0" class="flex-1 flex items-center justify-center px-6 py-12">
      <div class="text-gray-400 mb-4">
        <svg class="w-12 h-12 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
        </svg>
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-2">No Connections Found</h3>
      <!-- <p class="text-gray-600">No connections match your current filters.</p> -->
    </div>

    <!-- Table -->
    <div v-else class="flex-1 overflow-hidden">
      <div class="overflow-auto h-full">
      <table class="min-w-full divide-y divide-gray-200 table-fixed">
        <colgroup>
          <col class="w-20"> <!-- Protocol -->
          <col class="w-32"> <!-- Local Address -->
          <col class="w-20"> <!-- Local Port -->
          <col class="w-32"> <!-- Remote Address -->
          <col class="w-20"> <!-- Remote Port -->
          <col class="w-24"> <!-- State -->
          <col class="w-16"> <!-- PID -->
          <col class="w-auto"> <!-- Process -->
        </colgroup>
        <thead class="table-header">
          <tr>
            <th @click="sortBy('protocol')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>Protocol</span>
                <SortIcon :column="'protocol'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('local_address')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>Local Address</span>
                <SortIcon :column="'local_address'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('local_port')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>Local Port</span>
                <SortIcon :column="'local_port'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('remote_address')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>Remote Address</span>
                <SortIcon :column="'remote_address'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('remote_port')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>Remote Port</span>
                <SortIcon :column="'remote_port'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('state')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>State</span>
                <SortIcon :column="'state'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('pid')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>PID</span>
                <SortIcon :column="'pid'" :sort-config="sortConfig" />
              </div>
            </th>
            <th @click="sortBy('process_name')" class="px-6 py-3 text-left cursor-pointer hover:bg-gray-100 transition-colors">
              <div class="flex items-center space-x-1">
                <span>Process</span>
                <SortIcon :column="'process_name'" :sort-config="sortConfig" />
              </div>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="connection in filteredConnections" :key="`${connection.id}`"  class="hover:bg-gray-50 transition-all duration-150 ease-in-out">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-blue-600 uppercase">
              {{ connection.protocol }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
              {{ connection.local_address }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
              {{ connection.local_port }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
              {{ connection.remote_address || '-' }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
              {{ connection.remote_port || '-' }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <StatusBadge :state="connection.state" />
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
              {{ connection.pid }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 truncate">
              {{ connection.process_name || 'Unknown' }}
            </td>
          </tr>
        </tbody>
      </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  connections: any[]
  filteredConnections: any[]
  isLoading: boolean
  error: string | null
  sortConfig: any
  fetchConnections: () => void
  sortBy: (column: string) => void
}

const props = defineProps<Props>()

// watch(
//   () => props.filteredConnections,
//   (conns) => {
//     console.log('filteredConnections changed', conns);
//   },
//   { deep: true } // 深度监听配置
// )
</script>
