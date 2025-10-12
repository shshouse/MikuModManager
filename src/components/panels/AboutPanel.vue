<script setup lang="ts">
import { ref } from 'vue'
import { APP_VERSION } from '../../version'
const appInfo = ref({
  name: 'Miku Mod Manager',
  version: APP_VERSION,
  author: 'shshouse',
  description: '简单的补丁管理工具，支持所有需要安装与回滚功能的场景',
  buildDate: '2025-08-13',
  license: 'Apache 2.0 License',
  image: 'MikuMod'
})

const links = [
  { name: 'GitHub仓库', url: 'https://github.com/shshouse/MikuModsManager' },
  { name: '问题反馈', url: 'https://github.com/shshouse/MikuModsManager/issues' }
]

function openLink(url: string) {
  try {
    window.open(url, '_blank')
  } catch (error) {
    console.error('Failed to open link:', error)
  }
}
</script>

<template>
  <div class="about-panel">
    <div class="content-header">
      <h2>关于</h2>
    </div>
    
    <div class="content-body">
      <div class="about-content">
        <div class="app-info-card">
          <div class="app-icon">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M21 16V8C21 4.13 17.87 1 14 1H10C6.13 1 3 4.13 3 8V16C3 19.87 6.13 23 10 23H14C17.87 23 21 19.87 21 16Z" fill="#3498db"/>
              <path d="M10 11L14 15M14 11L10 15" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
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
              <div class="info-item">
                <span class="label">图标来自:</span>
                <span class="value">{{ appInfo.image }}</span>
              </div>
            </div>
          </div>
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
          <p>特别感谢开源社区提供的优秀工具和库。</p>
        </div>

        <div class="copyright">
          <p>&copy; 2025 shshouse. All rights reserved.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.about-panel {
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
  overflow-y: auto;
}

.about-content {
  width: 100%;
  max-width: 600px;
  background-color: white;
  border-radius: 12px;
  padding: 32px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.2s ease;
}

.app-info-card {
  display: flex;
  gap: 20px;
  align-items: flex-start;
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid #e0e0e0;
}

.app-icon {
  flex-shrink: 0;
}

.app-details h2 {
  margin: 0 0 4px 0;
  color: #2c3e50;
  font-size: 24px;
  font-weight: 600;
}

.version {
  color: #666;
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 500;
}

.description {
  color: #666;
  line-height: 1.5;
  margin-bottom: 20px;
  font-size: 14px;
}

.info-grid {
  display: grid;
  gap: 8px;
}

.info-item {
  display: flex;
  gap: 12px;
  align-items: center;
}

.label {
  color: #666;
  min-width: 80px;
  font-size: 13px;
  font-weight: 500;
}

.value {
  color: #2c3e50;
  font-size: 13px;
  font-weight: 500;
}

.links-section, .credits-section {
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid #e0e0e0;
}

.links-section:last-child, .credits-section:last-child {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

.links-section h3, .credits-section h3 {
  margin: 0 0 20px 0;
  color: #2c3e50;
  font-size: 18px;
  font-weight: 600;
}

.links-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.link-button {
  padding: 12px 16px;
  background-color: #f8f9fa;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  cursor: pointer;
  color: #2c3e50;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  text-align: left;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.link-button:hover {
  background-color: #e9ecef;
  border-color: #3498db;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(52, 152, 219, 0.2);
}

.link-button:hover::after {
  content: "→";
  margin-left: 8px;
  color: #3498db;
}

.credits-section p {
  color: #666;
  line-height: 1.5;
  margin: 0;
  font-size: 14px;
}

.copyright {
  text-align: center;
  color: #666;
  font-size: 12px;
  margin-top: 32px;
  padding-top: 16px;
  border-top: 1px solid #e0e0e0;
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
  
  .about-content {
    padding: 24px;
  }
  
  .app-info-card {
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 16px;
  }
  
  .app-icon svg {
    width: 48px;
    height: 48px;
  }
  
  .info-item {
    justify-content: center;
  }
  
  .links-section h3, .credits-section h3 {
    text-align: center;
  }
  
  .link-button {
    text-align: center;
    justify-content: center;
  }
}
</style>