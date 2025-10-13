<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'
import ContentPanel from './components/ContentPanel.vue'

const activeTab = ref('mods')

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
    <ContentPanel :active-tab="activeTab" @tab-change="handleTabChange" />
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
@import './styles/design-system.css';

#app {
  height: 100vh;
}
</style>