<template>
  <v-container class="pt-10">
    <v-row justify="space-between">
      <v-col>
        <h1>Resumes</h1>
      </v-col>
      <v-col cols="2">
        <CreateModal />
      </v-col>
      <v-col cols="12">
        <v-divider></v-divider>
      </v-col>
      <v-col cols="12">
        <p>Looks like you do not have any Resumes yet; get started by hitting that "create" button to the top right.</p>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup>


let resumes = [
  {
    title: '',

  }
]
</script>
