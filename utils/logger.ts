// Tauri type declarations
declare global {
  interface Window {
    __TAURI__?: {
      core: {
        invoke: (cmd: string, args?: any) => Promise<any>
      }
    }
  }
}

// Custom logger for both dev and production modes
export const logger = {
  log: (message: string, data?: any) => {
    // Always show in console (dev mode)
    console.log(message, data)
    
    // Also send to Tauri backend for production logging
    if (typeof window !== 'undefined' && window.__TAURI__) {
      try {
        // Send log to Rust backend
        window.__TAURI__.core.invoke('log_message', {
          level: 'info',
          message: message,
          data: data ? JSON.stringify(data) : null
        }).catch(() => {
          // Silently fail if backend logging not available
        })
      } catch (e) {
        // Silently fail if Tauri not available
      }
    }
  },
  
  debug: (message: string, data?: any) => {
    console.log(`🐛 ${message}`, data)
    
    if (typeof window !== 'undefined' && window.__TAURI__) {
      try {
        window.__TAURI__.core.invoke('log_message', {
          level: 'debug',
          message: `🐛 ${message}`,
          data: data ? JSON.stringify(data) : null
        }).catch(() => {})
      } catch (e) {}
    }
  },
  
  error: (message: string, error?: any) => {
    console.error(`❌ ${message}`, error)
    
    if (typeof window !== 'undefined' && window.__TAURI__) {
      try {
        window.__TAURI__.core.invoke('log_message', {
          level: 'error',
          message: `❌ ${message}`,
          data: error ? JSON.stringify(error) : null
        }).catch(() => {})
      } catch (e) {}
    }
  }
}
