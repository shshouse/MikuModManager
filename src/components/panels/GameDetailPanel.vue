<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

interface CustomGame {
  id: string
  name: string
  directory: string
  icon?: string
  lastPlayed?: Date
  playTime?: number
  launchOptions?: string
}

interface Patch {
  name: string
  path: string
  installed: boolean
}

interface Backup {
  name: string
  timestamp: string
  path: string
  logFile: string
}

const props = defineProps<{
  gameId: string
}>()

const emit = defineEmits<{
  back: []
}>()

const game = ref<CustomGame | null>(null)
const patches = ref<Patch[]>([])
const backups = ref<Backup[]>([])
const selectedPatches = ref<Set<string>>(new Set())
const selectedBackup = ref<string>('')
const isLoading = ref(false)
const newGameDirectory = ref('')
const isEditingDirectory = ref(false)
const launchOptions = ref('')
const isEditingLaunchOptions = ref(false)

// Removed unused computed property

const appDirectory = ref('')

onMounted(async () => {
  await loadAppDirectory()
  await loadGame()
  await scanPatches()
  await scanBackups()
})

async function loadAppDirectory() {
  try {
    // Get current directory (where exe is located)
    appDirectory.value = await invoke('get_app_dir')

    // Check if game directory exists
    const gameDir = `${appDirectory.value}/game`
    const gameDirExists = await invoke('file_exists', { path: gameDir })

    // Create game directory if it doesn't exist
    if (!gameDirExists) {
      await invoke('create_directory', { path: gameDir })
      console.log('Created game directory:', gameDir)
    }
  } catch (error) {
    console.error('Failed to get app directory:', error)
    // Fallback to current directory
    appDirectory.value = '.'
  }
}

async function loadGame() {
  try {
    const saved = localStorage.getItem('customGames')
    if (saved) {
      const games: CustomGame[] = JSON.parse(saved)
      game.value = games.find(g => g.id === props.gameId) || null
      if (game.value) {
        newGameDirectory.value = game.value.directory
        // 加载启动选项，如果是JC3则设置默认值
        if (props.gameId === 'jc3-special') {
          launchOptions.value = game.value.launchOptions || '--vfs-fs dropzone --vfs-archive patch_win64 --vfs-archive archives_win64 --vfs-fs'
        } else {
          launchOptions.value = game.value.launchOptions || ''
        }
      }
    }
  } catch (error) {
    console.error('Failed to load game:', error)
  }
}

async function scanPatches() {
  if (!game.value) return
  
  try {
    const patchDir = `${appDirectory.value}/game/${game.value.name}/patch`
    const patchFolders = await invoke('scan_directory', { path: patchDir }) as string[]
    
    patches.value = patchFolders.map((folder: string) => ({
      name: folder,
      path: `${patchDir}/${folder}`,
      installed: false
    }))
  } catch (error) {
    console.error('Failed to scan patches:', error)
    patches.value = []
  }
}

async function scanBackups() {
  if (!game.value) return
  
  try {
    const backupDir = `${appDirectory.value}/game/${game.value.name}/backup`
    const backupFolders = await invoke('scan_directory', { path: backupDir }) as string[]
    
    backups.value = backupFolders.map((folder: string) => ({
      name: folder,
      timestamp: formatTimestamp(folder),
      path: `${backupDir}/${folder}`,
      logFile: `${backupDir}/${folder}/install.log`
    })).sort((a: Backup, b: Backup) => b.name.localeCompare(a.name)) // Sort by timestamp desc
  } catch (error) {
    console.error('Failed to scan backups:', error)
    backups.value = []
  }
}

function formatTimestamp(timestamp: string): string {
  try {
    // Assuming timestamp format: YYYYMMDD_HHMMSS
    const year = timestamp.substring(0, 4)
    const month = timestamp.substring(4, 6)
    const day = timestamp.substring(6, 8)
    const hour = timestamp.substring(9, 11)
    const minute = timestamp.substring(11, 13)
    const second = timestamp.substring(13, 15)
    
    return `${year}-${month}-${day} ${hour}:${minute}:${second}`
  } catch {
    return timestamp
  }
}

function togglePatch(patchName: string) {
  if (selectedPatches.value.has(patchName)) {
    selectedPatches.value.delete(patchName)
  } else {
    selectedPatches.value.add(patchName)
  }
}

