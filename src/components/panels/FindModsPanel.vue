<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const websiteUrl = 'https://mikumod.shshouse.icu'
const isOpening = ref(false)

async function openInBrowser() {
  try {
    isOpening.value = true
    // 使用Tauri的Webview API打开默认浏览器
    await invoke('open_url_in_browser', { url: websiteUrl })
  } catch (error) {
    console.error('Failed to open browser:', error)
    // 如果Tauri方法失败，可以尝试其他方式
  } finally {
    isOpening.value = false
  }
}
</script>

<template>
  <div class="find-mods-panel panel">
    <div class="panel-header">
      <h2>找模组</h2>
    </div>
    
    <div class="panel-body">
      <div class="card browser-container">
        <div class="card-body browser-content">
          <div class="browser-icon">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z" stroke="var(--primary-color)" stroke-width="2"/>
              <path d="M8 12H16" stroke="var(--primary-color)" stroke-width="2" stroke-linecap="round"/>
              <path d="M12 8V16" stroke="var(--primary-color)" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>
          
          <div class="browser-text">
            <h3>寻找游戏模组</h3>
            <p>点击下方按钮前往MikuMod获取您需要的模组，丰富您的游戏体验</p>
            <p class="website-url">{{ websiteUrl }}</p>
          </div>
          
          <div class="browser-actions">
            <button @click="openInBrowser" :disabled="isOpening" class="btn btn-primary">
              {{ isOpening ? '正在打开...' : '进入MikuMod' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.find-mods-panel {
  display: flex;
  flex-direction: column;
}

.panel-body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
}

.browser-container {
  width: 100%;
  max-width: 600px;
}

.browser-content {
  text-align: center;
}

.browser-icon {
  margin-bottom: var(--space-6);
  display: flex;
  justify-content: center;
  align-items: center;
}

.browser-text h3 {
  color: var(--text-primary);
  margin-bottom: var(--space-3);
  font-size: var(--font-xl);
  font-weight: var(--font-semibold);
  line-height: 1.4;
}

.browser-text p {
  color: var(--text-secondary);
  margin-bottom: var(--space-2);
  font-size: var(--font-sm);
  line-height: 1.6;
}

.website-url {
  color: var(--primary-color);
  font-weight: var(--font-medium);
  word-break: break-all;
  font-size: var(--font-sm);
  margin-top: var(--space-4);
  padding: var(--space-2) var(--space-3);
  background-color: var(--gray-50);
  border-radius: var(--radius-base);
  border: 1px solid var(--border-color);
  display: inline-block;
}

.browser-actions {
  margin-top: var(--space-6);
}

/* 响应式布局 */
@media (max-width: 768px) {
  .panel-body {
    padding: var(--space-4);
  }
  
  .browser-icon svg {
    width: 48px;
    height: 48px;
  }
  
  .browser-text h3 {
    font-size: var(--font-lg);
  }
}
</style>