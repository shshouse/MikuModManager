<script setup lang="ts">
import { ref } from 'vue'

interface AvailableMod {
  id: string
  name: string
  version: string
  author: string
  downloads: number
  rating: number
  description: string
  size: string
}

const availableMods = ref<AvailableMod[]>([
  {
    id: '1',
    name: 'Ultimate Graphics Pack',
    version: '3.2.1',
    author: 'GraphicsMaster',
    downloads: 15420,
    rating: 4.8,
    description: '终极画质增强包，包含高清纹理和光影效果',
    size: '245 MB'
  },
  {
    id: '2',
    name: 'Sound Overhaul Mod',
    version: '2.0.5',
    author: 'AudioPro',
    downloads: 8930,
    rating: 4.6,
    description: '全面音效重制，提供沉浸式音频体验',
    size: '128 MB'
  },
  {
    id: '3',
    name: 'Performance Optimizer',
    version: '1.8.3',
    author: 'SpeedDemon',
    downloads: 22100,
    rating: 4.9,
    description: '性能优化工具，提升游戏帧率和稳定性',
    size: '45 MB'
  }
])

const searchQuery = ref('')

function downloadMod(modId: string) {
  console.log('Downloading mod:', modId)
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

    <div class="mods-grid">
      <div v-for="mod in filteredMods()" :key="mod.id" class="mod-card">
        <div class="mod-header">
          <h3>{{ mod.name }}</h3>
          <div class="mod-rating">
            <span class="stars">★★★★★</span>
            <span class="rating-value">{{ mod.rating }}</span>
          </div>
        </div>
        
        <div class="mod-meta">
          <span class="version">v{{ mod.version }}</span>
          <span class="author">by {{ mod.author }}</span>
          <span class="size">{{ mod.size }}</span>
        </div>
        
        <p class="mod-description">{{ mod.description }}</p>
        
        <div class="mod-stats">
          <span class="downloads">{{ mod.downloads.toLocaleString() }} 下载</span>
        </div>
        
        <button @click="downloadMod(mod.id)" class="btn-download">
          下载模组
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.download-panel {
  max-width: 1000px;
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

.mod-rating {
  display: flex;
  align-items: center;
  gap: 5px;
}

.stars {
  color: #f39c12;
  font-size: 12px;
}

.rating-value {
  font-size: 12px;
  color: #7f8c8d;
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

.mod-stats {
  margin-bottom: 15px;
  font-size: 12px;
  color: #7f8c8d;
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