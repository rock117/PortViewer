import { ref, computed, watch, onMounted, readonly } from 'vue'

export type ThemeMode = 'light' | 'dark' | 'system'

const THEME_STORAGE_KEY = 'portviewer-theme'

// Reactive state
const themeMode = ref<ThemeMode>('system')
const systemPrefersDark = ref(false)

// Computed theme - resolves 'system' to actual theme
const currentTheme = computed(() => {
  if (themeMode.value === 'system') {
    return systemPrefersDark.value ? 'dark' : 'light'
  }
  return themeMode.value
})

// Check if current theme is dark
const isDark = computed(() => currentTheme.value === 'dark')

// Media query for system theme preference
let mediaQuery: MediaQueryList | null = null

const updateSystemTheme = () => {
  if (typeof window !== 'undefined') {
    systemPrefersDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
  }
}

const initializeTheme = () => {
  if (typeof window === 'undefined') return

  // Load saved theme preference
  const saved = localStorage.getItem(THEME_STORAGE_KEY)
  if (saved && ['light', 'dark', 'system'].includes(saved)) {
    themeMode.value = saved as ThemeMode
  }

  // Set up system theme detection
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  updateSystemTheme()
  
  // Listen for system theme changes
  mediaQuery.addEventListener('change', updateSystemTheme)
}

const setThemeMode = (mode: ThemeMode) => {
  themeMode.value = mode
  if (typeof window !== 'undefined') {
    localStorage.setItem(THEME_STORAGE_KEY, mode)
  }
}

const applyTheme = () => {
  if (typeof document === 'undefined') return

  const html = document.documentElement
  
  if (isDark.value) {
    html.classList.add('dark')
  } else {
    html.classList.remove('dark')
  }
}

// Watch for theme changes and apply them
watch(currentTheme, applyTheme, { immediate: true })

export const useTheme = () => {
  onMounted(() => {
    initializeTheme()
  })

  return {
    themeMode: readonly(themeMode),
    currentTheme: readonly(currentTheme),
    isDark: readonly(isDark),
    systemPrefersDark: readonly(systemPrefersDark),
    setThemeMode,
    applyTheme
  }
}

// Cleanup function for SSR
export const cleanupTheme = () => {
  if (mediaQuery) {
    mediaQuery.removeEventListener('change', updateSystemTheme)
    mediaQuery = null
  }
}
