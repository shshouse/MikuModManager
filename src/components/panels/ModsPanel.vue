<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { searchGames, fetchGameById } from '../../utils/supabase'
import type { MikuGameListItem, GameType } from '../../types/mikugame'

const emit = defineEmits<{
  'open-game': [gameId: string]
}>()

function navigateToGame(gameId: string) {
  emit('open-game', gameId)
}

interface CustomGame {
  id: string
  name: string
  directory: string
  icon?: string
  lastPlayed?: Date
  playTime?: number
  launchOptions?: string
  mikuGameId?: string // MikuGame绑定的游戏ID
  mikuGameType?: 'games' | 'h_games' | 'galgames' // MikuGame游戏类型
  images?: string[] // 本地图片路径列表
  coverImage?: string // 封面图片路径
}

const games = ref<CustomGame[]>([])
const showAddDialog = ref(false)
const newGame = ref({
  name: '',
  directory: '',
  mikuGameId: '',
  mikuGameType: '' as GameType | ''
})
const errors = ref({
  name: '',
  directory: ''
})
const isLoading = ref(false)
const showMikuGameSearch = ref(false)
const mikuGameSearchKeyword = ref('')
const mikuGameSearchResults = ref<MikuGameListItem[]>([])
const selectedMikuGame = ref<MikuGameListItem | null>(null)
const isSearchingMikuGame = ref(false)

const isFormValid = computed(() => {
  return newGame.value.name.trim() && 
         newGame.value.directory.trim() && 
         !errors.value.name && 
         !errors.value.directory
})

function showAddGameDialog() {
  showAddDialog.value = true
  newGame.value = { name: '', directory: '', mikuGameId: '', mikuGameType: '' as GameType | '' }
  errors.value = { name: '', directory: '' }
  selectedMikuGame.value = null
  mikuGameSearchKeyword.value = ''
  mikuGameSearchResults.value = []
  showMikuGameSearch.value = false
}

async function searchMikuGames() {
  if (!mikuGameSearchKeyword.value.trim()) {
    return
  }
  
  isSearchingMikuGame.value = true
  try {
    const results = await searchGames(mikuGameSearchKeyword.value)
    mikuGameSearchResults.value = results
  } catch (error) {
    console.error('搜索MikuGame失败:', error)
    alert('搜索失败: ' + error)
  } finally {
    isSearchingMikuGame.value = false
  }
}

function selectMikuGame(game: MikuGameListItem) {
  selectedMikuGame.value = game
  newGame.value.mikuGameId = game.id
  newGame.value.mikuGameType = game.game_type
  
  // 自动填充游戏名称（如果为空）
  if (!newGame.value.name.trim()) {
    newGame.value.name = game.title
    validateName()
  }
  
  showMikuGameSearch.value = false
}

function clearMikuGameSelection() {
  selectedMikuGame.value = null
  newGame.value.mikuGameId = ''
  newGame.value.mikuGameType = '' as GameType | ''
}

