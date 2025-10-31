<script setup lang="ts">
import { ref, watch } from 'vue'
import ModsPanel from './panels/ModsPanel.vue'
import GameDetailPanel from './panels/GameDetailPanel.vue'
import FindModsPanel from './panels/FindModsPanel.vue'
import DownloadPanel from './panels/DownloadPanel.vue'
import AboutPanel from './panels/AboutPanel.vue'

interface Props {
  activeTab?: string
}

const props = withDefaults(defineProps<Props>(), {
  activeTab: 'mods'
})

const currentPanel = ref(props.activeTab)
const currentGameId = ref('')

// 监听activeTab变化
watch(() => props.activeTab, (newTab) => {
  currentPanel.value = newTab
})

function openGame(gameId: string) {
  currentGameId.value = gameId
  currentPanel.value = 'game-detail'
}

function backToMods() {
  currentPanel.value = 'mods'
  currentGameId.value = ''
}
</script>

<template>
  <div class="content-panel">
    <div class="content-body">
      <ModsPanel 
        v-if="currentPanel === 'mods'" 
        @open-game="openGame"
      />
      <GameDetailPanel 
        v-if="currentPanel === 'game-detail'" 
        :game-id="currentGameId"
        @back="backToMods"
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

.content-body {
  flex: 1;
  padding: 0;
  overflow-y: auto;
  background-color: var(--bg-primary);
}
</style>