<template>
  <div class="bg-white rounded-lg shadow-md overflow-hidden">
    <!-- Table Header -->
    <div class="px-6 py-4 bg-gray-50 border-b border-gray-200">
      <h3 class="text-lg font-semibold text-gray-900">Active Connections</h3>
      <p class="text-sm text-gray-600 mt-1">
        Showing {{ filteredConnections.length }} of {{ connections.length }} connections
      </p>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="flex items-center justify-center py-12">
      <div class="loading-spinner mr-3"></div>
      <span class="text-gray-600">Loading connections...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="px-6 py-12 text-center">
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
    <div v-else-if="filteredConnections.length === 0" class="px-6 py-12 text-center">
      <div class="text-gray-400 mb-4">
        <svg class="w-12 h-12 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
        </svg>
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-2">No Connections Found</h3>
      <p class="text-gray-600">No connections match your current filters.</p>
    </div>

    <!-- Table -->
    <div v-else class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
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
          <tr v-for="connection in filteredConnections" :key="`${connection.protocol}-${connection.local_port}-${connection.pid}`" class="hover:bg-gray-50 transition-colors">
            <td class="table-cell">
              <span class="font-medium text-blue-600 uppercase">{{ connection.protocol }}</span>
            </td>
            <td class="table-cell font-mono text-sm">{{ connection.local_address }}</td>
            <td class="table-cell font-mono">{{ connection.local_port }}</td>
            <td class="table-cell font-mono text-sm">{{ connection.remote_address || '-' }}</td>
            <td class="table-cell font-mono">{{ connection.remote_port || '-' }}</td>
            <td class="table-cell">
              <StatusBadge :state="connection.state" />
            </td>
            <td class="table-cell font-mono">{{ connection.pid }}</td>
            <td class="table-cell">
              <span class="font-medium text-gray-900">{{ connection.process_name || 'Unknown' }}</span>
            </td>
          </tr>
        </tbody>
      </table>
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

defineProps<Props>()
</script>
