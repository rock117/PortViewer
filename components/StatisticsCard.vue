<template>
  <div class="grid grid-cols-2 md:grid-cols-5 gap-4 mb-6">
    <div class="card text-center">
      <div class="text-2xl font-bold text-blue-600 mb-1">{{ statistics.total }}</div>
      <div class="text-sm text-gray-600">Total</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-green-600 mb-1">{{ statistics.tcp }}</div>
      <div class="text-sm text-gray-600">TCP</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-purple-600 mb-1">{{ statistics.udp }}</div>
      <div class="text-sm text-gray-600">UDP</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-orange-600 mb-1">{{ statistics.listening }}</div>
      <div class="text-sm text-gray-600">Listening</div>
    </div>
    
    <div class="card text-center">
      <div class="text-2xl font-bold text-red-600 mb-1">{{ statistics.established }}</div>
      <div class="text-sm text-gray-600">Established</div>
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
