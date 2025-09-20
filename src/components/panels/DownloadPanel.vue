<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface AvailableMod {
  id: string
  name: string
  version: string
  author: string
  description: string
  size: string
  downloadUrl: string
}

const availableMods = ref<AvailableMod[]>([
  {
    id: '1',
    name: 'GTAIV 整合包',
    version: '1.0',
    author: 'shshouse',
    description: '包含修改器，汉化，帧率限制补丁，ENB，支线任务(让所有出租车可以接单)',
    size: '131 MB',
    downloadUrl: 'https://pan.quark.cn/s/aecd8dfab30c'
  },
    {
    id: '2',
    name: 'GTAIV 整合包（旧版）纪念用',
    version: '1.1',
    author: 'shshouse',
    description: '旧版的安装器+整合包，纪念用，需要.net runtime前置依赖',
    size: '180.9 MB',
    downloadUrl: 'https://pan.quark.cn/s/ec8d05bd40b7'
  }, 
  {
    id: '3',
    name: 'GTAIV 4K材质包',
    version: '未知',
    author: '未知',
    description: '转载自里昂，配置好可以尝试使用',
    size: '1.47 GB',
    downloadUrl: 'https://pan.quark.cn/s/e1f8cf6cd1b5'
  },
  {
    id: '4',
    name: '正当防卫3（Just Cause 3）整合包',
    version: '0.1',
    author: 'shshouse',
    description: '包含汉化，十公里钩索，跳过开头动画',
    size: '20.6 MB',
    downloadUrl: 'https://pan.quark.cn/s/0e846b35873f'
  }
])

const searchQuery = ref('')
const isOpening = ref<{[key: string]: boolean}>({})

async function downloadMod(modId: string) {
  const mod = availableMods.value.find(m => m.id === modId);
  if (mod && mod.downloadUrl) {
    try {
      isOpening.value[modId] = true
      // 使用Tauri的Webview API打开默认浏览器
      await invoke('open_url_in_browser', { url: mod.downloadUrl })
    } catch (error) {
      console.error('Failed to open browser:', error)
      // 如果Tauri方法失败，可以尝试其他方式
    } finally {
      isOpening.value[modId] = false
    }
  } else {
    console.log('Downloading mod:', modId);
  }
}

function filteredMods() {
  if (!searchQuery.value) return availableMods.value
  return availableMods.value.filter(mod => 
    mod.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    mod.description.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
}
</script>

<template>
  <div class="download-panel">
    <div class="search-section">
      <input 
        v-model="searchQuery"
        type="text" 
        placeholder="搜索模组..."
        class="search-input"
      >
      <button class="btn-refresh">刷新</button>
    </div>
    <div class="use-method-card">
      <h3 class="method-title">使用方法</h3>
        <div class="step-item">
          <div class="step-content">
            <p>点击下载按钮前往网盘下载。</p>
          </div>
        </div>
        <div class="step-item">
          <div class="step-content">
            <p>下载完成后，将模组文件解压到game/游戏路径名/patch，注意您的补丁文件夹内不能有其他文件，安装时会将文件夹内所有文件安装您选择的目录。</p>
          </div>
        </div>
        <div class="step-item">
          <div class="step-content">
            <p>如果游戏打开错误，请使用恢复功能。</p>
          </div>
        </div>
    </div>

    <div class="mods-grid">
      <div v-for="mod in filteredMods()" :key="mod.id" class="mod-card">
        <div class="mod-header">
          <h3>{{ mod.name }}</h3>
        </div>
        
        <div class="mod-meta">
          <span class="version">v{{ mod.version }}</span>
          <span class="author">by {{ mod.author }}</span>
          <span class="size">{{ mod.size }}</span>
        </div>
        
        <p class="mod-description">{{ mod.description }}</p>
        

        
        <button @click="downloadMod(mod.id)" :disabled="isOpening[mod.id]" class="btn-download">
          {{ isOpening[mod.id] ? '正在打开...' : '下载模组' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.download-panel {
  max-width: 1000px;
}

.use-method-card {
  background: white;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 30px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.08);
}

.method-title {
  margin-top: 0;
  margin-bottom: 20px;
  color: #2c3e50;
  font-size: 18px;
  font-weight: 600;
}

.step-item {
  margin-bottom: 16px;
  position: relative;
}

.step-item:last-child {
  margin-bottom: 0;
}

.step-content {
  flex: 1;
}

.step-content p {
  color: #34495e;
  line-height: 1.6;
  margin: 0;
  padding-left: 24px;
  position: relative;
}

.step-content p::before {
  content: '';
  position: absolute;
  left: 0;
  top: 8px;
  width: 8px;
  height: 8px;
  background-color: #3498db;
  border-radius: 50%;
}

.search-section {
  display: flex;
  gap: 15px;
  margin-bottom: 25px;
}

.search-input {
  flex: 1;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
}

.btn-refresh {
  padding: 12px 20px;
  background-color: #95a5a6;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
}

.mods-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.mod-card {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 20px;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.mod-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

.mod-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 10px;
}

.mod-header h3 {
  margin: 0;
  color: #2c3e50;
  font-size: 16px;
}



.mod-meta {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
  font-size: 12px;
  color: #7f8c8d;
}

.mod-description {
  color: #34495e;
  font-size: 14px;
  line-height: 1.4;
  margin-bottom: 15px;
}



.btn-download {
  width: 100%;
  padding: 10px;
  background-color: #27ae60;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: background-color 0.2s ease;
}

.btn-download:hover {
  background-color: #219a52;
}

.btn-refresh:hover {
  background-color: #7f8c8d;
}
</style>