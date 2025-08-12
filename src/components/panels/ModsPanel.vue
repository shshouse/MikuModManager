<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface CustomGame {
  id: string
  name: string
  directory: string
}

const games = ref<CustomGame[]>([])
const showAddDialog = ref(false)
const newGame = ref({
  name: '',
  directory: ''
})

function showAddGameDialog() {
  showAddDialog.value = true
  newGame.value = { name: '', directory: '' }
}

async function selectDirectory() {
  try {
    // Import the dialog API dynamically
    const { open } = await import('@tauri-apps/plugin-dialog')
    
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择游戏目录'
    })
    
    if (selected && typeof selected === 'string') {
      newGame.value.directory = selected
    }
  } catch (error) {
    console.error('Error selecting directory:', error)
    // Fallback: allow manual input
    const manualPath = prompt('请输入游戏目录路径:')
    if (manualPath) {
      newGame.value.directory = manualPath
    }
  }
}

function addGame() {
  if (newGame.value.name && newGame.value.directory) {
    const game: CustomGame = {
      id: Date.now().toString(),
      name: newGame.value.name,
      directory: newGame.value.directory
    }
    games.value.push(game)
    showAddDialog.value = false
  }
}

function cancelAdd() {
  showAddDialog.value = false
}

function removeGame(gameId: string) {
  const index = games.value.findIndex(g => g.id === gameId)
  if (index > -1) {
    games.value.splice(index, 1)
  }
}
</script>

<template>
  <div class="mods-panel">
    <div class="panel-header">
      <button class="btn-primary" @click="showAddGameDialog">添加自定义游戏</button>
    </div>

    <!-- Add Game Dialog -->
    <div v-if="showAddDialog" class="dialog-overlay">
      <div class="dialog">
        <h3>添加自定义游戏</h3>
        
        <div class="form-group">
          <label>游戏名称:</label>
          <input 
            v-model="newGame.name" 
            type="text" 
            placeholder="输入游戏名称..."
            class="form-input"
          >
        </div>
        
        <div class="form-group">
          <label>游戏目录:</label>
          <div class="directory-input">
            <input 
              v-model="newGame.directory" 
              type="text" 
              placeholder="选择游戏目录..."
              readonly
              class="form-input"
            >
            <button @click="selectDirectory" class="btn-browse">浏览</button>
          </div>
        </div>
        
        <div class="dialog-actions">
          <button @click="addGame" class="btn-primary">添加</button>
          <button @click="cancelAdd" class="btn-secondary">取消</button>
        </div>
      </div>
    </div>

    <!-- Games List -->
    <div class="games-list">
      <div v-if="games.length === 0" class="empty-state">
        <p>暂无自定义游戏，点击上方按钮添加游戏</p>
      </div>
      
      <div v-for="game in games" :key="game.id" class="game-item">
        <div class="game-info">
          <h3>{{ game.name }}</h3>
          <p class="game-directory">{{ game.directory }}</p>
        </div>
        
        <div class="game-actions">
          <button class="btn-danger" @click="removeGame(game.id)">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mods-panel {
  max-width: 800px;
}

.panel-header {
  display: flex;
  gap: 15px;
  margin-bottom: 20px;
}

.btn-primary, .btn-secondary, .btn-danger {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-secondary {
  background-color: #95a5a6;
  color: white;
}

.btn-danger {
  background-color: #e74c3c;
  color: white;
  padding: 8px 16px;
  font-size: 12px;
}

.btn-browse {
  padding: 8px 15px;
  background-color: #95a5a6;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.btn-primary:hover, .btn-secondary:hover, .btn-danger:hover, .btn-browse:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: white;
  border-radius: 8px;
  padding: 30px;
  width: 400px;
  max-width: 90vw;
}

.dialog h3 {
  margin: 0 0 20px 0;
  color: #2c3e50;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  color: #34495e;
  font-weight: 500;
}

.form-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.directory-input {
  display: flex;
  gap: 10px;
}

.directory-input .form-input {
  flex: 1;
}

.dialog-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  margin-top: 20px;
}

.games-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #7f8c8d;
  background: white;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
}

.game-item {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.game-info h3 {
  margin: 0 0 5px 0;
  color: #2c3e50;
}

.game-directory {
  color: #7f8c8d;
  font-size: 12px;
  margin: 0;
  font-family: monospace;
}

.game-actions {
  display: flex;
  align-items: center;
  gap: 15px;
}
</style>
