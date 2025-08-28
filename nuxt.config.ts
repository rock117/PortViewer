// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ['@nuxtjs/tailwindcss'],
  
  // Enable dark mode support
  tailwindcss: {
    config: {
      darkMode: 'class'
    }
  },
  
  // Tauri specific configuration
  ssr: false, // Disable SSR for Tauri desktop app
  
  // Development server configuration
  devServer: {
    port: 1420,
    host: 'localhost'
  },
  
  // Build configuration for Tauri
  nitro: {
    preset: 'static',
    output: {
      dir: 'dist',
      publicDir: 'dist'
    }
  },
  
  // Generate static files
  generate: {
    dir: 'dist'
  },
  
  // CSS framework
  css: ['~/assets/css/main.css'],
  
  // App configuration
  app: {
    head: {
      title: 'Port Viewer',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'Monitor Windows TCP/UDP port usage with process information' }
      ]
    }
  }
})
