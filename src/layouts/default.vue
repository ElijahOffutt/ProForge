<template>

  <v-navigation-drawer width="300" location="left" permanent :width="400">
    <h1 style="text-align: center;" class="mt-5">👔 ProForge ⚒️</h1>
    <span style="text-align: center; display: inline-block; width: 100%; font-size: .8em;" class="mb-5">
      <u><i>Proffesional</i></u>
      <b> Freedom</b>
    </span>
    <v-divider></v-divider>
    <v-list-item link title="List Item 1"></v-list-item>
    <v-list-item link title="List Item 2"></v-list-item>
    <v-list-item link title="List Item 3"></v-list-item>
  </v-navigation-drawer>

  <v-main>
    <router-view />
  </v-main>

  <AppFooter />
</template>

<script lang="ts" setup>
//
</script>
