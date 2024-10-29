<script setup lang="ts">
import SidePanel from './components/SidePanel.vue'
import MainPanel from './components/MainPanel.vue'
</script>

<template>
  <div class="flex min-h-screen bg-gray-100 dark:bg-gray-900">
    <SidePanel />
    <MainPanel />
  </div>
</template>

<style>
@import './style.css';
</style>
