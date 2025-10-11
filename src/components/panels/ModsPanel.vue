<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  'navigate-to-game': [gameId: string]
  'open-game': [gameId: string]
  'open-mod-manager': []
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
  launchOptions?: string
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

// 支持的游戏状态
const gta4Status = ref<'idle' | 'scanning' | 'found' | 'not-found'>('idle')
const gta4Path = ref('')
const gta4ModInfo = ref<any>(null)

const jc3Status = ref<'idle' | 'scanning' | 'found' | 'not-found'>('idle')
const jc3Path = ref('')
const jc3ModInfo = ref<any>(null)

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
      launchOptions: '' // 初始化启动选项为空字符串
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

// GTA4相关函数
async function scanGTA4() {
  gta4Status.value = 'scanning'
  try {
    // 检查invoke函数是否可用（在浏览器环境中可能不可用）
    if (typeof window !== 'undefined' && !(window as any).__TAURI__) {
      console.warn('Tauri environment not detected - skipping backend operations')
      gta4Status.value = 'not-found'
      return
    }
    
    const paths = await invoke('scan_gta4_path') as string[]
    if (paths.length > 0) {
      gta4Path.value = paths[0]
      gta4Status.value = 'found'
      
      // 获取GTA4模组信息
      try {
        gta4ModInfo.value = await invoke('get_gta4_mod_info', { gamePath: paths[0] })
      } catch (error) {
        console.error('Failed to get GTA4 mod info:', error)
      }
    } else {
      gta4Status.value = 'not-found'
    }
  } catch (error) {
    console.error('Failed to scan GTA4 path:', error)
    gta4Status.value = 'not-found'
  }
}

function openGTA4ModManager() {
  if (gta4Path.value) {
    // 确保路径包含GTAIV子目录
    let finalPath = gta4Path.value;
    
    // 检查路径是否已经包含GTAIV子目录
    if (!finalPath.endsWith('\\GTAIV') && !finalPath.endsWith('/GTAIV')) {
      // 检查路径末尾是否有斜杠
      if (finalPath.endsWith('\\') || finalPath.endsWith('/')) {
        finalPath += 'GTAIV';
      } else {
        // 根据操作系统使用适当的路径分隔符
        const separator = finalPath.includes('\\') ? '\\' : '/';
        finalPath += separator + 'GTAIV';
      }
    }
    
    // 创建一个临时的GTA4游戏对象并导航到游戏详情页面
    const gta4Game: CustomGame = {
      id: 'gta4-special',
      name: 'GTAIV',
      directory: finalPath,
      lastPlayed: undefined,
      playTime: 0
    }
    
    // 将GTA4添加到游戏列表（如果不存在）
    const existingGTA4 = games.value.find((g: any) => g.id === 'gta4-special')
    if (!existingGTA4) {
      games.value.unshift(gta4Game)
      saveGames()
    } else {
      // 更新现有GTA4游戏的路径，确保始终使用正确的路径
      existingGTA4.directory = finalPath
      saveGames()
    }
    
    navigateToGame('gta4-special')
  }
}

// 正当防卫3相关函数
async function scanJC3() {
  jc3Status.value = 'scanning'
  try {
    // 检查invoke函数是否可用（在浏览器环境中可能不可用）
    if (typeof window !== 'undefined' && !(window as any).__TAURI__) {
      console.warn('Tauri environment not detected - skipping backend operations')
      jc3Status.value = 'not-found'
      return
    }
    
    const paths = await invoke('scan_jc3_path') as string[]
    if (paths.length > 0) {
      jc3Path.value = paths[0]
      jc3Status.value = 'found'
      
      // 获取正当防卫3模组信息
      try {
        jc3ModInfo.value = await invoke('get_jc3_mod_info', { gamePath: paths[0] })
      } catch (error) {
        console.error('Failed to get JC3 mod info:', error)
      }
    } else {
      jc3Status.value = 'not-found'
    }
  } catch (error) {
    console.error('Failed to scan JC3 path:', error)
    jc3Status.value = 'not-found'
  }
}