function validateName() {
  const name = newGame.value.name.trim()
  if (!name) {
    errors.value.name = '游戏名称不能为空'
  } else if (games.value.some((g: any) => g.name.toLowerCase() === name.toLowerCase())) {
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
      playTime: 0,
      launchOptions: '', // 初始化启动选项为空字符串
      mikuGameId: newGame.value.mikuGameId || undefined,
      mikuGameType: newGame.value.mikuGameType || undefined
    }
    
    // 如果绑定了MikuGame，下载图片
    if (game.mikuGameId && game.mikuGameType) {
      try {
        const mikuGameData = await fetchGameById(game.mikuGameId, game.mikuGameType)
        if (mikuGameData && mikuGameData.image_urls && mikuGameData.image_urls.length > 0) {
          // 获取app目录
          const appDirectory = await invoke('get_app_dir') as string
          const pictureDir = `${appDirectory}/game/${game.name}/picture`
          
          // 下载图片
          const downloadedPaths = await invoke('download_images', {
            urls: mikuGameData.image_urls,
            saveDir: pictureDir
          }) as string[]
          
          game.images = downloadedPaths
          game.coverImage = downloadedPaths[0] // 第一张作为封面
          
          console.log('已下载游戏图片:', downloadedPaths)
        }
      } catch (error) {
        console.error('下载MikuGame图片失败:', error)
        // 继续添加游戏，即使图片下载失败
      }
    }
    
    games.value.push(game)
    showAddDialog.value = false
    
    // Save to local storage or backend
    saveGames()
    
    // Create game directory structure
    try {
      // Get app directory
      const appDirectory = await invoke('get_app_dir')
      
      // Create main game directory if it doesn't exist
      const gameDir = `${appDirectory}/game`
      const gameDirExists = await invoke('file_exists', { path: gameDir })
      if (!gameDirExists) {
        await invoke('create_directory', { path: gameDir })
        console.log('Created main game directory:', gameDir)
      }
      
      // Create game-specific directory
      const gameSpecificDir = `${gameDir}/${game.name}`
      const gameSpecificDirExists = await invoke('file_exists', { path: gameSpecificDir })
      if (!gameSpecificDirExists) {
        await invoke('create_directory', { path: gameSpecificDir })
        console.log('Created game-specific directory:', gameSpecificDir)
      }
      
      // Create patch directory
      const patchDir = `${gameSpecificDir}/patch`
      const patchDirExists = await invoke('file_exists', { path: patchDir })
      if (!patchDirExists) {
        await invoke('create_directory', { path: patchDir })
        console.log('Created patch directory:', patchDir)
      }
      
      // Create backup directory
      const backupDir = `${gameSpecificDir}/backup`
      const backupDirExists = await invoke('file_exists', { path: backupDir })
      if (!backupDirExists) {
        await invoke('create_directory', { path: backupDir })
        console.log('Created backup directory:', backupDir)
      }
      
      // Create game_status.json file
      try {
        await invoke('create_game_status', {
          gameName: game.name,
          gamePath: game.directory,
          launchOptions: ''
        })
        console.log('Created game_status.json for:', game.name)
      } catch (error) {
        console.error('Failed to create game_status.json:', error)
      }
    } catch (error) {
      console.error('Failed to create game directory structure:', error)
    }
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
  const index = games.value.findIndex((g: any) => g.id === gameId)
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

// 组件挂载时加载游戏
onMounted(async () => {
  loadGames()
  
  // 检查invoke函数是否可用（在浏览器环境中可能不可用）
  if (typeof window !== 'undefined' && !(window as any).__TAURI__) {
    console.warn('Tauri environment not detected - skipping backend operations')
    return
  }
  
  // 扫描并创建缺失的game_status.json文件
  try {
    const appDirectory = await invoke('get_app_dir')
    const gamesWithoutStatus = await invoke('scan_games_for_status', { 
      appDir: appDirectory 
    }) as string[]
    
    for (const gamePath of gamesWithoutStatus) {
      try {
        const gameName = gamePath.split(/[/\\]/).pop() || 'Unknown Game'
        await invoke('create_game_status', {
          gameName: gameName,
          gamePath: gamePath,
          launchOptions: ''
        })
        console.log('Created missing game_status.json for:', gameName)
      } catch (error) {
        console.error('Failed to create game_status.json for', gamePath, ':', error)
      }
    }
  } catch (error) {
    console.error('Failed to scan for missing game status files:', error)
  }
})
</script>

<template>
  <div class="mods-panel panel">
    <div class="panel-header">
      <h2>游戏列表</h2>
      <div class="header-actions">
          <button class="btn btn-primary" @click.stop="showAddGameDialog" style="cursor: pointer;">
            添加游戏
          </button>
          <div class="stats">
            <span class="stat-item">已添加游戏: {{ games.length }}</span>
          </div>
        </div>
    </div>

    <div class="panel-body">
      <!-- 自定义游戏模块 -->
      <div class="module-section">
        <h3>自定义游戏</h3>
        <!-- Add Game Dialog -->
        <div v-if="showAddDialog" class="dialog-overlay" @click.self="cancelAdd">
          <div class="dialog">
            <div class="dialog-header">
              <h3>添加自定义游戏</h3>
              <button class="close-btn" @click="cancelAdd">×</button>
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
                  <button @click.stop="selectDirectory" class="btn btn-secondary" style="cursor: pointer;">
                    浏览
                  </button>
                </div>
                <div v-if="errors.directory" class="error-message">{{ errors.directory }}</div>
              </div>

              <!-- MikuGame绑定功能 -->
              <div class="form-group">
                <label>绑定MikuGame游戏 <span style="color: var(--text-muted); font-weight: normal;">(可选)</span></label>
                
                <!-- 已选择的游戏显示 -->
                <div v-if="selectedMikuGame" class="selected-miku-game">
                  <div class="miku-game-info">
                    <img v-if="selectedMikuGame.cover_image_url" :src="selectedMikuGame.cover_image_url" alt="封面" class="miku-game-cover">
                    <div class="miku-game-details">
                      <strong>{{ selectedMikuGame.title }}</strong>
                      <small>{{ selectedMikuGame.version || '未知版本' }}</small>
                      <small class="game-type-badge">{{ selectedMikuGame.game_type }}</small>
                    </div>
                  </div>
                  <button @click.stop="clearMikuGameSelection" class="btn btn-error btn-sm" style="cursor: pointer;">
                    取消绑定
                  </button>
                </div>

                <!-- 搜索按钮 -->
                <button 
                  v-else 
                  @click.stop="showMikuGameSearch = !showMikuGameSearch" 
                  class="btn btn-secondary"
                  style="cursor: pointer; width: 100%;"
                >
                  {{ showMikuGameSearch ? '隐藏搜索' : '搜索MikuGame游戏' }}
                </button>

                <!-- 搜索界面 -->
                <div v-if="showMikuGameSearch && !selectedMikuGame" class="miku-game-search">
                  <div class="search-input-group">
                    <input 
                      v-model="mikuGameSearchKeyword" 
                      type="text" 
                      placeholder="输入游戏名称搜索..."
                      class="form-input"
                      @keyup.enter="searchMikuGames"
                    >
                    <button 
                      @click.stop="searchMikuGames" 
                      :disabled="isSearchingMikuGame || !mikuGameSearchKeyword.trim()"
                      class="btn btn-primary"
                      style="cursor: pointer;"
                    >
                      {{ isSearchingMikuGame ? '搜索中...' : '搜索' }}
                    </button>
                  </div>

                  <!-- 搜索结果 -->
                  <div v-if="mikuGameSearchResults.length > 0" class="search-results">
                    <div 
                      v-for="result in mikuGameSearchResults" 
                      :key="result.id"
                      class="search-result-item"
                      @click="selectMikuGame(result)"
                      style="cursor: pointer;"
                    >
                      <img v-if="result.cover_image_url" :src="result.cover_image_url" alt="封面" class="result-cover">
                      <div class="result-info">
                        <strong>{{ result.title }}</strong>
                        <small>{{ result.version || '未知版本' }}</small>
                        <div class="result-tags">
                          <span v-for="tag in result.tags.slice(0, 3)" :key="tag" class="tag">{{ tag }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div v-else-if="mikuGameSearchKeyword && !isSearchingMikuGame" class="no-results">
                    未找到相关游戏
                  </div>
                </div>
              </div>

            </div>
            
            <div class="dialog-actions">
              <button 
                @click.stop="addGame" 
                class="btn btn-primary"
                :disabled="!isFormValid || isLoading"
                style="cursor: pointer;"
              >
                <span v-if="isLoading">添加中...</span>
                <span v-else>添加游戏</span>
              </button>
              <button @click.stop="cancelAdd" class="btn btn-secondary" style="cursor: pointer;">取消</button>
            </div>
          </div>
        </div>

        <!-- Games List -->
        <div class="games-list">
          <div v-if="games.length === 0" class="empty-state">
            <h3>暂无自定义游戏</h3>
            <p>点击上方按钮添加您的第一个游戏</p>
          </div>
          
          <div v-for="game in games" :key="game.id" class="card game-item">
            <div class="card-body">
              <div class="game-content" @click="navigateToGame(game.id)" style="cursor: pointer;">
                <div class="game-icon">
                  <span v-if="!game.icon">G</span>
                  <img v-else :src="game.icon" :alt="game.name">
                </div>
                
                <div class="game-info">
                  <h4>{{ game.name }}</h4>
                  <p class="game-directory">{{ game.directory }}</p>
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
                  class="btn btn-error" 
                  @click.stop="removeGame(game.id)"
                  title="删除游戏"
                >
                  删除
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mods-panel {
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-5);
  border-bottom: 1px solid var(--border-color);
}

.panel-header h2 {
  margin: 0;
}

.header-actions {
  display: flex;
  gap: var(--space-4);
  align-items: center;
}

.stats {
  font-size: var(--font-sm);
  color: var(--text-muted);
}

.panel-body {
  padding: var(--space-6);
  overflow-y: auto;
}

.module-section {
  margin-bottom: var(--space-8);
}

.module-section h3 {
  margin: 0 0 var(--space-5) 0;
  color: var(--text-primary);
  font-size: var(--font-xl);
  font-weight: var(--font-semibold);
  padding-bottom: var(--space-3);
  border-bottom: 2px solid var(--primary-color);
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
  backdrop-filter: blur(4px);
}

.dialog {
  background: var(--bg-card);
  border-radius: var(--radius-xl);
  width: 500px;
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: var(--shadow-2xl);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-5);
  border-bottom: 1px solid var(--border-color);
}

