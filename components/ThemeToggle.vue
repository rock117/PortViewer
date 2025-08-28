<template>
  <div class="flex items-center space-x-2">
    <!-- Theme Mode Selector -->
    <div class="relative">
      <select
        v-model="selectedTheme"
        @change="handleThemeChange"
        class="appearance-none bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md px-3 py-1 pr-8 text-sm text-gray-700 dark:text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
      >
        <option value="light">☀️ Light</option>
        <option value="dark">🌙 Dark</option>
        <option value="system">💻 System</option>
      </select>
      
      <!-- Custom dropdown arrow -->
      <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700 dark:text-gray-300">
        <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
          <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/>
        </svg>
      </div>
    </div>

    <!-- Current Theme Indicator -->
    <div class="flex items-center space-x-1 text-xs text-gray-500 dark:text-gray-400">
      <span v-if="currentTheme === 'light'">☀️</span>
      <span v-else-if="currentTheme === 'dark'">🌙</span>
      <span v-if="themeMode === 'system'" class="text-blue-600 dark:text-blue-400">
        ({{ systemPrefersDark ? 'Dark' : 'Light' }})
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useTheme, type ThemeMode } from '~/composables/useTheme'

const { themeMode, currentTheme, systemPrefersDark, setThemeMode } = useTheme()

const selectedTheme = ref<ThemeMode>(themeMode.value)

// Watch for external theme changes
watch(themeMode, (newMode) => {
  selectedTheme.value = newMode
})

const handleThemeChange = () => {
  setThemeMode(selectedTheme.value)
}
</script>
