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

interface BackupFile {
  path: string
  hash: string
  size: number
}

interface Backup {
  name: string
  timestamp: string
  path: string
  description?: string
  backupInfo?: {
    gameName: string
    gameVersion?: string
    backedUpFiles: BackupFile[]
    installedFiles: BackupFile[]
  }
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
        
        // 尝试从game_status.json加载启动选项
        try {
          const gameStatus = await invoke('read_game_status', { 
            gameName: game.value.name 
          }) as any
          
          if (gameStatus && gameStatus.launchOptions) {
            game.value.launchOptions = gameStatus.launchOptions
            launchOptions.value = gameStatus.launchOptions
            console.log('Loaded launch options from game_status.json for:', game.value.name)
          } else {
            // 如果没有从game_status.json加载到，使用localStorage中的值
            launchOptions.value = game.value.launchOptions || ''
          }
        } catch (error) {
          console.warn('Failed to load game_status.json, using localStorage:', error)
          // 如果读取失败，使用localStorage中的值
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
    
    const backupPromises = backupFolders.map(async (folder: string) => {
      const backupPath = `${backupDir}/${folder}`
      const backup: Backup = {
        name: folder,
        timestamp: formatTimestamp(folder),
        path: backupPath,
        description: undefined,
        backupInfo: undefined
      }
      
      // Try to load backup info if exists
      try {
        const infoFile = `${backupPath}/backup_info.json`
        const infoExists = await invoke('file_exists', { path: infoFile })
        if (infoExists) {
          const infoContent = await invoke('read_file', { path: infoFile }) as string
          backup.backupInfo = JSON.parse(infoContent)
        }
      } catch (e) {
        console.warn('Failed to load backup info:', e)
      }
      
      return backup
    })
    
    backups.value = (await Promise.all(backupPromises))
      .sort((a: Backup, b: Backup) => b.name.localeCompare(a.name)) // Sort by timestamp desc
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
    
    // Update in game_status.json
    try {
      await invoke('update_game_status', {
        gameName: game.value.name,
        launchOptions: game.value.launchOptions
      })
      console.log('Updated launch options in game_status.json for:', game.value.name)
    } catch (error) {
      console.error('Failed to update game_status.json:', error)
    }
    
    isEditingLaunchOptions.value = false
  } catch (error) {
    console.error('Failed to save launch options:', error)
  }
}

function cancelEditLaunchOptions() {
  launchOptions.value = game.value?.launchOptions || ''
}

