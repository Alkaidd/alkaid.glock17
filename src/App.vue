<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <a-config-provider
    :theme="{
      colorPrimary: '#008dff'
    }"
  >
    <RouterView />
  </a-config-provider>
</template>

<style scoped></style>
