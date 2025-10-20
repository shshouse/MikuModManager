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
  <div class="about-panel panel">
    <div class="panel-header">
      <h2>关于</h2>
    </div>
    
    <div class="panel-body">
      <div class="about-content">
        <div class="card app-info-card">
          <div class="card-body">
            <div class="app-icon">
              <svg width="64" height="64" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M21 16V8C21 4.13 17.87 1 14 1H10C6.13 1 3 4.13 3 8V16C3 19.87 6.13 23 10 23H14C17.87 23 21 19.87 21 16Z" fill="var(--primary-color)"/>
                <path d="M10 11L14 15M14 11L10 15" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <div class="app-details">
              <h2>{{ appInfo.name }}</h2>
              <p class="version badge badge-secondary">版本 {{ appInfo.version }}</p>
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
                  <span class="value">なの</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="card links-section">
          <div class="card-header">
            <h3>相关链接</h3>
          </div>
          <div class="card-body">
            <div class="links-grid">
              <button 
                v-for="link in links" 
                :key="link.name"
                @click="openLink(link.url)"
                class="btn btn-outline"
              >
                {{ link.name }}
              </button>
            </div>
          </div>
        </div>

        <div class="card credits-section">
           <div class="card-header">
            <h3>致谢</h3>
          </div>
          <div class="card-body">
            <p>特别感谢开源社区提供的优秀工具和库。</p>
          </div>
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
  display: flex;
  flex-direction: column;
}

.panel-body {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: var(--space-6);
  overflow-y: auto;
}

.about-content {
  width: 100%;
  max-width: 700px;
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.app-info-card .card-body {
  display: flex;
  gap: var(--space-6);
  align-items: flex-start;
}

.app-icon {
  flex-shrink: 0;
}

.app-details h2 {
  margin: 0 0 var(--space-2) 0;
  color: var(--text-primary);
  font-size: var(--font-2xl);
  font-weight: var(--font-bold);
}

.version {
  margin: 0 0 var(--space-4) 0;
}

.description {
  color: var(--text-secondary);
  line-height: 1.6;
  margin-bottom: var(--space-5);
  font-size: var(--font-base);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--space-3);
}

.info-item {
  display: flex;
  gap: var(--space-3);
  align-items: center;
  background-color: var(--gray-50);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-base);
}

.label {
  color: var(--text-muted);
  font-size: var(--font-sm);
  font-weight: var(--font-medium);
}

.value {
  color: var(--text-primary);
  font-size: var(--font-sm);
  font-weight: var(--font-semibold);
}

.links-section h3, .credits-section h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: var(--font-lg);
  font-weight: var(--font-semibold);
}

.links-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-4);
}

.credits-section p {
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0;
  font-size: var(--font-base);
}

.copyright {
  text-align: center;
  color: var(--text-muted);
  font-size: var(--font-xs);
  margin-top: var(--space-5);
  padding-top: var(--space-5);
  border-top: 1px solid var(--border-color);
}

/* 响应式布局 */
@media (max-width: 768px) {
  .panel-body {
    padding: var(--space-4);
  }
  
  .app-info-card .card-body {
    flex-direction: column;
    align-items: center;
    text-align: center;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
  
  .info-item {
    justify-content: center;
  }

  .links-grid {
    grid-template-columns: 1fr;
  }
}
</style>