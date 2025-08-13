<script setup lang="ts">
import { ref, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

const emit = defineEmits<{
  'navigate-to-game': [gameId: string]
}>()

function navigateToGame(gameId: string) {
  emit('navigate-to-game', gameId)
}

interface CustomGame {
  id: string
  name: string
  directory: string
  icon?: string
  lastPlayed?: Date
  playTime?: number
}

const games = ref<CustomGame[]>([])
const showAddDialog = ref(false)
const newGame = ref({
  name: '',
  directory: ''
})
const errors = ref({
  name: '',
  directory: ''
})
const isLoading = ref(false)

const isFormValid = computed(() => {
  return newGame.value.name.trim() && 
         newGame.value.directory.trim() && 
         !errors.value.name && 
         !errors.value.directory
})

function showAddGameDialog() {
  showAddDialog.value = true
  newGame.value = { name: '', directory: '' }
  errors.value = { name: '', directory: '' }
}

function validateName() {
  const name = newGame.value.name.trim()
  if (!name) {
    errors.value.name = '游戏名称不能为空'
  } else if (name.length < 1) {
    errors.value.name = '游戏名称至少需要1个字符'
  } else if (games.value.some(g => g.name.toLowerCase() === name.toLowerCase())) {
    errors.value.name = '游戏名称已存在'
  } else {
    errors.value.name = ''
  }
}

function validateDirectory() {
  const directory = newGame.value.directory.trim()
  if (!directory) {
    errors.value.directory = '请选择游戏目录'
  } else {
    errors.value.directory = ''
  }
}

async function selectDirectory() {
  try {
    const selected = await open({
      multiple: false,
      directory: true,
      title: '选择游戏安装目录'
    })
    
    if (selected) {
      newGame.value.directory = selected as string
      validateDirectory()
      
      // Auto-detect game name from directory if name is empty
      if (!newGame.value.name.trim()) {
        const dirName = (selected as string).split(/[/\\]/).pop() || ''
        newGame.value.name = dirName
        validateName()
      }
      
    }
  } catch (error) {
    console.error('Failed to open directory dialog:', error)
    errors.value.directory = '无法打开目录选择器'
  }
}

async function addGame() {
  validateName()
  validateDirectory()
  
  if (!isFormValid.value) {
    return
  }
  
  isLoading.value = true
  
  try {
    const game: CustomGame = {
      id: Date.now().toString(),
      name: newGame.value.name.trim(),
      directory: newGame.value.directory.trim(),
      lastPlayed: undefined,
      playTime: 0
    }
    
    games.value.push(game)
    showAddDialog.value = false
    
    // Save to local storage or backend
    saveGames()
  } catch (error) {
    console.error('Failed to add game:', error)
  } finally {
    isLoading.value = false
  }
}

function cancelAdd() {
  showAddDialog.value = false
}

function removeGame(gameId: string) {
  const index = games.value.findIndex(g => g.id === gameId)
  if (index > -1) {
    games.value.splice(index, 1)
    saveGames()
  }
}


function saveGames() {
  // Save to localStorage for persistence
  localStorage.setItem('customGames', JSON.stringify(games.value))
}

function loadGames() {
  try {
    const saved = localStorage.getItem('customGames')
    if (saved) {
      games.value = JSON.parse(saved)
    }
  } catch (error) {
    console.error('Failed to load games:', error)
  }
}

function formatDate(date: Date): string {
  const now = new Date()
  const diff = now.getTime() - new Date(date).getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (days === 0) return '今天'
  if (days === 1) return '昨天'
  if (days < 7) return `${days}天前`
  
  return new Date(date).toLocaleDateString('zh-CN')
}

function formatPlayTime(minutes: number): string {
  if (minutes < 60) return `${minutes}分钟`
  
  const hours = Math.floor(minutes / 60)
  const remainingMinutes = minutes % 60
  
  if (hours < 24) {
    return remainingMinutes > 0 ? `${hours}小时${remainingMinutes}分钟` : `${hours}小时`
  }
  
  const days = Math.floor(hours / 24)
  const remainingHours = hours % 24
  
  return remainingHours > 0 ? `${days}天${remainingHours}小时` : `${days}天`
}

// Load games on component mount
loadGames()
</script>

<template>
  <div class="mods-panel">
    <div class="panel-header">
      <button class="btn-primary" @click="showAddGameDialog">
        添加自定义游戏
      </button>
      <div class="stats">
        <span class="stat-item">已添加游戏: {{ games.length }}</span>
      </div>
    </div>

    <!-- Add Game Dialog -->
    <div v-if="showAddDialog" class="dialog-overlay" @click.self="cancelAdd">
      <div class="dialog">
        <div class="dialog-header">
          <h3>添加自定义游戏</h3>
          <button class="close-btn" @click="cancelAdd">✕</button>
        </div>
        
        <div class="dialog-body">
          <div class="form-group">
            <label>游戏名称 <span class="required">*</span></label>
            <input 
              v-model="newGame.name" 
              type="text" 
              placeholder="输入游戏名称..."
              class="form-input"
              :class="{ error: errors.name }"
              @blur="validateName"
              @input="validateName"
            >
            <div v-if="errors.name" class="error-message">{{ errors.name }}</div>
          </div>
          
          <div class="form-group">
            <label>游戏目录 <span class="required">*</span></label>
            <div class="directory-input">
              <input 
                v-model="newGame.directory" 
                type="text" 
                placeholder="选择游戏安装目录..."
                readonly
                class="form-input"
                :class="{ error: errors.directory }"
              >
              <button @click="selectDirectory" class="btn-browse">
                📁 浏览
              </button>
            </div>
            <div v-if="errors.directory" class="error-message">{{ errors.directory }}</div>
          </div>

        </div>
        
        <div class="dialog-actions">
          <button 
            @click="addGame" 
            class="btn-primary"
            :disabled="!isFormValid || isLoading"
          >
            <span v-if="isLoading">添加中...</span>
            <span v-else>✓ 添加游戏</span>
          </button>
          <button @click="cancelAdd" class="btn-secondary">取消</button>
        </div>
      </div>
    </div>

    <!-- Games List -->
    <div class="games-list">
      <div v-if="games.length === 0" class="empty-state">
        <h3>暂无自定义游戏</h3>
        <p>点击上方按钮添加您的第一个游戏</p>
      </div>
      
      <div v-for="game in games" :key="game.id" class="game-item">
        <div class="game-content" @click="navigateToGame(game.id)">
          <div class="game-icon">
            <span v-if="!game.icon">🎮</span>
            <img v-else :src="game.icon" :alt="game.name">
          </div>
          
          <div class="game-info">
            <h3>{{ game.name }}</h3>
            <p class="game-directory">📁 {{ game.directory }}</p>
            <div class="game-meta">
              <span v-if="game.lastPlayed" class="last-played">
                上次游玩: {{ formatDate(game.lastPlayed) }}
              </span>
              <span v-if="game.playTime" class="play-time">
                游玩时间: {{ formatPlayTime(game.playTime) }}
              </span>
            </div>
          </div>
        </div>
        
        <div class="game-actions">
          <button 
            class="btn-manage"
            @click="navigateToGame(game.id)"
            title="管理游戏"
          >
            ⚙️ 管理
          </button>
          <button 
            class="btn-danger" 
            @click.stop="removeGame(game.id)"
            title="删除游戏"
          >
            🗑️ 删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mods-panel {
  max-width: 1000px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 25px;
  padding: 20px;
  background: white;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
}

.btn-icon {
  margin-right: 8px;
}

.stats {
  display: flex;
  gap: 20px;
}

.stat-item {
  color: #7f8c8d;
  font-size: 14px;
  font-weight: 500;
}

.btn-primary, .btn-secondary, .btn-danger, .btn-launch, .btn-manage {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn-primary {
  background-color: #3498db;
  color: white;
}

.btn-primary:disabled {
  background-color: #bdc3c7;
  cursor: not-allowed;
  transform: none;
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

.btn-launch {
  background-color: #27ae60;
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
  white-space: nowrap;
}

.btn-primary:not(:disabled):hover, .btn-secondary:hover, .btn-danger:hover, .btn-launch:hover, .btn-browse:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.dialog {
  background: white;
  border-radius: 12px;
  width: 500px;
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: 0 10px 30px rgba(0,0,0,0.3);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 25px;
  border-bottom: 1px solid #e0e0e0;
  background: #f8f9fa;
}

.dialog-header h3 {
  margin: 0;
  color: #2c3e50;
  font-size: 18px;
}

.close-btn {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  color: #7f8c8d;
  padding: 5px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: #e0e0e0;
  color: #2c3e50;
}

.dialog-body {
  padding: 25px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  color: #34495e;
  font-weight: 600;
  font-size: 14px;
}

.required {
  color: #e74c3c;
}

.optional {
  color: #7f8c8d;
  font-weight: normal;
}

.form-input {
  width: 100%;
  padding: 12px;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  font-size: 14px;
  transition: border-color 0.2s ease;
}

.form-input:focus {
  outline: none;
  border-color: #3498db;
}

.form-input.error {
  border-color: #e74c3c;
}

.directory-input {
  display: flex;
  gap: 10px;
}

.directory-input .form-input {
  flex: 1;
}

.error-message {
  color: #e74c3c;
  font-size: 12px;
  margin-top: 5px;
  font-weight: 500;
}

.help-text {
  color: #7f8c8d;
  font-size: 12px;
  margin-top: 5px;
  line-height: 1.4;
}

.dialog-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding: 20px 25px;
  border-top: 1px solid #e0e0e0;
  background: #f8f9fa;
}

.games-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  background: white;
  border-radius: 12px;
  border: 2px dashed #e0e0e0;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 15px;
}

.empty-state h3 {
  color: #2c3e50;
  margin: 0 0 10px 0;
  font-size: 18px;
}

.empty-state p {
  color: #7f8c8d;
  margin: 0;
}

.game-item {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 20px;
  transition: all 0.2s ease;
}

.game-item:hover {
  border-color: #3498db;
  box-shadow: 0 2px 8px rgba(52, 152, 219, 0.1);
}

.game-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f8f9fa;
  border-radius: 8px;
  font-size: 24px;
}

.game-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 8px;
}

.game-info {
  flex: 1;
}

.game-info h3 {
  margin: 0 0 8px 0;
  color: #2c3e50;
  font-size: 16px;
}

.game-directory {
  color: #7f8c8d;
  font-size: 12px;
  margin: 4px 0;
  font-family: 'Consolas', 'Monaco', monospace;
  word-break: break-all;
}

.game-meta {
  display: flex;
  gap: 15px;
  margin-top: 8px;
}

.last-played, .play-time {
  color: #95a5a6;
  font-size: 11px;
  font-weight: 500;
}

.game-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}
</style>