.dialog-header h3 {
  margin: 0;
  font-size: var(--font-lg);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.8em;
  cursor: pointer;
  color: var(--text-muted);
  transition: all var(--transition-base);
}

.close-btn:hover {
  color: var(--text-primary);
  transform: rotate(90deg);
}

.dialog-body {
  padding: var(--space-6);
  overflow-y: auto;
}

.form-group {
  margin-bottom: var(--space-5);
}

.form-group label {
  display: block;
  margin-bottom: var(--space-2);
  font-weight: var(--font-semibold);
}

.required {
  color: var(--error-color);
}

.directory-input {
  display: flex;
  gap: var(--space-3);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-5);
  border-top: 1px solid var(--border-color);
  background-color: var(--gray-50);
}

.empty-state {
  text-align: center;
  padding: var(--space-10);
  color: var(--text-muted);
  background-color: var(--gray-50);
  border-radius: var(--radius-md);
}

.games-list {
  display: grid;
  grid-template-columns: 1fr;
  gap: var(--space-4);
}

.game-item .card-body {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-4);
}

.game-content {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--space-4);
  min-width: 0;
}

.game-icon {
  width: 48px;
  height: 48px;
  background: var(--primary-color);
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: var(--font-bold);
  color: white;
  font-size: var(--font-xl);
  flex-shrink: 0;
}

