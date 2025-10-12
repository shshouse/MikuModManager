<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const websiteUrl = 'https://mikugame.icu'
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
  <div class="download-panel">
    <div class="content-header">
      <h2>找游戏</h2>
    </div>
    
    <div class="content-body">
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
            <h3>点击下方按钮前往MikuGame获取您需要的游戏</h3>
            <p>MikuGame将在您的默认浏览器中打开，提供各类游戏下载和模组资源</p>
            <p class="website-url">{{ websiteUrl }}</p>
          </div>
          
          <div class="browser-actions">
            <button @click="openInBrowser" :disabled="isOpening" class="btn-primary">
              {{ isOpening ? '正在打开...' : '进入MikuGame' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.download-panel {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: #f5f5f5;
}

.content-header {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  background-color: white;
  border-bottom: 1px solid #e0e0e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.content-header h2 {
  margin: 0;
  font-size: 24px;
  color: #2c3e50;
  font-weight: 600;
}

.content-body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.browser-container {
  width: 100%;
  max-width: 600px;
  background-color: white;
  border-radius: 12px;
  padding: 32px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.2s ease;
}

.browser-container:hover {
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
}

.browser-content {
  text-align: center;
}

.browser-icon {
  margin-bottom: 24px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.browser-text h3 {
  color: #2c3e50;
  margin-bottom: 12px;
  font-size: 24px;
  font-weight: 600;
  line-height: 1.3;
}

.browser-text p {
  color: #666;
  margin-bottom: 8px;
  font-size: 14px;
  line-height: 1.5;
}

.website-url {
  color: #3498db;
  font-weight: 500;
  word-break: break-all;
  font-size: 13px;
  margin-top: 12px;
  padding: 8px 12px;
  background-color: #f8f9fa;
  border-radius: 4px;
  border: 1px solid #e0e0e0;
  display: inline-block;
}

.browser-actions {
  margin-top: 24px;
}

.btn-primary {
  padding: 12px 24px;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 16px;
  box-shadow: 0 2px 4px rgba(52, 152, 219, 0.3);
}

.btn-primary:hover:not(:disabled) {
  background-color: #2980b9;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(52, 152, 219, 0.4);
}

.btn-primary:disabled {
  background-color: #bdc3c7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .content-header {
    padding: 12px 16px;
  }
  
  .content-header h2 {
    font-size: 20px;
  }
  
  .content-body {
    padding: 12px;
  }
  
  .browser-container {
    padding: 24px;
  }
  
  .browser-icon svg {
    width: 48px;
    height: 48px;
  }
  
  .browser-text h3 {
    font-size: 20px;
  }
  
  .btn-primary {
    padding: 10px 20px;
    font-size: 14px;
  }
}
</style>