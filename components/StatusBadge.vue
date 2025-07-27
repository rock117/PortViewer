<template>
  <span :class="badgeClass">
    {{ state }}
  </span>
</template>

<script setup lang="ts">
interface Props {
  state: string
}

const props = defineProps<Props>()

const badgeClass = computed(() => {
  const baseClass = 'px-2 py-1 rounded-full text-xs font-medium'
  const state = props.state.toLowerCase()
  
  switch (state) {
    case 'listening':
      return `${baseClass} status-listening`
    case 'established':
      return `${baseClass} status-established`
    default:
      return `${baseClass} status-other`
  }
})
</script>