.game-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: var(--radius-lg);
}

.game-info {
  flex: 1;
  min-width: 0;
}

.game-info h4 {
  margin: 0 0 var(--space-1) 0;
  font-size: var(--font-lg);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.game-directory {
  font-size: var(--font-xs);
  color: var(--text-muted);
  font-family: var(--font-family-mono);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.game-meta {
  display: flex;
  gap: var(--space-4);
  margin-top: var(--space-2);
  font-size: var(--font-xs);
  color: var(--text-light);
}

.game-actions {
  display: flex;
  gap: var(--space-3);
}

.error-message {
  color: var(--error-color);
  font-size: var(--font-xs);
  margin-top: var(--space-2);
}

/* MikuGame绑定相关样式 */
.selected-miku-game {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-3);
  background: var(--gray-50);
  border-radius: var(--radius-base);
  border: 1px solid var(--border-color);
}

.miku-game-info {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex: 1;
}

.miku-game-cover {
  width: 60px;
  height: 60px;
  object-fit: cover;
  border-radius: var(--radius-base);
}

.miku-game-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.miku-game-details strong {
  font-size: var(--font-sm);
  color: var(--text-primary);
}

.miku-game-details small {
  font-size: var(--font-xs);
  color: var(--text-muted);
}

.game-type-badge {
  display: inline-block;
  padding: 2px 8px;
  background: var(--primary-color);
  color: white;
  border-radius: var(--radius-sm);
  font-size: 10px !important;
}

.miku-game-search {
  margin-top: var(--space-3);
  padding: var(--space-3);
  background: var(--gray-50);
  border-radius: var(--radius-base);
  border: 1px solid var(--border-color);
}

.search-input-group {
  display: flex;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.search-input-group .form-input {
  flex: 1;
}

.search-results {
  max-height: 300px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.search-result-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3);
  background: white;
  border-radius: var(--radius-base);
  border: 1px solid var(--border-color);
  transition: all var(--transition-base);
}

.search-result-item:hover {
  border-color: var(--primary-color);
  box-shadow: var(--shadow-sm);
  transform: translateY(-1px);
}

.result-cover {
  width: 50px;
  height: 50px;
  object-fit: cover;
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}

.result-info {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  flex: 1;
}

.result-info strong {
  font-size: var(--font-sm);
  color: var(--text-primary);
}

.result-info small {
  font-size: var(--font-xs);
  color: var(--text-muted);
}

.result-tags {
  display: flex;
  gap: var(--space-1);
  flex-wrap: wrap;
  margin-top: var(--space-1);
}

.result-tags .tag {
  padding: 2px 6px;
  background: var(--gray-100);
  color: var(--text-secondary);
  border-radius: var(--radius-sm);
  font-size: 10px;
}

.no-results {
  text-align: center;
  padding: var(--space-5);
  color: var(--text-muted);
  font-size: var(--font-sm);
}
</style>