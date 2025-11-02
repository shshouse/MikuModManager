<script setup lang="ts">
import { ref } from 'vue'
import { APP_VERSION } from '../../version'
const appInfo = ref({
  name: 'Miku Mod Manager',
  version: APP_VERSION,
  author: '松灰酸 shshouse',
  description: '开源的模组管理工具，支持快捷安装与卸载模组！附带模组冲突检测功能，完整的恢复回滚功能',
  buildDate: '2025-08-13',
  license: 'Apache 2.0 License',
  image: 'MikuMod'
})

const links = [
  { name: 'GitHub仓库', url: 'https://github.com/shshouse/MikuModsManager' },
  { name: '问题反馈', url: 'https://github.com/shshouse/MikuModsManager/issues' },
  { name: '爱发电', url: 'https://afdian.com/a/shshouse' },
  { name: 'Bilibili', url: 'https://space.bilibili.com/309820452' },
  { name: 'MikuGame', url: 'https://www.mikugame.icu' },
  { name: 'MikuMod', url: 'https://www.mikumod.com' }
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
              <img src="/Miku.png" alt="Miku Mod Manager" />
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
  max-width: 900px;
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
  width: 80px;
  height: 80px;
}

.app-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  border-radius: var(--radius-lg);
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