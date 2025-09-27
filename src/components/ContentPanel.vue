<script setup lang="ts">
import { ref } from 'vue'
import ModsPanel from './panels/ModsPanel.vue'
import GameDetailPanel from './panels/GameDetailPanel.vue'
import ModManagerPanel from './panels/ModManagerPanel.vue'

const currentPanel = ref('mods')
const currentGameId = ref('')

function getTitle(): string {
  if (currentPanel.value === 'game-detail') {
    return '游戏详情'
  } else if (currentPanel.value === 'mod-manager') {
    return '模组管理器'
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
</script>

<template>
  <div class="content-panel">
    <div class="header">
      <h1>{{ getTitle() }}</h1>
    </div>
    
    <div class="content">
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
    </div>
  </div>
</template>

<style scoped>
.content-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
}

.content-header {
  padding: 20px 30px;
  border-bottom: 1px solid #e0e0e0;
  background-color: #f8f9fa;
}

.content-header h1 {
  font-size: 24px;
  font-weight: 600;
  color: #2c3e50;
}

.content-body {
  flex: 1;
  padding: 20px 30px;
  overflow-y: auto;
}
</style>
<ModsPanel 
  v-if="currentPanel === 'mods'" 
  @open-game="openGame"
  @open-download="openDownload"
  @open-mod-manager="openModManager"
/>
<GameDetailPanel 
  v-if="currentPanel === 'game-detail'" 
  :game-id="currentGameId"
  @back="backToMods"
/>
<DownloadPanel 
  v-if="currentPanel === 'download'" 
  @back="backToMods"
/>
<ModManagerPanel 
  v-if="currentPanel === 'mod-manager'" 
/>