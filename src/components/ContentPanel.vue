<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, defineAsyncComponent } from 'vue'

const ModsPanel = defineAsyncComponent(() => import('./panels/game-management/ModsPanel.vue'))
const GameDetailPanel = defineAsyncComponent(() => import('./panels/game-management/GameDetailPanel.vue'))
const FindModsPanel = defineAsyncComponent(() => import('./panels/FindModsPanel.vue'))
const DownloadPanel = defineAsyncComponent(() => import('./panels/DownloadPanel.vue'))
const AboutPanel = defineAsyncComponent(() => import('./panels/AboutPanel.vue'))

interface Props {
  activeTab?: string
}

const props = withDefaults(defineProps<Props>(), {
  activeTab: 'mods'
})

interface NavigationState {
  panel: string
  gameId: string
}

const currentPanel = ref(props.activeTab)
const currentGameId = ref('')
const panelHistory = ref<NavigationState[]>([{ panel: props.activeTab, gameId: '' }])
const panelHistoryIndex = ref(0)
const isNavigatingPanel = ref(false)

watch(() => props.activeTab, (newTab) => {
  if (!isNavigatingPanel.value) {
    currentPanel.value = newTab
    currentGameId.value = ''
    
    if (panelHistoryIndex.value < panelHistory.value.length - 1) {
      panelHistory.value = panelHistory.value.slice(0, panelHistoryIndex.value + 1)
    }
    
    panelHistory.value.push({ panel: newTab, gameId: '' })
    panelHistoryIndex.value = panelHistory.value.length - 1
  }
})

function openGame(gameId: string) {
  currentGameId.value = gameId
  currentPanel.value = 'game-detail'
  
  if (panelHistoryIndex.value < panelHistory.value.length - 1) {
    panelHistory.value = panelHistory.value.slice(0, panelHistoryIndex.value + 1)
  }
  
  panelHistory.value.push({ panel: 'game-detail', gameId })
  panelHistoryIndex.value = panelHistory.value.length - 1
}

function backToMods() {
  currentPanel.value = 'mods'
  currentGameId.value = ''
  
  if (panelHistoryIndex.value < panelHistory.value.length - 1) {
    panelHistory.value = panelHistory.value.slice(0, panelHistoryIndex.value + 1)
  }
  
  panelHistory.value.push({ panel: 'mods', gameId: '' })
  panelHistoryIndex.value = panelHistory.value.length - 1
}

function navigatePanelBack() {
  if (panelHistoryIndex.value > 0) {
    isNavigatingPanel.value = true
    panelHistoryIndex.value--
    const state = panelHistory.value[panelHistoryIndex.value]
    currentPanel.value = state.panel
    currentGameId.value = state.gameId
    setTimeout(() => {
      isNavigatingPanel.value = false
    }, 0)
  }
}

function navigatePanelForward() {
  if (panelHistoryIndex.value < panelHistory.value.length - 1) {
    isNavigatingPanel.value = true
    panelHistoryIndex.value++
    const state = panelHistory.value[panelHistoryIndex.value]
    currentPanel.value = state.panel
    currentGameId.value = state.gameId
    setTimeout(() => {
      isNavigatingPanel.value = false
    }, 0)
  }
}

function handleMouseButton(event: MouseEvent) {
  if (event.button === 3) {
    event.preventDefault()
    navigatePanelBack()
  } else if (event.button === 4) {
    event.preventDefault()
    navigatePanelForward()
  }
}

onMounted(() => {
  window.addEventListener('mousedown', handleMouseButton)
})

onUnmounted(() => {
  window.removeEventListener('mousedown', handleMouseButton)
})
</script>

<template>
  <div class="content-panel">
    <div class="content-body">
      <Transition name="panel-slide" mode="out-in">
        <ModsPanel 
          v-if="currentPanel === 'mods'" 
          key="mods"
          @open-game="openGame"
        />
        <GameDetailPanel 
          v-else-if="currentPanel === 'game-detail'" 
          key="game-detail"
          :game-id="currentGameId"
          @back="backToMods"
        />
        <FindModsPanel 
          v-else-if="currentPanel === 'find-mods'" 
          key="find-mods"
        />
        <DownloadPanel 
          v-else-if="currentPanel === 'download'" 
          key="download"
        />
        <AboutPanel 
          v-else-if="currentPanel === 'about'" 
          key="about"
        />
      </Transition>
    </div>
  </div>
</template>

<style scoped>
.content-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  height: 100vh;
}

.content-body {
  flex: 1;
  padding: 0;
  overflow-y: auto;
  background-color: var(--bg-primary);
  position: relative;
}

.panel-slide-enter-active,
.panel-slide-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.panel-slide-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.panel-slide-enter-to {
  opacity: 1;
  transform: translateX(0);
}

.panel-slide-leave-from {
  opacity: 1;
  transform: translateX(0);
}

.panel-slide-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

.panel-slide-enter-active {
  position: relative;
  z-index: 2;
}

.panel-slide-leave-active {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1;
}
</style>