// Utilities
import { defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'

export const useAppStore = defineStore('app', () => {

  // CONFIGURATION LOGIC
  let configured = useLocalStorage('Configured', true)
  let updateConfigured = () => configured.value = !configured.value

  // 
  let databasePath = ref('')

  return {
    // STATE
    configured,
    // ACTIONS
    updateConfigured
  }
})
