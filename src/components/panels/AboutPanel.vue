<script setup lang="ts">
import { ref } from 'vue'
import { APP_VERSION } from '../../version'
import { NCard, NButton, NSpace, NTag, NGrid, NGridItem } from 'naive-ui'

const appInfo = ref({
  name: 'Miku Mod Manager',
  version: APP_VERSION,
  author: '松灰酸 shshouse',
  description: '开源的模组管理工具，支持快捷安装与卸载模组！附带模组冲突检测功能，完整的恢复回滚功能',
  buildDate: '2025-11-4',
  license: 'GPL-3.0 license',
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
  <div class="about-panel">
    <div class="panel-header">
      <h2>关于</h2>
    </div>
    
    <div class="panel-body">
      <div class="about-content">
        <NCard size="large" :bordered="true">
          <NSpace align="start" :size="24">
            <div class="app-icon">
              <img src="/Miku.png" alt="Miku Mod Manager" />
            </div>
            <div class="app-details">
              <h2>{{ appInfo.name }}</h2>
              <NTag type="info" size="medium" style="margin: 8px 0 16px 0">
                版本 {{ appInfo.version }}
              </NTag>
              <p class="description">{{ appInfo.description }}</p>
              
              <NGrid :x-gap="12" :y-gap="12" :cols="2" style="margin-top: 16px">
                <NGridItem>
                  <NCard size="small" embedded>
                    <div class="info-item">
                      <span class="label">开发者</span>
                      <span class="value">{{ appInfo.author }}</span>
                    </div>
                  </NCard>
                </NGridItem>
                <NGridItem>
                  <NCard size="small" embedded>
                    <div class="info-item">
                      <span class="label">构建日期</span>
                      <span class="value">{{ appInfo.buildDate }}</span>
                    </div>
                  </NCard>
                </NGridItem>
                <NGridItem>
                  <NCard size="small" embedded>
                    <div class="info-item">
                      <span class="label">许可证</span>
                      <span class="value">{{ appInfo.license }}</span>
                    </div>
                  </NCard>
                </NGridItem>
                <NGridItem>
                  <NCard size="small" embedded>
                    <div class="info-item">
                      <span class="label">图标来自</span>
                      <span class="value">なの</span>
                    </div>
                  </NCard>
                </NGridItem>
              </NGrid>
            </div>
          </NSpace>
        </NCard>

        <NCard size="large" title="相关链接" :bordered="true">
          <NGrid :x-gap="12" :y-gap="12" :cols="2">
            <NGridItem v-for="link in links" :key="link.name">
              <NButton block @click="openLink(link.url)">
                {{ link.name }}
              </NButton>
            </NGridItem>
          </NGrid>
        </NCard>

        <NCard size="large" title="致谢" :bordered="true">
          <p style="color: #7f8c8d; line-height: 1.6; margin: 0">
            特别感谢开源社区提供的优秀工具和库。
          </p>
        </NCard>

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
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 24px;
  overflow-y: auto;
}

.about-content {
  width: 100%;
  max-width: 900px;
  display: flex;
  flex-direction: column;
  gap: 24px;
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
  border-radius: 12px;
}

.app-details {
  flex: 1;
}

.app-details h2 {
  margin: 0;
  color: #2c3e50;
  font-size: 24px;
  font-weight: 700;
}

.description {
  color: #7f8c8d;
  line-height: 1.6;
  margin: 0;
  font-size: 14px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.label {
  color: #7f8c8d;
  font-size: 12px;
  font-weight: 500;
}

.value {
  color: #2c3e50;
  font-size: 14px;
  font-weight: 600;
}

.copyright {
  text-align: center;
  color: #95a5a6;
  font-size: 12px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #e0e0e0;
}

@media (max-width: 768px) {
  .panel-body {
    padding: 16px;
  }
  
  :deep(.n-space) {
    flex-direction: column;
    align-items: center;
  }
  
  .app-details {
    text-align: center;
  }
}
</style>