async function selectNewDirectory() {
  try {
    const selected = await open({
      multiple: false,
      directory: true,
      title: '选择新的游戏目录'
    })
    
    if (selected) {
      newGameDirectory.value = selected as string
    }
  } catch (error) {
    console.error('Failed to open directory dialog:', error)
  }
}

async function saveGameDirectory() {
  if (!game.value || !newGameDirectory.value.trim()) return
  
  try {
    game.value.directory = newGameDirectory.value.trim()
    
    // Update in localStorage
    const saved = localStorage.getItem('customGames')
    if (saved) {
      const games: CustomGame[] = JSON.parse(saved)
      const index = games.findIndex(g => g.id === props.gameId)
      if (index > -1) {
        games[index] = game.value
        localStorage.setItem('customGames', JSON.stringify(games))
      }
    }
    
    isEditingDirectory.value = false
  } catch (error) {
    console.error('Failed to save game directory:', error)
  }
}

function cancelEditDirectory() {
  newGameDirectory.value = game.value?.directory || ''
  isEditingDirectory.value = false
}

async function saveLaunchOptions() {
  if (!game.value) return
  
  try {
    game.value.launchOptions = launchOptions.value.trim()
    
    // Update in localStorage
    const saved = localStorage.getItem('customGames')
    if (saved) {
      const games: CustomGame[] = JSON.parse(saved)
      const index = games.findIndex(g => g.id === props.gameId)
      if (index > -1) {
        games[index] = game.value
        localStorage.setItem('customGames', JSON.stringify(games))
      }
    }
    
    isEditingLaunchOptions.value = false
  } catch (error) {
    console.error('Failed to save launch options:', error)
  }
}

function cancelEditLaunchOptions() {
  if (props.gameId === 'jc3-special') {
    launchOptions.value = game.value?.launchOptions || '--vfs-fs dropzone --vfs-archive patch_win64 --vfs-archive archives_win64 --vfs-fs'
  } else {
    launchOptions.value = game.value?.launchOptions || ''
  }
  isEditingLaunchOptions.value = false
}

function resetToDefaultLaunchOptions() {
  if (props.gameId === 'jc3-special') {
    launchOptions.value = '--vfs-fs dropzone --vfs-archive patch_win64 --vfs-archive archives_win64 --vfs-fs'
  }
}

async function launchGame() {
  if (!game.value) return
  
  isLoading.value = true
  
  try {
    const options = launchOptions.value || ''
    const result = await invoke('launch_game', { 
      gamePath: game.value.directory, 
      launchOptions: options 
    })
    
    alert(result as string)
  } catch (error) {
    console.error('Failed to launch game:', error)
    alert('启动游戏失败：' + error)
  } finally {
    isLoading.value = false
  }
}

async function installPatches() {
  if (selectedPatches.value.size === 0 || !game.value) return
  
  isLoading.value = true
  
  try {
    const timestamp = new Date().toISOString().replace(/[-:]/g, '').replace(/\..+/, '').replace('T', '_')
    const backupDir = `${appDirectory.value}/game/${game.value.name}/backup/${timestamp}`
    
    // Create backup directory
    await invoke('create_directory', { path: backupDir })
    
    const logEntries: string[] = []
    logEntries.push(`Installation Log - ${new Date().toISOString()}`)
    logEntries.push(`Game: ${game.value.name}`)
    logEntries.push(`Game Directory: ${game.value.directory}`)
    logEntries.push(`Patches Installed: ${Array.from(selectedPatches.value).join(', ')}`)
    logEntries.push('---')
    
    for (const patchName of selectedPatches.value) {
      const patch = patches.value.find(p => p.name === patchName)
      if (!patch) continue
      
      logEntries.push(`Installing patch: ${patchName}`)
      
      // Get all files in patch directory
      const patchFiles = await invoke('get_all_files', { path: patch.path }) as string[]
      
      for (const file of patchFiles) {
        const relativePath = file.replace(patch.path, '').replace(/^[/\\]/, '')
        const targetPath = `${game.value.directory}/${relativePath}`
        const backupPath = `${backupDir}/${relativePath}`
        
        // Check if target file exists and backup if needed
        const targetExists = await invoke('file_exists', { path: targetPath })
        if (targetExists) {
          await invoke('create_directory', { path: backupPath.substring(0, backupPath.lastIndexOf('/')) })
          await invoke('copy_file', { from: targetPath, to: backupPath })
          logEntries.push(`Backed up: ${relativePath}`)
        }
        
        // Copy patch file to target
        await invoke('create_directory', { path: targetPath.substring(0, targetPath.lastIndexOf('/')) })
        await invoke('copy_file', { from: file, to: targetPath })
        logEntries.push(`Installed: ${relativePath}`)
      }
    }
    
    // Write log file
    const logContent = logEntries.join('\n')
    await invoke('write_file', { path: `${backupDir}/install.log`, content: logContent })
    
    // Clear selection and refresh
    selectedPatches.value.clear()
    await scanBackups()
    
    alert('补丁安装完成！')
  } catch (error) {
    console.error('Failed to install patches:', error)
    alert('补丁安装失败：' + error)
  } finally {
    isLoading.value = false
  }
}

