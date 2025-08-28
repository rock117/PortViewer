<template>
  <div class="grid grid-cols-2 md:grid-cols-5 gap-4 mb-6">
    <div class="card text-center">
      <div class="text-2xl font-bold text-blue-600 dark:text-blue-400 mb-1">{{ statistics.total }}</div>
      <div class="text-sm text-gray-600 dark:text-gray-400">Total</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-green-600 dark:text-green-400 mb-1">{{ statistics.tcp }}</div>
      <div class="text-sm text-gray-600 dark:text-gray-400">TCP</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-purple-600 dark:text-purple-400 mb-1">{{ statistics.udp }}</div>
      <div class="text-sm text-gray-600 dark:text-gray-400">UDP</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-orange-600 dark:text-orange-400 mb-1">{{ statistics.listening }}</div>
      <div class="text-sm text-gray-600 dark:text-gray-400">Listening</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-red-600 dark:text-red-400 mb-1">{{ statistics.established }}</div>
      <div class="text-sm text-gray-600 dark:text-gray-400">Established</div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  statistics: {
    total: number
    tcp: number
    udp: number
    listening: number
    established: number
  }
}

defineProps<Props>()
</script>
