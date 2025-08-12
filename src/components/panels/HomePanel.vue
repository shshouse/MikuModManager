<script setup lang="ts">
import { ref } from 'vue'

const stats = ref({
  installedMods: 12,
  activeMods: 8,
  totalDownloads: '1.2K'
})
</script>

<template>
  <div class="home-panel">
    <div class="welcome-card">
      <h2>欢迎使用 Miku Mods Manager</h2>
      <p>你可以在这里对你的游戏补丁进行自动安装和回滚</p>
    </div>
    
    <div class="quick-actions">
      <h3>快速操作</h3>
      <div class="action-buttons">
        <button class="action-btn primary">安装新模组</button>
        <button class="action-btn secondary">启动游戏</button>
        <button class="action-btn secondary">检查更新</button>
      </div>
    </div>
    
    <div class="stats-grid">
      <div class="stat-card">
        <h4>已安装模组</h4>
        <span class="stat-number">{{ stats.installedMods }}</span>
      </div>
      <div class="stat-card">
        <h4>活跃模组</h4>
        <span class="stat-number">{{ stats.activeMods }}</span>
      </div>
      <div class="stat-card">
        <h4>总下载量</h4>
        <span class="stat-number">{{ stats.totalDownloads }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-panel {
  max-width: 800px;
}

.welcome-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 30px;
  border-radius: 12px;
  margin-bottom: 30px;
}

.welcome-card h2 {
  margin-bottom: 10px;
  font-size: 28px;
}

.quick-actions {
  margin-bottom: 30px;
}

.quick-actions h3 {
  margin-bottom: 15px;
  color: #2c3e50;
}

.action-buttons {
  display: flex;
  gap: 15px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn.primary {
  background-color: #3498db;
  color: white;
}

.action-btn.secondary {
  background-color: #ecf0f1;
  color: #2c3e50;
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
}

.stat-card {
  background: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  text-align: center;
}

.stat-card h4 {
  color: #7f8c8d;
  margin-bottom: 10px;
  font-size: 14px;
}

.stat-number {
  font-size: 32px;
  font-weight: bold;
  color: #2c3e50;
}
</style>