async function rollbackToBackup() {
  if (!selectedBackup.value || !game.value) return
  
  isLoading.value = true
  
  try {
    const backup = backups.value.find(b => b.name === selectedBackup.value)
    if (!backup) return
    
    // Read log file to understand what was installed
    const logContent = await invoke('read_file', { path: backup.logFile }) as string
    const logLines = logContent.split('\n')
    
    const installedFiles: string[] = []
    const backedUpFiles: string[] = []
    
    for (const line of logLines) {
      if (line.startsWith('Installed: ')) {
        installedFiles.push(line.substring(11))
      } else if (line.startsWith('Backed up: ')) {
        backedUpFiles.push(line.substring(11))
      }
    }
    
    // Restore backed up files
    for (const file of backedUpFiles) {
      const backupFilePath = `${backup.path}/${file}`
      const targetPath = `${game.value.directory}/${file}`
      
      const backupExists = await invoke('file_exists', { path: backupFilePath })
      if (backupExists) {
        await invoke('copy_file', { from: backupFilePath, to: targetPath })
      }
    }
    
    // Remove files that were only installed (not backed up)
    for (const file of installedFiles) {
      if (!backedUpFiles.includes(file)) {
        const targetPath = `${game.value.directory}/${file}`
        const fileExists = await invoke('file_exists', { path: targetPath })
        if (fileExists) {
          await invoke('delete_file', { path: targetPath })
        }
      }
    }
    
    selectedBackup.value = ''
    alert('回滚完成！')
  } catch (error) {
    console.error('Failed to rollback:', error)
    alert('回滚失败：' + error)
  } finally {
    isLoading.value = false
  }
}

function goBack() {
  emit('back')
}
</script>

