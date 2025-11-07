<script setup lang="ts">

interface MenuItem {
  id: string
  label: string
  icon: string
}

defineProps<{
  activeTab: string
}>()

const emit = defineEmits<{
  tabChange: [tab: string]
}>()

const menuItems: MenuItem[] = [
  { id: 'mods', label: '游戏库', icon: '' },
  { id: 'find-mods', label: '找模组', icon: '' },
  { id: 'download', label: '找游戏', icon: '' },
  { id: 'about', label: '关于', icon: '' }
]

function selectTab(tabId: string) {
  emit('tabChange', tabId)
}
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <img src="/MikuModManager.png" alt="Miku Mod Manager" class="logo-image" />
    </div>
    
    <nav class="sidebar-nav">
      <div 
        v-for="item in menuItems" 
        :key="item.id"
        class="nav-item"
        :class="{ active: activeTab === item.id }"
        @click="selectTab(item.id)"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </div>
    </nav>
  </div>
</template>

<style scoped>
.sidebar {
  width: 240px;
  min-width: 240px;
  flex-shrink: 0;
  background-color: #2c3e50;
  color: white;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #34495e;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.15);
  z-index: 10;
  height: 100vh;
}

.sidebar-header {
  padding: 20px 16px;
  border-bottom: 1px solid #34495e;
  text-align: center;
  background-color: #34495e;
}

.sidebar-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: white;
  margin: 0;
}

.logo-image {
  max-width: 100%;
  height: auto;
  max-height: 60px;
  object-fit: contain;
}

.sidebar-nav {
  flex: 1;
  padding: 12px 0;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-left: 3px solid transparent;
  margin: 0 8px;
  border-radius: 6px;
}

.nav-item:hover {
  background-color: #34495e;
  transform: translateX(2px);
}

.nav-item.active {
  background-color: #3498db;
  border-left-color: #2980b9;
  box-shadow: 0 2px 8px rgba(52, 152, 219, 0.3);
}

.nav-icon {
  font-size: 16px;
  margin-right: 12px;
  width: 20px;
  text-align: center;
  opacity: 0.8;
}

.nav-item.active .nav-icon {
  opacity: 1;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
  flex: 1;
}
</style>