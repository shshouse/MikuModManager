<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'
import ContentPanel from './components/ContentPanel.vue'

const activeTab = ref('find-mods')

function handleTabChange(tab: string) {
  activeTab.value = tab
}
</script>

<template>
  <div class="app-container">
    <Sidebar 
      :active-tab="activeTab" 
      @tab-change="handleTabChange" 
    />
    <ContentPanel :active-tab="activeTab" />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background-color: #f5f5f5;
}

#app {
  height: 100vh;
}
</style>