<template>
  <div class="game-detail-panel">
    <div class="panel-header">
      <button class="btn-back" @click="goBack">
        ← 返回游戏列表
      </button>
      <h2 v-if="game">{{ game.name }}</h2>
    </div>

    <div v-if="!game" class="loading">
      <p>加载中...</p>
    </div>

    <div v-else class="content">
      <!-- 游戏详情模块 -->
      <div class="section">
        <h3>游戏详情</h3>
        <div class="game-info-card">
          <div class="info-row">
            <label>游戏名称:</label>
            <span>{{ game.name }}</span>
          </div>
          <div class="info-row">
            <label>游戏路径:</label>
            <div v-if="!isEditingDirectory" class="directory-display">
              <span class="directory-path">{{ game.directory }}</span>
              <button class="btn-edit" @click="isEditingDirectory = true">
                修改
              </button>
            </div>
            <div v-else class="directory-edit">
              <div class="directory-input">
                <input 
                  v-model="newGameDirectory" 
                  type="text" 
                  class="form-input"
                  placeholder="游戏目录路径..."
                >
                <button @click="selectNewDirectory" class="btn-browse">
                  浏览
                </button>
              </div>
              <div class="edit-actions">
                <button @click="saveGameDirectory" class="btn-save">
                  保存
                </button>
                <button @click="cancelEditDirectory" class="btn-cancel">
                  取消
                </button>
              </div>
            </div>
          </div>
          
          <!-- 启动选项设置 (仅对Just Cause 3显示) -->
          <div v-if="game.id === 'jc3-special'" class="info-row">
            <label>启动选项:</label>
            <div v-if="!isEditingLaunchOptions" class="launch-options-display">
              <span class="launch-options-text">{{ launchOptions || '(未设置)' }}</span>
              <button class="btn-edit" @click="isEditingLaunchOptions = true">
                修改
              </button>
            </div>
            <div v-else class="launch-options-edit">
              <div class="launch-options-input">
                <input 
                  v-model="launchOptions" 
                  type="text" 
                  class="form-input"
                  placeholder="输入启动选项..."
                >
                <button @click="resetToDefaultLaunchOptions" class="btn-reset">
                  重置默认
                </button>
              </div>
              <div class="edit-actions">
                <button @click="saveLaunchOptions" class="btn-save">
                  保存
                </button>
                <button @click="cancelEditLaunchOptions" class="btn-cancel">
                  取消
                </button>
              </div>
              <div class="launch-options-help">
                <small>默认选项: --vfs-fs dropzone --vfs-archive patch_win64 --vfs-archive archives_win64 --vfs-fs</small>
              </div>
            </div>
          </div>
          
          <!-- 游戏启动按钮 (仅对Just Cause 3显示) -->
          <div v-if="game.id === 'jc3-special'" class="info-row">
            <label></label>
            <div class="launch-game-section">
              <button 
                @click="launchGame" 
                :disabled="isLoading"
                class="btn-launch-game"
              >
                <span v-if="isLoading">启动中...</span>
                <span v-else>启动 正当防卫3</span>
              </button>
              <small class="launch-help">学习版使用上面设置的启动选项启动游戏，steam版根据视频在启动选项中添加</small>
            </div>
          </div>
        </div>
      </div>

      <!-- 补丁安装模块 -->
      <div class="section">
        <h3>补丁安装</h3>
        <div class="patches-card">
          <div v-if="patches.length === 0" class="empty-state">
            <p>未找到可用补丁</p>
            <small>补丁应放置在: game/{{ game.name }}/patch/ 目录下</small>
          </div>
          <div v-else>
            <div class="patches-list">
              <div 
                v-for="patch in patches" 
                :key="patch.name"
                class="patch-item"
                :class="{ selected: selectedPatches.has(patch.name) }"
                @click="togglePatch(patch.name)"
              >
                <input 
                  type="checkbox" 
                  :checked="selectedPatches.has(patch.name)"
                  @change="togglePatch(patch.name)"
                >
                <div class="patch-info">
                  <span class="patch-name">{{ patch.name }}</span>
                  <small class="patch-path">{{ patch.path }}</small>
                </div>
              </div>
            </div>
            <div class="install-actions">
              <button 
                @click="installPatches"
                :disabled="selectedPatches.size === 0 || isLoading"
                class="btn-install"
              >
                <span v-if="isLoading">安装中...</span>
                <span v-else>安装选中补丁 ({{ selectedPatches.size }})</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 备份回滚模块 -->
      <div class="section">
        <h3>备份回滚</h3>
        <div class="backups-card">
          <div v-if="backups.length === 0" class="empty-state">
            <p>暂无可用备份</p>
            <small>安装补丁时会自动创建备份</small>
          </div>
          <div v-else>
            <div class="backups-list">
              <div 
                v-for="backup in backups" 
                :key="backup.name"
                class="backup-item"
                :class="{ selected: selectedBackup === backup.name }"
                @click="selectedBackup = backup.name"
              >
                <input 
                  type="radio" 
                  :value="backup.name"
                  v-model="selectedBackup"
                  name="backup"
                >
                <div class="backup-info">
                  <span class="backup-name">{{ backup.timestamp }}</span>
                  <small class="backup-path">{{ backup.path }}</small>
                </div>
              </div>
            </div>
            <div class="rollback-actions">
              <button 
                @click="rollbackToBackup"
                :disabled="!selectedBackup || isLoading"
                class="btn-rollback"
              >
                <span v-if="isLoading">回滚中...</span>
                <span v-else>回滚到选中备份</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.game-detail-panel {
  max-width: 1000px;
  padding: 20px;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 30px;
  padding: 20px;
  background: white;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
}

.btn-back {
  padding: 10px 20px;
  background-color: #95a5a6;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s ease;
}

.btn-back:hover {
  background-color: #7f8c8d;
  transform: translateY(-1px);
}

.panel-header h2 {
  margin: 0;
  color: #2c3e50;
  font-size: 24px;
}