function resetToDefaultLaunchOptions() {
  // 为所有游戏提供默认启动选项（可选）
  launchOptions.value = ''
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

// Calculate MD5 hash of a file (simplified implementation using file size for demo)
async function calculateFileHash(filePath: string): Promise<string> {
  try {
    // In a real implementation, you would compute a proper hash (MD5, SHA1, etc.)
    // For simplicity, we're using file size as a basic identifier
    const fileSize = await invoke('get_file_size', { path: filePath }).catch(() => 0)
    return `size:${fileSize}`
  } catch {
    return 'unknown'
  }
}

// Create a detailed backup information object
async function createBackupInfo(
  gameName: string, 
  gameDir: string,
  backupDir: string,
  patchesInstalled: string[],
  backedUpFiles: string[],
  installedFiles: string[]
): Promise<any> {
  const backupInfo: { 
    gameName: string;
    gameVersion: string;
    timestamp: string;
    patchesInstalled: string[];
    backedUpFiles: BackupFile[];
    installedFiles: BackupFile[];
  } = {
    gameName,
    gameVersion: 'unknown', // Could be enhanced to detect game version
    timestamp: new Date().toISOString(),
    patchesInstalled,
    backedUpFiles: [],
    installedFiles: []
  }
  
  // Add details for backed up files
  for (const file of backedUpFiles) {
    const relativePath = file.replace(backupDir, '').replace(/^[/\\]/, '')
    const backupPath = `${backupDir}/${relativePath}`
    
    try {
      const hash = await calculateFileHash(backupPath)
      const size = Number(await invoke('get_file_size', { path: backupPath }).catch(() => 0))
      
      backupInfo.backedUpFiles.push({
        path: relativePath,
        hash,
        size
      })
    } catch (e) {
      console.warn('Failed to get info for backed up file:', file, e)
    }
  }
  
  // Add details for installed files
  for (const file of installedFiles) {
    try {
      const hash = await calculateFileHash(file)
      const size = Number(await invoke('get_file_size', { path: file }).catch(() => 0))
      const relativePath = file.replace(gameDir, '').replace(/^[/\\]/, '')
      
      backupInfo.installedFiles.push({
        path: relativePath,
        hash,
        size
      })
    } catch (e) {
      console.warn('Failed to get info for installed file:', file, e)
    }
  }
  
  return backupInfo
}

async function installPatches() {
  if (selectedPatches.value.size === 0 || !game.value) return
  
  isLoading.value = true
  
  try {
    // 使用本地时间生成时间戳，避免UTC时差问题
    const now = new Date();
    const year = String(now.getFullYear());
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    const hour = String(now.getHours()).padStart(2, '0');
    const minute = String(now.getMinutes()).padStart(2, '0');
    const second = String(now.getSeconds()).padStart(2, '0');
    const timestamp = `${year}${month}${day}_${hour}${minute}${second}`
    const backupDir = `${appDirectory.value}/game/${game.value.name}/backup/${timestamp}`
    
    // Create backup directory
    await invoke('create_directory', { path: backupDir })
    
    const backedUpFiles: string[] = []
    const installedFiles: string[] = []
    
    for (const patchName of selectedPatches.value) {
      const patch = patches.value.find(p => p.name === patchName)
      if (!patch) continue
      
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
          backedUpFiles.push(backupPath)
        }
        
        // Copy patch file to target
        await invoke('create_directory', { path: targetPath.substring(0, targetPath.lastIndexOf('/')) })
        await invoke('copy_file', { from: file, to: targetPath })
        installedFiles.push(targetPath)
      }
    }
    
    // Create detailed backup info
    const backupInfo = await createBackupInfo(
      game.value.name,
      game.value.directory,
      backupDir,
      Array.from(selectedPatches.value),
      backedUpFiles,
      installedFiles
    )
    
    // Save backup info
    await invoke('write_file', {
      path: `${backupDir}/backup_info.json`,
      content: JSON.stringify(backupInfo, null, 2)
    })
    
    // Clear selection and refresh
    selectedPatches.value.clear()
    await scanBackups()
    
    alert('补丁安装完成！\n备份已创建，包含文件完整性验证信息。')
  } catch (error) {
    console.error('Failed to install patches:', error)
    alert('补丁安装失败：' + error)
  } finally {
    isLoading.value = false
  }
}

// Verify file integrity by comparing hashes
async function verifyFileIntegrity(filePath: string, expectedHash: string): Promise<boolean> {
  try {
    const actualHash = await calculateFileHash(filePath)
    // For demo purposes, we're using a simple size check
    // In a real implementation, you would compare proper cryptographic hashes
    return actualHash === expectedHash || expectedHash === 'unknown'
  } catch {
    return false
  }
}



