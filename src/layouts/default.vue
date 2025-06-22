<template>

  <Setup v-if="!appData.configured" />

  <Left v-if="appData.configured" />

  <v-main v-if="appData.configured">
    <router-view />
  </v-main>

  <AppFooter v-if="appData.configured" />
</template>

<script setup>
import { useAppStore } from '@/stores/app'; let appData = useAppStore()

</script>
