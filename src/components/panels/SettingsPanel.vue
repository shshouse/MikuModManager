<script setup lang="ts">
import { ref } from 'vue'

const settings = ref({
  language: 'zh-CN',
  theme: 'light',
  autoUpdate: true,
  notifications: true,
  startMinimized: false,
  closeToTray: true,
  downloadPath: 'C:\\Users\\Downloads\\Mods',
  maxDownloads: 3
})

const languages = [
  { value: 'zh-CN', label: '简体中文' },
  { value: 'en-US', label: 'English' },
  { value: 'ja-JP', label: '日本語' }
]

const themes = [
  { value: 'light', label: '浅色主题' },
  { value: 'dark', label: '深色主题' },
  { value: 'auto', label: '跟随系统' }
]

function selectDownloadPath() {
  console.log('Opening folder dialog...')
}

function saveSettings() {
  console.log('Saving settings:', settings.value)
}

function resetSettings() {
  if (confirm('确定要重置所有设置吗？')) {
    console.log('Resetting settings...')
  }
}
</script>

<template>
  <div class="settings-panel">
    <div class="settings-section">
      <h3>界面设置</h3>
      <div class="setting-item">
        <label>语言:</label>
        <select v-model="settings.language">
          <option v-for="lang in languages" :key="lang.value" :value="lang.value">
            {{ lang.label }}
          </option>
        </select>
      </div>
      
      <div class="setting-item">
        <label>主题:</label>
        <select v-model="settings.theme">
          <option v-for="theme in themes" :key="theme.value" :value="theme.value">
            {{ theme.label }}
          </option>
        </select>
      </div>
    </div>

    <div class="settings-section">
      <h3>应用行为</h3>
      <div class="checkbox-group">
        <label class="checkbox-item">
          <input v-model="settings.autoUpdate" type="checkbox">
          <span>自动检查更新</span>
        </label>
        
        <label class="checkbox-item">
          <input v-model="settings.notifications" type="checkbox">
          <span>显示通知</span>
        </label>
        
        <label class="checkbox-item">
          <input v-model="settings.startMinimized" type="checkbox">
          <span>启动时最小化到托盘</span>
        </label>
        
        <label class="checkbox-item">
          <input v-model="settings.closeToTray" type="checkbox">
          <span>关闭时最小化到托盘</span>
        </label>
      </div>
    </div>

    <div class="settings-section">
      <h3>下载设置</h3>
      <div class="setting-item">
        <label>下载路径:</label>
        <div class="path-input">
          <input 
            v-model="settings.downloadPath" 
            type="text" 
            readonly
          >
          <button @click="selectDownloadPath" class="btn-browse">浏览</button>
        </div>
      </div>
      
      <div class="setting-item">
        <label>最大同时下载数:</label>
        <input 
          v-model.number="settings.maxDownloads" 
          type="number" 
          min="1" 
          max="10"
          class="number-input"
        >
      </div>
    </div>

    <div class="actions">
      <button @click="saveSettings" class="btn-primary">保存设置</button>
      <button @click="resetSettings" class="btn-danger">重置设置</button>
    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  max-width: 600px;
}

.settings-section {
  margin-bottom: 30px;
  padding: 20px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.settings-section h3 {
  margin: 0 0 20px 0;
  color: #2c3e50;
  font-size: 18px;
}

.setting-item {
  margin-bottom: 15px;
  display: flex;
  align-items: center;
  gap: 15px;
}

.setting-item label {
  min-width: 100px;
  color: #34495e;
  font-weight: 500;
}

select, .number-input {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  min-width: 150px;
}

.path-input {
  display: flex;
  gap: 10px;
  flex: 1;
}

.path-input input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.btn-browse {
  padding: 8px 15px;
  background-color: #95a5a6;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.checkbox-item input[type="checkbox"] {
  margin: 0;
}

.actions {
  display: flex;
  gap: 15px;
}

.btn-primary, .btn-danger {
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-danger {
  background-color: #e74c3c;
  color: white;
}

.btn-primary:hover, .btn-danger:hover, .btn-browse:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}
</style>