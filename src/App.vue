<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineAsyncComponent } from 'vue'

const Sidebar = defineAsyncComponent(() => import('./components/Sidebar.vue'))
const ContentPanel = defineAsyncComponent(() => import('./components/ContentPanel.vue'))
import { NMessageProvider, NConfigProvider, GlobalThemeOverrides } from 'naive-ui'

const activeTab = ref('mods')
const navigationHistory = ref<string[]>(['mods'])
const historyIndex = ref(0)
const isNavigating = ref(false)

function handleTabChange(tab: string) {
  if (isNavigating.value) {
    return
  }
  
  if (historyIndex.value < navigationHistory.value.length - 1) {
    navigationHistory.value = navigationHistory.value.slice(0, historyIndex.value + 1)
  }
  
  if (tab !== activeTab.value) {
    navigationHistory.value.push(tab)
    historyIndex.value = navigationHistory.value.length - 1
  }
  
  activeTab.value = tab
}

function navigateBack() {
  if (historyIndex.value > 0) {
    isNavigating.value = true
    historyIndex.value--
    activeTab.value = navigationHistory.value[historyIndex.value]
    setTimeout(() => {
      isNavigating.value = false
    }, 0)
  }
}

function navigateForward() {
  if (historyIndex.value < navigationHistory.value.length - 1) {
    isNavigating.value = true
    historyIndex.value++
    activeTab.value = navigationHistory.value[historyIndex.value]
    setTimeout(() => {
      isNavigating.value = false
    }, 0)
  }
}

function handleMouseButton(event: MouseEvent) {
  if (event.button === 3) {
    event.preventDefault()
    navigateBack()
  } else if (event.button === 4) {
    event.preventDefault()
    navigateForward()
  }
}

onMounted(() => {
  window.addEventListener('mousedown', handleMouseButton)
})

onUnmounted(() => {
  window.removeEventListener('mousedown', handleMouseButton)
})

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