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

// Logger function type definitions
type LogLevel = 'info' | 'debug' | 'error'

interface LogMessage {
  level: LogLevel
  message: string
  data: string | null
}

// Custom logger for both dev and production modes
export const logger = {
  log: (message: string, data?: any): void => {
    // Always show in console (dev mode)
    console.log(message, data)
    
    // Also send to Tauri backend for production logging
    if (typeof window !== 'undefined' && window.__TAURI__) {
      try {
        // Send log to Rust backend
        window.__TAURI__.core.invoke('log_message', {
          level: 'info' as LogLevel,
          message: message,
          data: data ? JSON.stringify(data) : null
        } as LogMessage).catch((): void => {
          // Silently fail if backend logging not available
        })
      } catch (e: unknown) {
        // Silently fail if Tauri not available
      }
    }
  },
  
  debug: (message: string, data?: any): void => {
    console.log(`🐛 ${message}`, data)
    
    if (typeof window !== 'undefined' && window.__TAURI__) {
      try {
        window.__TAURI__.core.invoke('log_message', {
          level: 'debug' as LogLevel,
          message: `🐛 ${message}`,
          data: data ? JSON.stringify(data) : null
        } as LogMessage).catch((): void => {})
      } catch (e: unknown) {}
    }
  },
  
  error: (message: string, error?: any): void => {
    console.error(`❌ ${message}`, error)
    
    if (typeof window !== 'undefined' && window.__TAURI__) {
      try {
        window.__TAURI__.core.invoke('log_message', {
          level: 'error' as LogLevel,
          message: `❌ ${message}`,
          data: error ? JSON.stringify(error) : null
        } as LogMessage).catch((): void => {})
      } catch (e: unknown) {}
    }
  }
}
