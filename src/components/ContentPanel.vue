<script setup lang="ts">
import { ref, watch } from 'vue'
import ModsPanel from './panels/ModsPanel.vue'
import GameDetailPanel from './panels/GameDetailPanel.vue'
import ModManagerPanel from './panels/ModManagerPanel.vue'
import FindModsPanel from './panels/FindModsPanel.vue'
import DownloadPanel from './panels/DownloadPanel.vue'
import AboutPanel from './panels/AboutPanel.vue'

interface Props {
  activeTab?: string
}

const props = withDefaults(defineProps<Props>(), {
  activeTab: 'mods'
})

// const emit = defineEmits<{
//   tabChange: [tab: string]
// }>()

const currentPanel = ref(props.activeTab)
const currentGameId = ref('')

// 监听activeTab变化
watch(() => props.activeTab, (newTab) => {
  currentPanel.value = newTab
})

function getTitle(): string {
  if (currentPanel.value === 'game-detail') {
    return '游戏详情'
  } else if (currentPanel.value === 'mod-manager') {
    return '模组管理器'
  } else if (currentPanel.value === 'find-mods') {
    return '找模组'
  } else if (currentPanel.value === 'download') {
    return '找游戏'
  } else if (currentPanel.value === 'about') {
    return '关于'
  }
  return '游戏管理'
}

function openGame(gameId: string) {
  currentGameId.value = gameId
  currentPanel.value = 'game-detail'
}

function openModManager() {
  currentPanel.value = 'mod-manager'
}

function backToMods() {
  currentPanel.value = 'mods'
  currentGameId.value = ''
}

// 处理侧边栏标签切换
// function handleTabChange(tab: string) {
//   currentPanel.value = tab
//   currentGameId.value = ''
//   emit('tabChange', tab)
// }
</script>

<template>
  <div class="content-panel">
    <div class="content-header">
      <h1>{{ getTitle() }}</h1>
    </div>
    
    <div class="content-body">
      <ModsPanel 
        v-if="currentPanel === 'mods'" 
        @open-game="openGame"
        @open-mod-manager="openModManager"
      />
      <GameDetailPanel 
        v-if="currentPanel === 'game-detail'" 
        :game-id="currentGameId"
        @back="backToMods"
      />
      <ModManagerPanel 
        v-if="currentPanel === 'mod-manager'" 
      />
      <FindModsPanel 
        v-if="currentPanel === 'find-mods'" 
      />
      <DownloadPanel 
        v-if="currentPanel === 'download'" 
      />
      <AboutPanel 
        v-if="currentPanel === 'about'" 
      />
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

.content-header {
  padding: var(--space-5) var(--space-8);
  border-bottom: 1px solid var(--border-color);
  background-color: var(--bg-card);
  border-radius: var(--radius-lg) var(--radius-lg) 0 0;
  margin-bottom: 0;
  box-shadow: var(--shadow-sm);
}

.content-header h1 {
  font-size: var(--font-2xl);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin: 0;
}

.content-body {
  flex: 1;
  padding: 0;
  overflow-y: auto;
  background-color: var(--bg-primary);
}
</style>