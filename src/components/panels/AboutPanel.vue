<script setup lang="ts">
import { ref } from 'vue'

const appInfo = ref({
  name: 'Miku Mods Manager',
  version: '1.0.0',
  author: 'SnailHouse',
  description: '专为游戏模组管理而设计的现代化工具',
  buildDate: '2025-01-08',
  license: 'MIT License'
})

const links = [
  { name: 'GitHub', url: 'https://github.com/snailhouse/mikumodsmanager' },
  { name: '官方网站', url: 'https://mikumods.com' },
  { name: '用户手册', url: 'https://docs.mikumods.com' },
  { name: '问题反馈', url: 'https://github.com/snailhouse/mikumodsmanager/issues' }
]

function openLink(url: string) {
  console.log('Opening link:', url)
  // In a real Tauri app, you would use the shell API to open external links
}

function checkUpdates() {
  console.log('Checking for updates...')
}
</script>

<template>
  <div class="about-panel">
    <div class="app-info-card">
      <div class="app-icon">
        <div class="icon-placeholder">🎵</div>
      </div>
      
      <div class="app-details">
        <h2>{{ appInfo.name }}</h2>
        <p class="version">版本 {{ appInfo.version }}</p>
        <p class="description">{{ appInfo.description }}</p>
        
        <div class="info-grid">
          <div class="info-item">
            <span class="label">开发者:</span>
            <span class="value">{{ appInfo.author }}</span>
          </div>
          <div class="info-item">
            <span class="label">构建日期:</span>
            <span class="value">{{ appInfo.buildDate }}</span>
          </div>
          <div class="info-item">
            <span class="label">许可证:</span>
            <span class="value">{{ appInfo.license }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="actions-section">
      <button @click="checkUpdates" class="btn-primary">检查更新</button>
    </div>

    <div class="links-section">
      <h3>相关链接</h3>
      <div class="links-grid">
        <button 
          v-for="link in links" 
          :key="link.name"
          @click="openLink(link.url)"
          class="link-button"
        >
          {{ link.name }}
        </button>
      </div>
    </div>

    <div class="credits-section">
      <h3>致谢</h3>
      <p>感谢所有为这个项目做出贡献的开发者和用户。</p>
      <p>特别感谢开源社区提供的优秀工具和库。</p>
    </div>

    <div class="copyright">
      <p>&copy; 2025 SnailHouse. All rights reserved.</p>
    </div>
  </div>
</template>

<style scoped>
.about-panel {
  max-width: 600px;
}

.app-info-card {
  background: white;
  border-radius: 12px;
  padding: 30px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  margin-bottom: 30px;
  display: flex;
  gap: 20px;
  align-items: flex-start;
}

.app-icon {
  flex-shrink: 0;
}

.icon-placeholder {
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  color: white;
}

.app-details h2 {
  margin: 0 0 5px 0;
  color: #2c3e50;
  font-size: 24px;
}

.version {
  color: #7f8c8d;
  margin: 0 0 10px 0;
  font-size: 14px;
}

.description {
  color: #34495e;
  margin: 0 0 20px 0;
  line-height: 1.5;
}

.info-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-item {
  display: flex;
  gap: 10px;
}

.label {
  color: #7f8c8d;
  min-width: 80px;
  font-size: 14px;
}

.value {
  color: #2c3e50;
  font-size: 14px;
}

.actions-section {
  margin-bottom: 30px;
}

.btn-primary {
  padding: 12px 24px;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}

.links-section, .credits-section {
  background: white;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.links-section h3, .credits-section h3 {
  margin: 0 0 15px 0;
  color: #2c3e50;
  font-size: 18px;
}

.links-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 10px;
}

.link-button {
  padding: 10px 15px;
  background-color: #ecf0f1;
  color: #2c3e50;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s ease;
}

.link-button:hover {
  background-color: #d5dbdb;
  transform: translateY(-1px);
}

.credits-section p {
  color: #34495e;
  line-height: 1.5;
  margin: 0 0 10px 0;
}

.copyright {
  text-align: center;
  padding: 20px;
  color: #7f8c8d;
  font-size: 12px;
}
</style>