function openJC3ModManager() {
  if (jc3Path.value) {
    // 创建一个临时的正当防卫3游戏对象并导航到游戏详情页面
    const jc3Game: CustomGame = {
      id: 'jc3-special',
      name: 'Just Cause 3',
      directory: jc3Path.value,
      lastPlayed: undefined,
      playTime: 0,
      launchOptions: '--vfs-fs dropzone --vfs-archive patch_win64 --vfs-archive archives_win64 --vfs-fs' // 为Just Cause 3设置默认启动选项
    }
    
    // 将正当防卫3添加到游戏列表（如果不存在）
    const existingJC3 = games.value.find((g: any) => g.id === 'jc3-special')
    if (!existingJC3) {
      games.value.unshift(jc3Game)
      saveGames()
    } else {
      // 更新现有正当防卫3游戏的路径，确保始终使用正确的路径
      existingJC3.directory = jc3Path.value
      saveGames()
    }
    
    navigateToGame('jc3-special')
  }
}

// 组件挂载时自动扫描支持的游戏和加载游戏
onMounted(async () => {
  loadGames()
  
  // 检查invoke函数是否可用（在浏览器环境中可能不可用）
  if (typeof window !== 'undefined' && !(window as any).__TAURI__) {
    console.warn('Tauri environment not detected - skipping backend operations')
    return
  }
  
  scanGTA4()
  scanJC3()
  
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
  <div class="mods-panel">
    <div class="panel-header">
      <button class="btn-primary" @click="showAddGameDialog">
        添加游戏
      </button>
      <div class="stats">
        <span class="stat-item">在选择游戏路径后开始管理模组＞﹏＜,现在已添加游戏: {{ games.length }}</span>
      </div>
    </div>

    <!-- 特别支持游戏模块 -->
    <div class="module-section">
      <h2>特别支持游戏</h2>
      <div class="supported-games-content">
        <div class="game-cards-container">
          <!-- GTA4 游戏卡片 -->
          <div class="game-card gta4-card" @click="scanGTA4">
            <div class="game-info">
              <h3>GTAIV</h3>
              <p v-if="gta4Status === 'scanning'">正在扫描安装路径...</p>
              <p v-else-if="gta4Status === 'found'" class="game-directory">已找到游戏: {{ gta4Path }}</p>
              <p v-else-if="gta4Status === 'not-found'">未找到游戏安装路径</p>
              <p v-else>点击扫描游戏安装路径</p>
            </div>
            <div class="game-actions">
              <button v-if="gta4Status === 'found'" @click.stop="openGTA4ModManager" class="btn-manage">
                管理模组
              </button>
              <button @click.stop="scanGTA4" class="btn-secondary" :disabled="gta4Status === 'scanning'">
                {{ gta4Status === 'scanning' ? '扫描中...' : '重新扫描' }}
              </button>
            </div>
          </div>
          
          <!-- 正当防卫3 游戏卡片 -->
          <div class="game-card jc3-card" @click="scanJC3">
            <div class="game-info">
              <h3>Just Cause 3</h3>
              <p v-if="jc3Status === 'scanning'">正在扫描安装路径...</p>
              <p v-else-if="jc3Status === 'found'" class="game-directory">已找到游戏: {{ jc3Path }}</p>
              <p v-else-if="jc3Status === 'not-found'">未找到游戏安装路径</p>
              <p v-else>点击扫描游戏安装路径</p>
            </div>
            <div class="game-actions">
              <button v-if="jc3Status === 'found'" @click.stop="openJC3ModManager" class="btn-manage">
                管理模组
              </button>
              <button @click.stop="scanJC3" class="btn-secondary" :disabled="jc3Status === 'scanning'">
                {{ jc3Status === 'scanning' ? '扫描中...' : '重新扫描' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 自定义游戏模块 -->
    <div class="module-section">
      <h2>游戏列表</h2>
      <div class="custom-games-content">
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
                  <button @click="selectDirectory" class="btn-browse">
                    浏览
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
                <span v-else>添加游戏</span>
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
                <span v-if="!game.icon">G</span>
                <img v-else :src="game.icon" :alt="game.name">
              </div>
              
              <div class="game-info">
                <h3>{{ game.name }}</h3>
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
                class="btn-manage"
                @click="navigateToGame(game.id)"
                title="管理游戏"
              >
                管理
              </button>
              <button 
                class="btn-danger" 
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
</template>

<style scoped>
.mods-panel {
  padding: var(--space-5);
  height: 100%;
  overflow-y: auto;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-6);
  padding: var(--space-5);
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-sm);
}

.header-actions {
  display: flex;
  gap: var(--space-3);
}

.btn-icon {
  margin-right: var(--space-2);
}

.stats {
  display: flex;
  gap: var(--space-5);
}

.stat-item {
  color: var(--text-muted);
  font-size: var(--font-sm);
  font-weight: var(--font-medium);
}

/* 模块样式 */
.module-section {
  margin-bottom: var(--space-8);
}

.module-section h2 {
  margin: 0 0 var(--space-5) 0;
  color: var(--text-primary);
  font-size: var(--font-xl);
  font-weight: var(--font-semibold);
  padding-bottom: var(--space-3);
  border-bottom: 2px solid var(--primary-color);
}

.supported-games-content, .custom-games-content {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
  padding: var(--space-6);
  box-shadow: var(--shadow-sm);
}

.game-cards-container {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

/* 游戏卡片样式 */
.game-card {
  background: var(--bg-card);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--space-5);
  display: flex;
  align-items: center;
  gap: var(--space-5);
  transition: all var(--transition-base);
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.game-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 4px;
  height: 100%;
  background: var(--primary-color);
  opacity: 0;
  transition: opacity var(--transition-base);
}

.game-card:hover {
  border-color: var(--primary-color);
  box-shadow: var(--shadow-primary);
  transform: translateY(-2px);
}

.game-card:hover::before {
  opacity: 1;
}

.gta4-card {
  border-color: var(--warning-color);
}

.gta4-card:hover {
  border-color: #e67e22;
  box-shadow: 0 4px 12px rgba(243, 156, 18, 0.15);
}

.jc3-card {
  border-color: #9b59b6;
}

.jc3-card:hover {
  border-color: #8e44ad;
  box-shadow: 0 4px 12px rgba(142, 68, 173, 0.15);
}

.game-info {
  flex: 1;
}

.game-title {
  font-size: var(--font-lg);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin-bottom: var(--space-2);
}

.game-directory {
  color: var(--text-muted);
  font-size: var(--font-xs);
  margin: var(--space-1) 0;
  font-family: var(--font-family-mono);
  word-break: break-all;
}

.game-meta {
  display: flex;
  gap: var(--space-4);
  margin-top: var(--space-2);
}

.last-played, .play-time {
  color: var(--text-light);
  font-size: var(--font-xs);
  font-weight: var(--font-medium);
}

.game-actions {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

/* 对话框样式 */
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
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  width: 500px;
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: var(--shadow-xl);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-5) var(--space-6);
  border-bottom: 1px solid var(--border-color);
  background: var(--gray-50);
}

.dialog-header h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: var(--font-lg);
}

