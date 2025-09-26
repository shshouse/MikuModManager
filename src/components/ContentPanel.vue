<script setup lang="ts">
import { ref } from 'vue'
import ModsPanel from './panels/ModsPanel.vue'
import GameDetailPanel from './panels/GameDetailPanel.vue'
import DownloadPanel from './panels/DownloadPanel.vue'
import AboutPanel from './panels/AboutPanel.vue'
import FindModsPanel from './panels/FindModsPanel.vue'
// Remove unused import
// import ToolsPanel from './panels/ToolsPanel.vue'

const props = defineProps<{
  activeTab: string
}>()

const currentGameId = ref<string>('')
const showGameDetail = ref(false)

function getTitle(): string {
  if (showGameDetail.value) {
    return '游戏详情'
  }
  
  const titles: Record<string, string> = {
    'find-mods': '找模组',
    'mods': '游戏管理',
    'tools': '小工具',
    'download': '下载(网盘)',
    'about': '关于'
  }
  return titles[props.activeTab] || '未知页面'
}

function handleNavigateToGame(gameId: string) {
  currentGameId.value = gameId
  showGameDetail.value = true
}

function handleBackToMods() {
  showGameDetail.value = false
  currentGameId.value = ''
}
</script>

<template>
  <div class="content-panel">
    <div class="content-header">
      <h1>{{ getTitle() }}</h1>
    </div>
    
    <div class="content-body">
      <FindModsPanel v-if="activeTab === 'find-mods'" />
      <GameDetailPanel 
        v-else-if="activeTab === 'mods' && showGameDetail" 
        :game-id="currentGameId"
        @back="handleBackToMods"
      />
      <ModsPanel 
        v-else-if="activeTab === 'mods'" 
        @navigate-to-game="handleNavigateToGame"
      />
      <DownloadPanel v-else-if="activeTab === 'download'" />
      
      <AboutPanel v-else-if="activeTab === 'about'" />
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