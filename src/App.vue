<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'
import ContentPanel from './components/ContentPanel.vue'
import { NMessageProvider, NConfigProvider, GlobalThemeOverrides } from 'naive-ui'

const activeTab = ref('mods')

function handleTabChange(tab: string) {
  activeTab.value = tab
}

const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#5DADE2',
    primaryColorHover: '#3498DB',
    primaryColorPressed: '#2E86C1',
    primaryColorSuppl: '#AED6F1',
  },
  Button: {
    colorPrimary: '#5DADE2',
    colorHoverPrimary: '#3498DB',
    colorPressedPrimary: '#2E86C1',
  },
  Card: {
    borderRadius: '12px',
  },
}
</script>

<template>
  <NConfigProvider :theme-overrides="themeOverrides">
    <NMessageProvider>
      <div class="app-container">
        <Sidebar 
          :active-tab="activeTab" 
          @tab-change="handleTabChange" 
        />
        <ContentPanel :active-tab="activeTab" @tab-change="handleTabChange" />
      </div>
    </NMessageProvider>
  </NConfigProvider>
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