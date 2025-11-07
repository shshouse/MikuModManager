<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { NCard, NButton, NSpace } from 'naive-ui'

const websiteUrl = 'https://www.mikugame.icu'
const isOpening = ref(false)

async function openInBrowser() {
  try {
    isOpening.value = true
    await invoke('open_url_in_browser', { url: websiteUrl })
  } catch (error) {
    console.error('Failed to open browser:', error)
  } finally {
    isOpening.value = false
  }
}
</script>

<template>
  <div class="download-panel">
    <div class="panel-header">
      <h2>找游戏</h2>
    </div>
    
    <div class="panel-body">
      <NCard size="large" class="browser-card" hoverable>
        <NSpace vertical align="center" :size="24">
          <div class="browser-icon">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z" stroke="#5DADE2" stroke-width="2"/>
              <path d="M8 12H16" stroke="#5DADE2" stroke-width="2" stroke-linecap="round"/>
              <path d="M12 8V16" stroke="#5DADE2" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>
          
          <div class="browser-text">
            <h3>MikuGame初音游戏库持续更新中，遇到问题可以直接进群反馈</h3>
            <p>将在您的默认浏览器中打开，提供各类Steam游戏，Galgame下载</p>
            <NCard size="small" embedded style="margin-top: 12px; text-align: center">
              <span style="color: #5DADE2; font-weight: 500">{{ websiteUrl }}</span>
            </NCard>
          </div>
          
          <NButton type="primary" size="large" :loading="isOpening" @click="openInBrowser">
            {{ isOpening ? '正在打开...' : '进入MikuGame' }}
          </NButton>
        </NSpace>
      </NCard>
    </div>
  </div>
</template>

<style scoped>
.download-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
}

.panel-header {
  padding: 24px;
  border-bottom: 1px solid #e8e8e8;
  background: linear-gradient(135deg, #EBF5FB 0%, #ffffff 100%);
}

.panel-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 700;
  background: linear-gradient(135deg, #5DADE2, #3498DB);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.panel-body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.browser-card {
  max-width: 600px;
  width: 100%;
}

.browser-icon {
  display: flex;
  justify-content: center;
  align-items: center;
}

.browser-text {
  text-align: center;
  width: 100%;
}

.browser-text h3 {
  color: #2c3e50;
  margin: 0 0 12px 0;
  font-size: 20px;
  font-weight: 600;
}

.browser-text p {
  color: #7f8c8d;
  margin: 0;
  font-size: 14px;
  line-height: 1.6;
}
</style>
