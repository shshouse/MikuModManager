<script setup lang="ts">
import ModsPanel from './panels/ModsPanel.vue'
import DownloadPanel from './panels/DownloadPanel.vue'
import SettingsPanel from './panels/SettingsPanel.vue'
import AboutPanel from './panels/AboutPanel.vue'

const props = defineProps<{
  activeTab: string
}>()

function getTitle(): string {
  const titles: Record<string, string> = {
    mods: '游戏管理',
    download: '下载中心',
    settings: '应用设置',
    about: '关于我们'
  }
  return titles[props.activeTab] || '未知页面'
}
</script>

<template>
  <div class="content-panel">
    <div class="content-header">
      <h1>{{ getTitle() }}</h1>
    </div>
    
    <div class="content-body">
      <ModsPanel v-if="activeTab === 'mods'" />
      <DownloadPanel v-else-if="activeTab === 'download'" />
      <SettingsPanel v-else-if="activeTab === 'settings'" />
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