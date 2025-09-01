<script setup lang="ts">
import { ref } from 'vue'
const appInfo = ref({
  name: 'Miku Mod Manager',
  version: '0.1.2',
  author: 'shshouse',
  description: '简单的补丁管理工具，支持所有需要安装与回滚功能的场景',
  buildDate: '2025-08-13',
  license: 'Apache 2.0 License',
  image:'なの'
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
    <div class="app-info-card">
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
</template>

<style scoped>
  max-width: 600px;
  max-width: 600px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  background: white;
  border-radius: 12px;
  padding: 30px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  margin-bottom: 30px;

  gap: 20px;
  align-items: flex-start;
  align-items: center;
  justify-content: center;
  color: white;
}

.app-details h2 {
  margin: 0 0 5px 0;
  width: 80px;
  height: 80px;
}
  border-radius: 16px;
.version {
  color: #7f8c8d;
  margin: 0 0 10px 0;
  font-size: 32px;
}
}

  display: flex;
  margin: 0 0 5px 0;
  color: #2c3e50;
  font-size: 24px;
}

.links-section, .credits-section {
  color: #7f8c8d;
  margin: 0 0 10px 0;
  font-size: 14px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.links-section h3, .credits-section h3 {
  margin: 0 0 20px 0;
  line-height: 1.5;
}

  display: grid;
  display: flex;
  flex-direction: column;
  gap: 8px;

.link-button {
  padding: 10px 15px;
  background-color: #ecf0f1;
  gap: 10px;

.credits-section p {
  color: #34495e;
  line-height: 1.5;
  min-width: 80px;
  font-size: 14px;
  text-align: center;
  padding: 20px;
  color: #7f8c8d;
  color: #2c3e50;
  font-size: 14px;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);