.loading {
  text-align: center;
  padding: 60px;
  color: #7f8c8d;
}

.content {
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.section {
  background: white;
  border-radius: 12px;
  border: 1px solid #e0e0e0;
  overflow: hidden;
}

.section h3 {
  margin: 0;
  padding: 20px 25px;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
  color: #2c3e50;
  font-size: 18px;
}

.game-info-card, .patches-card, .backups-card {
  padding: 25px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 15px;
}

.info-row:last-child {
  margin-bottom: 0;
}

.info-row label {
  font-weight: 600;
  color: #34495e;
  min-width: 80px;
}

.directory-display {
  display: flex;
  align-items: center;
  gap: 15px;
  flex: 1;
}

.directory-path {
  font-family: 'Consolas', 'Monaco', monospace;
  background: #f8f9fa;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #e0e0e0;
  flex: 1;
  word-break: break-all;
}

.btn-edit {
  padding: 6px 12px;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
}

.directory-edit {
  flex: 1;
}

.directory-input {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
}

.form-input {
  flex: 1;
  padding: 10px;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  font-size: 14px;
}

.btn-browse {
  padding: 10px 15px;
  background-color: #95a5a6;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  white-space: nowrap;
}

.launch-options-display {
  display: flex;
  align-items: center;
  gap: 15px;
  flex: 1;
}

.launch-options-text {
  font-family: 'Consolas', 'Monaco', monospace;
  background: #f8f9fa;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #e0e0e0;
  flex: 1;
  word-break: break-all;
  color: #2c3e50;
}

.launch-options-edit {
  flex: 1;
}

.launch-options-input {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
}

.btn-reset {
  padding: 10px 15px;
  background-color: #f39c12;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  white-space: nowrap;
}

.launch-options-help {
  margin-top: 8px;
  padding: 8px 12px;
  background: #e8f4fd;
  border-radius: 4px;
  border-left: 3px solid #3498db;
}

.launch-options-help small {
  color: #2980b9;
  font-family: 'Consolas', 'Monaco', monospace;
}

.launch-game-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1;
}

.btn-launch-game {
  padding: 15px 30px;
  background: linear-gradient(135deg, #e74c3c, #c0392b);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  font-size: 16px;
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(231, 76, 60, 0.3);
}

.btn-launch-game:hover:not(:disabled) {
  background: linear-gradient(135deg, #c0392b, #a93226);
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(231, 76, 60, 0.4);
}

.btn-launch-game:disabled {
  background: #bdc3c7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.launch-help {
  color: #7f8c8d;
  font-size: 12px;
  text-align: center;
}

.edit-actions {
  display: flex;
  gap: 10px;
}

.btn-save, .btn-cancel {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
}

.btn-save {
  background-color: #27ae60;
  color: white;
}

.btn-cancel {
  background-color: #e74c3c;
  color: white;
}

.empty-state {
  text-align: center;
  padding: 40px;
  color: #7f8c8d;
}

.empty-state p {
  margin: 0 0 5px 0;
  font-size: 16px;
}

.empty-state small {
  font-size: 12px;
}

.patches-list, .backups-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 20px;
}

.patch-item, .backup-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 15px;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.patch-item:hover, .backup-item:hover {
  border-color: #3498db;
  background: #f8f9fa;
}

.patch-item.selected, .backup-item.selected {
  border-color: #3498db;
  background: #e3f2fd;
}

.patch-info, .backup-info {
  flex: 1;
}

.patch-name, .backup-name {
  display: block;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 4px;
}

.patch-path, .backup-path {
  display: block;
  font-size: 12px;
  color: #7f8c8d;
  font-family: 'Consolas', 'Monaco', monospace;
}

.install-actions, .rollback-actions {
  display: flex;
  justify-content: center;
}

.btn-install, .btn-rollback {
  padding: 12px 30px;
  border: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-install {
  background-color: #27ae60;
  color: white;
}

.btn-rollback {
  background-color: #f39c12;
  color: white;
}

.btn-install:disabled, .btn-rollback:disabled {
  background-color: #bdc3c7;
  cursor: not-allowed;
}

.btn-install:not(:disabled):hover, .btn-rollback:not(:disabled):hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.2);
}

.btn-edit:hover, .btn-browse:hover, .btn-save:hover, .btn-cancel:hover, .btn-reset:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}
</style>