.close-btn {
  background: none;
  border: none;
  font-size: var(--font-lg);
  cursor: pointer;
  color: var(--text-muted);
  padding: var(--space-1);
  border-radius: var(--radius-sm);
  transition: all var(--transition-base);
}

.close-btn:hover {
  background: var(--gray-200);
  color: var(--text-primary);
}

.dialog-body {
  padding: var(--space-6);
}

.form-group {
  margin-bottom: var(--space-5);
}

.form-group label {
  display: block;
  margin-bottom: var(--space-2);
  color: var(--text-secondary);
  font-weight: var(--font-semibold);
  font-size: var(--font-sm);
}

.required {
  color: var(--error-color);
}

.form-input {
  width: 100%;
  padding: var(--space-3) var(--space-4);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-base);
  font-size: var(--font-sm);
  transition: border-color var(--transition-base);
  background: var(--bg-secondary);
}

.form-input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
}

.form-input.error {
  border-color: var(--error-color);
}

.directory-input {
  display: flex;
  gap: var(--space-3);
}

.directory-input .form-input {
  flex: 1;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-5) var(--space-6);
  border-top: 1px solid var(--border-color);
  background: var(--gray-50);
}

.empty-state {
  text-align: center;
  padding: var(--space-10);
  color: var(--text-muted);
}

.empty-state h3 {
  margin-bottom: var(--space-3);
  color: var(--text-secondary);
}

.empty-state p {
  margin-bottom: var(--space-4);
}
</style>