<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-shell'

const websiteUrl = 'https://mikumod.shshouse.icu'
const isOpening = ref(false)

async function openInBrowser() {
  try {
    isOpening.value = true
    await open(websiteUrl)
  } catch (error) {
    console.error('Failed to open browser:', error)
    // 备用方案：使用window.open
    window.open(websiteUrl, '_blank')
  } finally {
    isOpening.value = false
  }
}

// onMounted(() => {
//   // 组件挂载后自动打开网站
//   openInBrowser()
// })
</script>

<template>
  <div class="find-mods-panel">
    <div class="browser-container">
      <div class="browser-content">
        <div class="browser-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z" stroke="#3498db" stroke-width="2"/>
            <path d="M8 12H16" stroke="#3498db" stroke-width="2" stroke-linecap="round"/>
            <path d="M12 8V16" stroke="#3498db" stroke-width="2" stroke-linecap="round"/>
          </svg>
        </div>
        
        <div class="browser-text">
          <h3>点击下方按钮前往MikuMod获取您需要的模组~</h3>
          <p>MikuMod将在您的默认浏览器中打开</p>
          <p class="website-url">{{ websiteUrl }}</p>
        </div>
        
        <div class="browser-actions">
          <button @click="openInBrowser" :disabled="isOpening" class="btn-open-browser">
            {{ isOpening ? '正在打开...' : '进入MikuMod' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.find-mods-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.browser-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f8f9fa;
  border-radius: 12px;
  border: 2px dashed #e0e0e0;
}

.browser-content {
  text-align: center;
  max-width: 400px;
  padding: 40px 20px;
}

.browser-icon {
  margin-bottom: 24px;
}

.browser-text h3 {
  color: #2c3e50;
  margin-bottom: 12px;
  font-size: 20px;
  font-weight: 600;
}

.browser-text p {
  color: #7f8c8d;
  margin-bottom: 8px;
  font-size: 14px;
}

.website-url {
  color: #3498db;
  font-weight: 500;
  word-break: break-all;
  font-size: 13px;
  margin-top: 12px;
}

.browser-actions {
  margin-top: 24px;
}

.btn-open-browser {
  padding: 12px 24px;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s ease;
  margin-bottom: 12px;
}

.btn-open-browser:hover:not(:disabled) {
  background-color: #2980b9;
}

.btn-open-browser:disabled {
  background-color: #bdc3c7;
  cursor: not-allowed;
}
</style>