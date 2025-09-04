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
  { id: 'find-mods', label: '找模组', icon: '' },
  { id: 'mods', label: '模组管理', icon: '' },
  { id: 'download', label: '下载中心', icon: '' },
  { id: 'about', label: '关于', icon: '' }
]

function selectTab(tabId: string) {
  emit('tabChange', tabId)
}
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <h2>Miku Mod Manager</h2>
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
  width: 200px;
  background-color: #2c3e50;
  color: white;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #34495e;
}

.sidebar-header {
  padding: 20px 16px;
  border-bottom: 1px solid #34495e;
  text-align: center;
}

.sidebar-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #ecf0f1;
}

.sidebar-nav {
  flex: 1;
  padding: 10px 0;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background-color: #34495e;
}

.nav-item.active {
  background-color: #3498db;
  border-left-color: #2980b9;
}

.nav-icon {
  font-size: 16px;
  margin-right: 12px;
  width: 20px;
  text-align: center;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}
</style>