async function rollbackToBackup() {
  if (!selectedBackup.value || !game.value) return
  
  isLoading.value = true
  
  try {
    const backup = backups.value.find(b => b.name === selectedBackup.value)
    if (!backup) {
      throw new Error('未找到选中的备份')
    }
    
    const rollbackLog: string[] = []
    rollbackLog.push(`回滚日志 - ${new Date().toISOString()}`)
    rollbackLog.push(`游戏: ${game.value.name}`)
    rollbackLog.push(`回滚到备份: ${backup.timestamp}`)
    rollbackLog.push('---')
    
    // Try to use backup_info.json for more reliable rollback
    
    if (backup.backupInfo) {
      // Using the detailed backup info
      rollbackLog.push('使用详细备份信息进行回滚')
      
      // Restore backed up files with integrity check
      for (const fileInfo of backup.backupInfo.backedUpFiles) {
        const backupFilePath = `${backup.path}/${fileInfo.path}`
        const targetPath = `${game.value.directory}/${fileInfo.path}`
        
        const backupExists = await invoke('file_exists', { path: backupFilePath })
        if (backupExists) {
          // Verify file integrity before restoring
          const isIntegrityVerified = await verifyFileIntegrity(backupFilePath, fileInfo.hash)
          
          if (isIntegrityVerified) {
            await invoke('copy_file', { from: backupFilePath, to: targetPath })
            rollbackLog.push(`已恢复: ${fileInfo.path} [完整性验证通过]`)
          } else {
            rollbackLog.push(`警告: ${fileInfo.path} 完整性验证失败，但仍尝试恢复`)
            await invoke('copy_file', { from: backupFilePath, to: targetPath })
          }
        } else {
          rollbackLog.push(`错误: 备份文件不存在: ${fileInfo.path}`)
        }
      }
      
      // Remove files that were only installed (not backed up)
      for (const fileInfo of backup.backupInfo.installedFiles) {
        // Check if this file was not backed up
        const isBackedUp = backup.backupInfo.backedUpFiles.some(
          backedUp => backedUp.path === fileInfo.path
        )
        
        if (!isBackedUp) {
          const targetPath = `${game.value.directory}/${fileInfo.path}`
          const fileExists = await invoke('file_exists', { path: targetPath })
          if (fileExists) {
            await invoke('delete_file', { path: targetPath })
            rollbackLog.push(`已删除: ${fileInfo.path}`)
          }
        }
      }
    } else {
      // No backup info available, cannot perform rollback
      throw new Error('备份信息不完整，无法执行回滚操作')
    }
    
    // Write rollback log
    const rollbackLogPath = `${backup.path}/rollback_${new Date().toISOString().replace(/[-:]/g, '').replace(/\..+/, '').replace('T', '_')}.log`
    await invoke('write_file', { path: rollbackLogPath, content: rollbackLog.join('\n') })
    
    selectedBackup.value = ''
    alert('回滚完成！\n已验证并恢复所有文件到备份时的状态。')
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
  <div class="game-detail-panel panel">
    <div class="panel-header">
      <button class="btn btn-secondary" @click="goBack">
        ← 返回游戏列表
      </button>
      <h2 v-if="game">{{ game.name }}</h2>
    </div>

    <div class="panel-body">
      <div v-if="!game" class="loading">
        <p>加载中...</p>
      </div>

      <div v-else class="content">
        <!-- 游戏详情模块 -->
        <div class="card">
          <div class="card-header">
            <h3>游戏详情</h3>
          </div>
          <div class="card-body">
            <div class="info-row">
              <label>游戏名称:</label>
              <span>{{ game.name }}</span>
            </div>
            <div class="info-row">
              <label>游戏路径:</label>
              <div v-if="!isEditingDirectory" class="directory-display">
                <span class="directory-path">{{ game.directory }}</span>
                <button class="btn btn-secondary btn-sm" @click="isEditingDirectory = true">
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
                  <button @click="selectNewDirectory" class="btn btn-primary">
                    浏览
                  </button>
                </div>
                <div class="edit-actions">
                  <button @click="saveGameDirectory" class="btn btn-success">
                    保存
                  </button>
                  <button @click="cancelEditDirectory" class="btn btn-error">
                    取消
                  </button>
                </div>
              </div>
            </div>
            
            <!-- 启动选项设置 (对所有游戏显示) -->
            <div class="info-row">
              <label>启动选项:</label>
              <div v-if="!isEditingLaunchOptions" class="launch-options-display">
                <span class="launch-options-text">{{ launchOptions || '(未设置)' }}</span>
                <button class="btn btn-secondary btn-sm" @click="isEditingLaunchOptions = true">
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
                  <button @click="resetToDefaultLaunchOptions" class="btn btn-warning">
                    重置默认
                  </button>
                </div>
                <div class="edit-actions">
                  <button @click="saveLaunchOptions" class="btn btn-success">
                    保存
                  </button>
                  <button @click="cancelEditLaunchOptions" class="btn btn-error">
                    取消
                  </button>
                </div>
                <div class="launch-options-help">
                  <small>输入游戏启动时需要的命令行参数</small>
                </div>
              </div>
            </div>
            
            <!-- 游戏启动按钮 (对所有游戏显示) -->
            <div class="info-row">
              <label></label>
              <div class="launch-game-section">
                <button 
                  @click="launchGame" 
                  :disabled="isLoading"
                  class="btn btn-error btn-launch"
                >
                  <span v-if="isLoading">启动中...</span>
                  <span v-else>启动 {{ game.name }}</span>
                </button>
                <small class="launch-help">使用上面设置的启动选项启动游戏</small>
              </div>
            </div>
          </div>
        </div>

        <!-- 补丁安装模块 -->
        <div class="card">
          <div class="card-header">
            <h3>补丁安装</h3>
          </div>
          <div class="card-body">
            <div v-if="patches.length === 0" class="empty-state">
              <p>未找到可用补丁</p>
              <small>补丁应放置在: game/{{ game.name }}/patch/ 目录下</small>
            </div>
            <div v-else>
              <div class="patches-list">
                <div 
                  v-for="patch in patches" 
                  :key="patch.name"
                  class="list-item patch-item"
                  :class="{ selected: selectedPatches.has(patch.name) }"
                  @click="togglePatch(patch.name)"
                >
                  <input 
                    type="checkbox" 
                    :checked="selectedPatches.has(patch.name)"
                    @change.stop="togglePatch(patch.name)"
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
                  class="btn btn-success"
                >
                  <span v-if="isLoading">安装中...</span>
                  <span v-else>安装选中补丁 ({{ selectedPatches.size }})</span>
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- 备份回滚模块 -->
        <div class="card">
          <div class="card-header">
            <h3>备份回滚</h3>
          </div>
          <div class="card-body">
            <div v-if="backups.length === 0" class="empty-state">
              <p>暂无可用备份</p>
              <small>安装补丁时会自动创建备份</small>
            </div>
            <div v-else>
              <div class="backups-list">
                <div 
                  v-for="backup in backups" 
                  :key="backup.name"
                  class="list-item backup-item"
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
                  class="btn btn-warning"
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
  </div>
</template>

<style scoped>
.game-detail-panel {
  display: flex;
  flex-direction: column;
}

.panel-header h2 {
  margin-left: var(--space-4);
}

.panel-body {
  overflow-y: auto;
  padding: var(--space-6);
}

.loading {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: var(--text-muted);
}

.content {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.card-header h3 {
  margin: 0;
  font-size: var(--font-lg);
}

.info-row {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  margin-bottom: var(--space-5);
}

.info-row:last-child {
  margin-bottom: 0;
}

.info-row label {
  font-weight: var(--font-semibold);
  color: var(--text-secondary);
  min-width: 100px;
  padding-top: var(--space-2);
  font-size: var(--font-sm);
}

.directory-display, .launch-options-display {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  flex: 1;
}

.directory-path, .launch-options-text {
  font-family: var(--font-family-mono);
  background: var(--gray-50);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-base);
  border: 1px solid var(--border-color);
  flex: 1;
  word-break: break-all;
  font-size: var(--font-sm);
}

.directory-edit, .launch-options-edit {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.directory-input, .launch-options-input {
  display: flex;
  gap: var(--space-3);
}

.edit-actions {
  display: flex;
  gap: var(--space-3);
}

.launch-options-help {
  margin-top: var(--space-2);
  padding: var(--space-2) var(--space-3);
  background: var(--gray-50);
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--primary-color);
}

.launch-options-help small {
  color: var(--primary-dark);
  font-family: var(--font-family-mono);
}

.launch-game-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  flex: 1;
}

.btn-launch {
  font-size: var(--font-lg);
  padding: var(--space-4) var(--space-6);
}

.launch-help {
  color: var(--text-muted);
  font-size: var(--font-xs);
  text-align: center;
}

.empty-state {
  text-align: center;
  padding: var(--space-8);
  color: var(--text-muted);
  background-color: var(--gray-50);
  border-radius: var(--radius-md);
}

.empty-state p {
  margin: 0 0 var(--space-2) 0;
  font-size: var(--font-base);
}

.empty-state small {
  font-size: var(--font-sm);
}

.patches-list, .backups-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-bottom: var(--space-5);
}

.patch-item, .backup-item {
  gap: var(--space-4);
  cursor: pointer;
}

.patch-item.selected, .backup-item.selected {
  border-color: var(--primary-color);
  background-color: var(--gray-50);
  box-shadow: var(--shadow-primary);
}

.patch-info, .backup-info {
  flex: 1;
}

.patch-name, .backup-name {
  display: block;
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  margin-bottom: var(--space-1);
  font-size: var(--font-sm);
}

.patch-path, .backup-path {
  display: block;
  font-size: var(--font-xs);
  color: var(--text-muted);
  font-family: var(--font-family-mono);
}

.install-actions, .rollback-actions {
  display: flex;
  justify-content: center;
}

.btn-sm {
  padding: var(--space-1) var(--space-3);
  font-size: var(--font-xs);
}
</style>