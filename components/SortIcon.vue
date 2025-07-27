<template>
  <div class="flex flex-col">
    <svg 
      class="w-3 h-3 text-gray-400"
      :class="{ 'text-blue-600': isActive && sortConfig.direction === 'asc' }"
      fill="currentColor" 
      viewBox="0 0 20 20"
    >
      <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" />
    </svg>
    <svg 
      class="w-3 h-3 text-gray-400 -mt-1"
      :class="{ 'text-blue-600': isActive && sortConfig.direction === 'desc' }"
      fill="currentColor" 
      viewBox="0 0 20 20"
    >
      <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
    </svg>
  </div>
</template>

<script setup lang="ts">
interface Props {
  column: string
  sortConfig: {
    column: string | null
    direction: 'asc' | 'desc'
  }
}

const props = defineProps<Props>()

const isActive = computed(() => props.sortConfig.column === props.column)
</script>
