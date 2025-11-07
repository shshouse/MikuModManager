<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { searchGames, fetchGameById } from '../../../utils/supabase'
import type { MikuGameListItem } from '../../../types/mikugame'

interface CustomGame {
  id: string
  name: string
  directory: string
  icon?: string
  lastPlayed?: Date
  playTime?: number
  launchOptions?: string
  mikuGameId?: string
  mikuGameType?: 'games' | 'h_games' | 'galgames'
  images?: string[]
  coverImage?: string
}

// 模组管理相关接口
interface ModConfig {
  id: string
  name: string
  version: string
  author: string
  description: string
  game: string
  installPath: string
  files: string[]
  dependencies: string[]
  conflicts: string[]
  installDate?: string
  installed?: boolean
  enabled?: boolean
  hasConflicts?: boolean
}

interface InstallLog {
  modId: string
  modName: string
  installDate: string
  installedFiles: string[]
  replacedFiles: string[]
}

interface FileMapping {
  [filePath: string]: string[]
}

interface GameStatus {
  game_name: string
  game_path: string
  launch_options: string
  installed_mods: string[]
  play_time: number  // 游玩时长，单位：秒
  last_updated: string
}


const props = defineProps<{
  gameId: string
}>()

const game = ref<CustomGame | null>(null)
const isLoading = ref(false)
const newGameDirectory = ref('')
const isEditingDirectory = ref(false)
const launchOptions = ref('')
const isEditingLaunchOptions = ref(false)

// MikuGame 绑定相关
const showMikuGameSearch = ref(false)
const mikuGameSearchKeyword = ref('')
const mikuGameSearchResults = ref<MikuGameListItem[]>([])
const selectedMikuGame = ref<MikuGameListItem | null>(null)
const isSearchingMikuGame = ref(false)
const isEditingBinding = ref(false)

const appDirectory = ref('')
const backgroundImage = ref('')

// 模组管理相关状态
const mods = ref<ModConfig[]>([])
const isApplyingMods = ref(false)

// 计算是否有模组更改
const hasModChanges = computed(() => {
  return mods.value.some(mod => mod.installed !== mod.enabled)
})

// 获取背景图片（从游戏图片中随机选择，如果没有则显示默认图）
const heroBackground = computed(() => {
  if (backgroundImage.value) {
    return backgroundImage.value
  }
  return '/Miku.png' // 默认背景
})

// 随机选择背景图片
function selectRandomBackground() {
  if (game.value?.images && game.value.images.length > 0) {
    // 如果游戏有图片，从游戏图片中随机选择
    const randomIndex = Math.floor(Math.random() * game.value.images.length)
    backgroundImage.value = game.value.images[randomIndex]
  } else {
    // 否则显示默认图片
    backgroundImage.value = '/MikuModManager.png'
  }
}

onMounted(async () => {
  await loadAppDirectory()
  await loadGame()
  await loadMods()
  selectRandomBackground()
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
          // 构建正确的game目录路径
          const gameManagerPath = `${appDirectory.value}/game/${game.value.name}`
          const gameStatusContent = await invoke('read_game_status', { 
            gamePath: gameManagerPath
          }) as string
          
          const gameStatus = JSON.parse(gameStatusContent)
          
          if (gameStatus && gameStatus.launch_options) {
            game.value.launchOptions = gameStatus.launch_options
            launchOptions.value = gameStatus.launch_options
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
      const gameManagerPath = `${appDirectory.value}/game/${game.value.name}`
      await invoke('update_game_status', {
        gamePath: gameManagerPath,
        launchOptions: game.value.launchOptions,
        installedMods: null
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


// MikuGame 绑定功能
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

function selectMikuGame(mikuGame: MikuGameListItem) {
  selectedMikuGame.value = mikuGame
  showMikuGameSearch.value = false
}

function clearMikuGameSelection() {
  selectedMikuGame.value = null
  mikuGameSearchKeyword.value = ''
  mikuGameSearchResults.value = []
}

function startEditBinding() {
  isEditingBinding.value = true
  showMikuGameSearch.value = false
  selectedMikuGame.value = null
  mikuGameSearchKeyword.value = ''
  mikuGameSearchResults.value = []
}

function cancelEditBinding() {
  isEditingBinding.value = false
  showMikuGameSearch.value = false
  clearMikuGameSelection()
}

async function saveBinding() {
  if (!game.value) return
  
  try {
    // 更新绑定信息
    if (selectedMikuGame.value) {
      game.value.mikuGameId = selectedMikuGame.value.id
      game.value.mikuGameType = selectedMikuGame.value.game_type
      
      // 获取并更新图片URL
      try {
        const mikuGameData = await fetchGameById(selectedMikuGame.value.id, selectedMikuGame.value.game_type)
        if (mikuGameData && mikuGameData.image_urls && mikuGameData.image_urls.length > 0) {
          game.value.images = mikuGameData.image_urls
          game.value.coverImage = mikuGameData.image_urls[0]
          console.log('已更新游戏图片URL:', mikuGameData.image_urls)
        }
      } catch (error) {
        console.error('获取MikuGame图片URL失败:', error)
      }
    } else {
      // 解除绑定
      game.value.mikuGameId = undefined
      game.value.mikuGameType = undefined
    }
    
    // 保存到 localStorage
    const saved = localStorage.getItem('customGames')
    if (saved) {
      const games: CustomGame[] = JSON.parse(saved)
      const index = games.findIndex(g => g.id === props.gameId)
      if (index > -1) {
        games[index] = game.value
        localStorage.setItem('customGames', JSON.stringify(games))
      }
    }
    
    isEditingBinding.value = false
    clearMikuGameSelection()
    
    // 重新选择背景图（如果更新了游戏图片）
    selectRandomBackground()
    
    alert('绑定已更新')
  } catch (error) {
    console.error('保存绑定失败:', error)
    alert('保存失败: ' + error)
  }
}

async function unbindMikuGame() {
  if (!game.value) return
  
  if (!confirm('确定要解除 MikuGame 绑定吗？')) {
    return
  }
  
  try {
    game.value.mikuGameId = undefined
    game.value.mikuGameType = undefined
    
    // 保存到 localStorage
    const saved = localStorage.getItem('customGames')
    if (saved) {
      const games: CustomGame[] = JSON.parse(saved)
      const index = games.findIndex(g => g.id === props.gameId)
      if (index > -1) {
        games[index] = game.value
        localStorage.setItem('customGames', JSON.stringify(games))
      }
    }
    
    alert('已解除绑定')
    } catch (error) {
    console.error('解除绑定失败:', error)
    alert('操作失败: ' + error)
  }
}

// =========================
// 模组管理功能
// =========================

// 加载所有模组
async function loadMods() {
  if (!game.value) return
  
  try {
    // 从 game/{游戏名}/mods/ 目录读取模组
    const modsDir = `${appDirectory.value}/game/${game.value.name}/mods`
    
    // 检查目录是否存在
    const modsDirExists = await invoke('file_exists', { path: modsDir }) as boolean
    if (!modsDirExists) {
      console.warn(`模组目录不存在: ${modsDir}`)
      mods.value = []
      return
    }
    
    const modFolders = await invoke('scan_directory', { path: modsDir }) as string[]
    
    // 读取 game_status.json，获取已安装模组列表
    let installedMods: Set<string> = new Set()
    
    try {
      const gameStatusPath = `${appDirectory.value}/game/${game.value.name}/game_status.json`
      const statusExists = await invoke('file_exists', { path: gameStatusPath }) as boolean
      
      if (statusExists) {
        const statusContent = await invoke('read_file', { path: gameStatusPath }) as string
        const gameStatus = JSON.parse(statusContent) as GameStatus
        installedMods = new Set(gameStatus.installed_mods || [])
      }
    } catch (error) {
      console.error('读取游戏状态失败:', error)
    }
    
    const modPromises = modFolders.map(async (folder: string) => {
      const modPath = `${modsDir}/${folder}`
      const configPath = `${modPath}/_mikumodinfo.json`
      
      try {
        const configExists = await invoke('file_exists', { path: configPath }) as boolean
        let config: ModConfig
        
        if (!configExists) {
          
          config = {
            id: folder,
            name: `${folder} (非MikuMod模组)`,
            version: '未知',
            author: '未知',
            description: '此模组文件夹未包含配置信息',
            game: game.value?.name || '',
            installPath: game.value?.directory || '',
            files: [],
            dependencies: [],
            conflicts: [],
            installed: false,
            enabled: false
          }
        } else {
          const configContent = await invoke('read_file', { path: configPath }) as string
          config = JSON.parse(configContent) as ModConfig
          config.id = folder
        }
        
        const allFiles = await invoke('get_all_files', { path: modPath }) as string[]
        config.files = allFiles
          .map(file => file.replace(modPath, '').replace(/^[/\\]/, ''))
          .filter(file => file !== '_mikumodinfo.json')
        
        config.installed = installedMods.has(folder)
        config.enabled = config.installed
        
        return config
      } catch (error) {
        console.error(`加载模组 ${folder} 失败:`, error)
        return null
      }
    })
    
    const loadedMods = (await Promise.all(modPromises)).filter(mod => mod !== null)
    mods.value = loadedMods
    
  } catch (error) {
    console.error('加载模组列表失败:', error)
    mods.value = []
  }
}

// 应用模组更改
async function applyModChanges() {
  if (!game.value) return
  
  isApplyingMods.value = true
  
  try {
    const modsToUninstall = mods.value.filter(m => m.installed && !m.enabled)
    const modsToInstall = mods.value.filter(m => !m.installed && m.enabled)
    
    console.log('需要卸载:', modsToUninstall.map(m => m.name))
    console.log('需要安装:', modsToInstall.map(m => m.name))
    
    if (modsToUninstall.length === 0 && modsToInstall.length === 0) {
      alert('没有需要应用的更改')
      return
    }
    
    // 步骤1: 先执行所有卸载
    if (modsToUninstall.length > 0) {
      for (const mod of modsToUninstall) {
        await uninstallMod(mod)
      }
    }
    
    // 步骤2: 检测待安装模组间的文件冲突
    if (modsToInstall.length > 1) {
      const conflictResult = await detectInstallConflicts(modsToInstall)
      if (conflictResult.hasConflict) {
        const selectedMod = await showModSelectionDialog(conflictResult.conflictingMods, conflictResult.conflictingFiles)
        if (!selectedMod) {
          alert('安装已取消')
          isApplyingMods.value = false
          return
        }
        
        await installModInternal(selectedMod)
      } else {
        for (const mod of modsToInstall) {
          await installModInternal(mod)
        }
      }
    } else if (modsToInstall.length === 1) {
      await installModInternal(modsToInstall[0])
    }
    
    await loadMods()
    alert('更改已成功应用！')
    
  } catch (error) {
    console.error('应用更改失败:', error)
    alert(`应用更改失败: ${error}`)
  } finally {
    isApplyingMods.value = false
  }
}

// 安装模组（内部方法）
async function installModInternal(mod: ModConfig) {
  if (!game.value) return
  
  try {
    const gameDir = `${appDirectory.value}/game/${game.value.name}`
    const installLogDir = `${gameDir}/installLog`
    const backupDir = `${gameDir}/backup`
    
    await invoke('create_directory', { path: installLogDir })
    await invoke('create_directory', { path: backupDir })
    
    const installLog: InstallLog = {
      modId: mod.id,
      modName: mod.name,
      installDate: new Date().toISOString(),
      installedFiles: [],
      replacedFiles: []
    }
    
    const modFilesPath = `${gameDir}/mods/${mod.id}`
    
    const { hasConflict, conflictingMods, conflictingFiles } = await checkConflictWithInstalled(mod)
    
    if (hasConflict) {
      const confirmed = confirm(
        `警告：安装模组 "${mod.name}" 将会影响以下已安装模组的效果：\n\n` +
        `${conflictingMods.map(m => `- ${m}`).join('\n')}\n\n` +
        `冲突文件：\n${conflictingFiles.map(f => `- ${f}`).join('\n')}\n\n` +
        `是否继续安装？`
      )
      
      if (!confirmed) {
        console.log('用户取消安装')
        return
      }
    }
    
    for (const file of mod.files) {
      const sourcePath = `${modFilesPath}/${file}`
      const targetPath = `${mod.installPath}/${file}`
      const normalizedTargetPath = targetPath.replace(/\\/g, '/')
      
      const targetExists = await invoke('file_exists', { path: targetPath }) as boolean
      
      if (targetExists) {
        const backupPath = `${backupDir}/${file}`
        const backupExists = await invoke('file_exists', { path: backupPath }) as boolean
        
        if (!backupExists) {
          const backupDirPath = backupPath.substring(0, backupPath.lastIndexOf('/'))
          await invoke('create_directory', { path: backupDirPath })
          await invoke('copy_file', { from: targetPath, to: backupPath })
          installLog.replacedFiles.push(normalizedTargetPath)
          console.log(`备份游戏原文件: ${targetPath} -> ${backupPath}`)
        }
      }
      
      const targetDirPath = targetPath.substring(0, targetPath.lastIndexOf('/'))
      if (targetDirPath) {
        await invoke('create_directory', { path: targetDirPath })
      }
      
      await invoke('copy_file', { from: sourcePath, to: targetPath })
      installLog.installedFiles.push(normalizedTargetPath)
      console.log(`安装文件: ${sourcePath} -> ${targetPath}`)
    }
    
    const logPath = `${installLogDir}/${mod.id}.json`
    await invoke('write_file', {
      path: logPath,
      content: JSON.stringify(installLog, null, 2)
    })
    
    await updateFileMapping(game.value.name, mod.id, mod.files.map(f => `${mod.installPath}/${f}`.replace(/\\/g, '/')), 'add')
    await updateGameStatus(game.value.name, mod.id, 'install')
    
    console.log(`模组 "${mod.name}" 安装成功`)
    
  } catch (error) {
    console.error('安装模组失败:', error)
    throw error
  }
}

// 卸载模组
async function uninstallMod(mod: ModConfig) {
  if (!game.value) return
  
  try {
    const gameDir = `${appDirectory.value}/game/${game.value.name}`
    const installLogPath = `${gameDir}/installLog/${mod.id}.json`
    const backupDir = `${gameDir}/backup`
    
    const logExists = await invoke('file_exists', { path: installLogPath }) as boolean
    if (!logExists) {
      console.warn(`未找到安装日志: ${installLogPath}`)
      return
    }
    
    const logContent = await invoke('read_file', { path: installLogPath }) as string
    const installLog: InstallLog = JSON.parse(logContent)
    
    for (const filePath of installLog.installedFiles) {
      const fileExists = await invoke('file_exists', { path: filePath }) as boolean
          if (fileExists) {
        await invoke('delete_file', { path: filePath })
        console.log(`删除文件: ${filePath}`)
      }
    }
    
    for (const filePath of installLog.replacedFiles) {
      const relativePath = filePath.replace(mod.installPath + '/', '')
      const backupPath = `${backupDir}/${relativePath}`
      
      const backupExists = await invoke('file_exists', { path: backupPath }) as boolean
      if (backupExists) {
        await invoke('copy_file', { from: backupPath, to: filePath })
        console.log(`恢复备份文件: ${backupPath} -> ${filePath}`)
      }
    }
    
    await invoke('delete_file', { path: installLogPath })
    await updateFileMapping(game.value.name, mod.id, installLog.installedFiles, 'remove')
    await updateGameStatus(game.value.name, mod.id, 'uninstall')
    
    console.log(`模组 "${mod.name}" 卸载成功`)
    
  } catch (error) {
    console.error('卸载模组失败:', error)
    throw error
  }
}

// 更新文件映射
async function updateFileMapping(gameName: string, modId: string, files: string[], action: 'add' | 'remove') {
  try {
    const mappingPath = `${appDirectory.value}/game/${gameName}/installLog/file_mapping.json`
    
    let mapping: FileMapping = {}
    const mappingExists = await invoke('file_exists', { path: mappingPath }) as boolean
    
    if (mappingExists) {
      const mappingContent = await invoke('read_file', { path: mappingPath }) as string
      mapping = JSON.parse(mappingContent)
    }
    
    for (const file of files) {
      const normalizedFile = file.replace(/\\/g, '/')
      
      if (action === 'add') {
        if (!mapping[normalizedFile]) {
          mapping[normalizedFile] = []
        }
        if (!mapping[normalizedFile].includes(modId)) {
          mapping[normalizedFile].push(modId)
        }
      } else {
        if (mapping[normalizedFile]) {
          mapping[normalizedFile] = mapping[normalizedFile].filter(id => id !== modId)
          if (mapping[normalizedFile].length === 0) {
            delete mapping[normalizedFile]
          }
        }
      }
    }
    
    await invoke('write_file', {
      path: mappingPath,
      content: JSON.stringify(mapping, null, 2)
    })
    
  } catch (error) {
    console.error('更新文件映射失败:', error)
  }
}

// 更新 game_status.json
async function updateGameStatus(gameName: string, modId: string, action: 'install' | 'uninstall') {
  try {
    const gameStatusPath = `${appDirectory.value}/game/${gameName}/game_status.json`
    
    const statusExists = await invoke('file_exists', { path: gameStatusPath }) as boolean
    
    if (!statusExists) {
      console.warn(`game_status.json 不存在: ${gameStatusPath}`)
      return
    }
    
    const statusContent = await invoke('read_file', { path: gameStatusPath }) as string
    const gameStatus = JSON.parse(statusContent) as GameStatus
    
    if (!gameStatus.installed_mods) {
      gameStatus.installed_mods = []
    }
    
    if (action === 'install') {
      if (!gameStatus.installed_mods.includes(modId)) {
        gameStatus.installed_mods.push(modId)
      }
    } else {
      gameStatus.installed_mods = gameStatus.installed_mods.filter(id => id !== modId)
    }
    
    gameStatus.last_updated = new Date().toISOString()
    
    await invoke('write_file', {
      path: gameStatusPath,
      content: JSON.stringify(gameStatus, null, 2)
    })
    
    console.log(`已更新 game_status.json: ${action} ${modId}`)
    
  } catch (error) {
    console.error('更新游戏状态失败:', error)
  }
}

// 检测待安装模组间的冲突
async function detectInstallConflicts(modsToInstall: ModConfig[]): Promise<{
  hasConflict: boolean
  conflictingMods: ModConfig[]
  conflictingFiles: string[]
}> {
  const fileToMods = new Map<string, ModConfig[]>()
  
  for (const mod of modsToInstall) {
    for (const file of mod.files) {
      const targetPath = `${mod.installPath}/${file}`.replace(/\\/g, '/')
      
      if (!fileToMods.has(targetPath)) {
        fileToMods.set(targetPath, [])
      }
      fileToMods.get(targetPath)!.push(mod)
    }
  }
  
  const conflictingFiles: string[] = []
  const conflictingModsSet = new Set<ModConfig>()
  
  for (const [filePath, modList] of fileToMods.entries()) {
    if (modList.length > 1) {
      conflictingFiles.push(filePath)
      modList.forEach(mod => conflictingModsSet.add(mod))
    }
  }
  
  return {
    hasConflict: conflictingFiles.length > 0,
    conflictingMods: Array.from(conflictingModsSet),
    conflictingFiles
  }
}

// 检测与已安装模组的冲突
async function checkConflictWithInstalled(mod: ModConfig): Promise<{
  hasConflict: boolean
  conflictingMods: string[]
  conflictingFiles: string[]
}> {
  if (!game.value) return { hasConflict: false, conflictingMods: [], conflictingFiles: [] }
  
  const conflictingFiles: string[] = []
  const conflictingMods: string[] = []
  
  try {
    const mappingPath = `${appDirectory.value}/game/${game.value.name}/installLog/file_mapping.json`
    
    const mappingExists = await invoke('file_exists', { path: mappingPath }) as boolean
    if (!mappingExists) {
      return { hasConflict: false, conflictingMods: [], conflictingFiles: [] }
    }
    
    const mappingContent = await invoke('read_file', { path: mappingPath }) as string
    const mapping: FileMapping = JSON.parse(mappingContent)
    
    for (const file of mod.files) {
      const targetPath = `${mod.installPath}/${file}`.replace(/\\/g, '/')
      
      if (mapping[targetPath] && mapping[targetPath].length > 0) {
        conflictingFiles.push(file)
        
        for (const conflictModId of mapping[targetPath]) {
          const conflictMod = mods.value.find(m => m.id === conflictModId)
          if (conflictMod && !conflictingMods.includes(conflictMod.name)) {
            conflictingMods.push(conflictMod.name)
          }
        }
      }
    }
    
  } catch (error) {
    console.error('检查冲突失败:', error)
  }
  
  return {
    hasConflict: conflictingFiles.length > 0,
    conflictingMods,
    conflictingFiles
  }
}

// 显示模组选择对话框
async function showModSelectionDialog(
  conflictingMods: ModConfig[],
  conflictingFiles: string[]
): Promise<ModConfig | null> {
  return new Promise((resolve) => {
    const dialog = document.createElement('div')
    dialog.className = 'conflict-dialog-overlay'
    dialog.innerHTML = `
      <div class="conflict-dialog">
        <div class="conflict-dialog-header">
          <h3>⚠️ 模组冲突检测</h3>
        </div>
        <div class="conflict-dialog-body">
          <p>以下模组存在文件冲突，请选择要安装的模组：</p>
          <div class="mod-selection-list">
            ${conflictingMods.map((mod, index) => `
              <label class="mod-selection-item">
                <input type="radio" name="mod-select" value="${index}">
                <div class="mod-selection-info">
                  <strong>${mod.name}</strong>
                  <small>版本: ${mod.version} | 作者: ${mod.author}</small>
                </div>
              </label>
            `).join('')}
          </div>
          <p style="margin-top: 15px; color: #e74c3c;">冲突文件（共 ${conflictingFiles.length} 个）：</p>
          <ul class="conflict-file-list">
            ${conflictingFiles.slice(0, 10).map(file => `<li>${file}</li>`).join('')}
            ${conflictingFiles.length > 10 ? `<li>... 还有 ${conflictingFiles.length - 10} 个文件</li>` : ''}
          </ul>
        </div>
        <div class="conflict-dialog-footer">
          <button class="btn btn-secondary" id="cancel-all-btn">都不安装</button>
          <button class="btn btn-primary" id="confirm-select-btn">安装选中的模组</button>
        </div>
      </div>
    `
    
    const style = document.createElement('style')
    style.textContent = `
      .conflict-dialog-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 2000;
        backdrop-filter: blur(4px);
      }
      .conflict-dialog {
        background: white;
        border-radius: 12px;
        padding: 24px;
        max-width: 700px;
        max-height: 85vh;
        overflow-y: auto;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
      }
      .conflict-dialog-header h3 {
        margin: 0 0 20px 0;
        color: #e74c3c;
        font-size: 20px;
      }
      .conflict-dialog-body {
        margin-bottom: 20px;
      }
      .mod-selection-list {
        display: flex;
        flex-direction: column;
        gap: 10px;
        margin: 15px 0;
      }
      .mod-selection-item {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px;
        border: 2px solid #e0e0e0;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
      }
      .mod-selection-item:hover {
        border-color: #3498db;
        background: #f0f8ff;
      }
      .mod-selection-info {
        display: flex;
        flex-direction: column;
        gap: 4px;
      }
      .mod-selection-info strong {
        font-size: 16px;
      }
      .mod-selection-info small {
        color: #666;
        font-size: 13px;
      }
      .conflict-file-list {
        max-height: 150px;
        overflow-y: auto;
        background: #f8f9fa;
        padding: 12px;
        border-radius: 6px;
        font-family: 'Consolas', 'Monaco', monospace;
        font-size: 12px;
        border: 1px solid #e0e0e0;
      }
      .conflict-file-list li {
        margin: 4px 0;
      }
      .conflict-dialog-footer {
        display: flex;
        gap: 10px;
        justify-content: flex-end;
        padding-top: 15px;
        border-top: 1px solid #e0e0e0;
      }
    `
    
    document.head.appendChild(style)
    document.body.appendChild(dialog)
    
    const confirmBtn = document.getElementById('confirm-select-btn')
    const cancelBtn = document.getElementById('cancel-all-btn')
    
    confirmBtn?.addEventListener('click', () => {
      const selected = document.querySelector('input[name="mod-select"]:checked') as HTMLInputElement
      if (selected) {
        const index = parseInt(selected.value)
        document.body.removeChild(dialog)
        document.head.removeChild(style)
        resolve(conflictingMods[index])
      } else {
        alert('请选择一个模组')
      }
    })
    
    cancelBtn?.addEventListener('click', () => {
      document.body.removeChild(dialog)
      document.head.removeChild(style)
      resolve(null)
    })
  })
}
</script>

<template>
  <div class="game-detail-panel panel">
    <div class="panel-body">
      <div v-if="!game" class="loading">
        <p>加载中...</p>
      </div>

      <div v-else class="content">
        <!-- Steam风格背景横幅 -->
        <div class="hero-banner" :style="{ backgroundImage: `url(${heroBackground})` }">
          <div class="hero-gradient"></div>
          <div class="hero-content">
            <div class="hero-game-info">
              <h1 class="hero-title">{{ game?.name || '游戏详情' }}</h1>
              <div v-if="game" class="hero-actions">
                <button 
                  @click="launchGame" 
                  :disabled="isLoading"
                  class="btn-hero-launch"
                >
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M8 5V19L19 12L8 5Z" fill="currentColor"/>
                  </svg>
                  <span v-if="isLoading">启动中...</span>
                  <span v-else">开始游戏</span>
                </button>
              </div>
            </div>
          </div>
        </div>

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
            
            <!-- MikuGame 绑定信息 -->
            <div class="info-row">
              <label>MikuGame 绑定:</label>
              <div v-if="!isEditingBinding" class="binding-display">
                <div v-if="game.mikuGameId" class="binding-info">
                  <span class="binding-badge">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M20 6L9 17L4 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                    已绑定
                  </span>
                  <span class="binding-type">{{ game.mikuGameType }}</span>
                </div>
                <span v-else class="binding-empty">(未绑定)</span>
                <div class="binding-actions">
                  <button class="btn btn-secondary btn-sm" @click="startEditBinding">
                    {{ game.mikuGameId ? '修改绑定' : '绑定游戏' }}
                  </button>
                  <button v-if="game.mikuGameId" class="btn btn-error btn-sm" @click="unbindMikuGame">
                    解除绑定
                  </button>
                </div>
              </div>
              <div v-else class="binding-edit">
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
                    取消选择
                  </button>
                </div>

                <!-- 搜索按钮 -->
                <button 
                  v-if="!selectedMikuGame" 
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

                <!-- 操作按钮 -->
                <div class="edit-actions" style="margin-top: var(--space-3);">
                  <button @click="saveBinding" class="btn btn-success" :disabled="!selectedMikuGame">
                    保存绑定
                  </button>
                  <button @click="cancelEditBinding" class="btn btn-error">
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
          </div>
        </div>

        <!-- 模组管理模块 -->
        <div class="card">
          <div class="card-header">
            <h3>模组管理</h3>
            <div class="header-actions">
              <button 
                @click="applyModChanges"
                :disabled="!hasModChanges || isApplyingMods"
                class="btn btn-success btn-sm"
              >
                {{ isApplyingMods ? '应用中...' : '应用更改' }}
              </button>
            </div>
          </div>
          <div class="card-body">
            <div v-if="mods.length === 0" class="empty-state">
              <p>暂无可用模组</p>
              <small>模组应放置在 game/{{ game.name }}/mods/ 目录下<br>
              每个文件夹对应一个模组，结构：mods/{文件夹名}/_mikumodinfo.json + 模组文件</small>
            </div>
            <div v-else>
              <div class="mods-list">
                <div 
                  v-for="mod in mods" 
                  :key="mod.id"
                  class="mod-item"
                  :class="{ enabled: mod.enabled, 'non-mikumod': mod.name.includes('非MikuMod模组') }"
                >
                  <div class="mod-info-section">
                    <div class="mod-header-row">
                      <div class="mod-title-group">
                        <h4 class="mod-name">{{ mod.name }}</h4>
                        <div class="mod-meta-badges">
                          <span class="badge badge-version">v{{ mod.version }}</span>
                          <span class="badge badge-author">{{ mod.author }}</span>
                          <span v-if="mod.name.includes('非MikuMod模组')" class="badge badge-warning">非标准模组</span>
                  </div>
                </div>
                      <label class="toggle-switch">
                  <input 
                          type="checkbox" 
                          v-model="mod.enabled"
                        >
                        <span class="slider"></span>
                        <span class="toggle-label">{{ mod.enabled ? '已启用' : '未启用' }}</span>
                      </label>
                    </div>
                    <p class="mod-description">{{ mod.description }}</p>
                    <div v-if="mod.installDate" class="mod-install-date">
                      📅 安装时间: {{ new Date(mod.installDate).toLocaleString('zh-CN') }}
                  </div>
                </div>
              </div>
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
  height: 100%;
  overflow: hidden;
}

/* Steam风格背景横幅 */
.hero-banner {
  position: relative;
  width: 100%;
  height: 450px;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  overflow: hidden;
  margin-bottom: var(--space-6);
}

.hero-gradient {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    to bottom,
    rgba(0, 0, 0, 0.3) 0%,
    rgba(0, 0, 0, 0.5) 50%,
    rgba(16, 24, 32, 0.95) 100%
  );
}

.hero-content {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: var(--space-8) var(--space-6);
  z-index: 1;
}

.hero-game-info {
  max-width: 1200px;
  margin: 0 auto;
}

.hero-title {
  color: white;
  font-size: 3rem;
  font-weight: 700;
  margin: 0 0 var(--space-6) 0;
  text-shadow: 2px 2px 8px rgba(0, 0, 0, 0.8);
  letter-spacing: -0.5px;
}

.hero-actions {
  display: flex;
  gap: var(--space-3);
  flex-wrap: wrap;
}

.btn-hero-launch {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-6);
  background: linear-gradient(135deg, #5C7CFA 0%, #4C6EF5 100%);
  color: white;
  border: none;
  border-radius: var(--radius-base);
  font-size: var(--font-lg);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 4px 12px rgba(76, 110, 245, 0.4);
}

.btn-hero-launch:hover:not(:disabled) {
  background: linear-gradient(135deg, #4C6EF5 0%, #3B5BDB 100%);
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(76, 110, 245, 0.5);
}

.btn-hero-launch:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.panel-body {
  flex: 1;
  overflow-y: auto;
  padding: 0;
  background: var(--bg-primary);
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
  gap: 0;
  width: 100%;
}

/* 卡片容器 */
.content > .card {
  max-width: 1200px;
  width: calc(100% - var(--space-12));
  margin: 0 auto var(--space-6);
}

/* 卡片美化 */
.card {
  background: white;
  border-radius: var(--radius-lg);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  border: 1px solid var(--border-color);
  transition: all 0.3s ease;
  overflow: hidden;
}

.card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  transform: translateY(-2px);
}

.content > .card:last-child {
  margin-bottom: var(--space-8);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-5) var(--space-6);
  background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
  border-bottom: 2px solid var(--primary-color);
}

.card-header h3 {
  margin: 0;
  font-size: var(--font-lg);
  font-weight: 700;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.card-header h3::before {
  content: '';
  display: inline-block;
  width: 4px;
  height: 20px;
  background: linear-gradient(135deg, #5C7CFA 0%, #4C6EF5 100%);
  border-radius: 2px;
}

.card-body {
  padding: var(--space-6);
}

.header-actions {
  display: flex;
  gap: var(--space-2);
}

.info-row {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  margin-bottom: var(--space-5);
  padding: var(--space-4);
  background: #fafbfc;
  border-radius: var(--radius-md);
  border-left: 3px solid var(--primary-color);
  transition: all 0.2s ease;
}

.info-row:hover {
  background: #f3f5f7;
}

.info-row:last-child {
  margin-bottom: 0;
}

.info-row label {
  font-weight: 700;
  color: var(--text-secondary);
  min-width: 120px;
  padding-top: var(--space-2);
  font-size: var(--font-sm);
  letter-spacing: 0.3px;
}

.directory-display, .launch-options-display {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  flex: 1;
}

.directory-path, .launch-options-text {
  font-family: var(--font-family-mono);
  background: white;
  padding: var(--space-3) var(--space-4);
  border-radius: var(--radius-md);
  border: 2px solid #e1e4e8;
  flex: 1;
  word-break: break-all;
  font-size: var(--font-sm);
  transition: all 0.2s ease;
}

.directory-path:hover, .launch-options-text:hover {
  border-color: var(--primary-color);
  box-shadow: 0 2px 8px rgba(76, 110, 245, 0.1);
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
  background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
  border-radius: var(--radius-lg);
  border: 2px dashed var(--border-color);
}

.empty-state p {
  margin: 0 0 var(--space-2) 0;
  font-size: var(--font-lg);
  font-weight: 600;
  color: var(--text-secondary);
}

.empty-state small {
  font-size: var(--font-sm);
  color: var(--text-muted);
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
  padding: var(--space-4);
  background: #fafbfc;
  border: 2px solid #e1e4e8;
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
}

.patch-item:hover, .backup-item:hover {
  border-color: #a8b8d8;
  background: #f3f5f7;
  transform: translateX(4px);
}

.patch-item.selected, .backup-item.selected {
  border-color: var(--primary-color);
  background: linear-gradient(135deg, #e7f5ff 0%, #d0ebff 100%);
  box-shadow: 0 2px 12px rgba(76, 110, 245, 0.2);
  transform: translateX(4px);
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

.backup-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: var(--space-3);
  padding: var(--space-3);
  background: white;
  border-radius: var(--radius-md);
  border: 1px solid #e1e4e8;
  border-left: 3px solid var(--primary-color);
}

.backup-details small {
  display: block;
  font-size: var(--font-xs);
  color: var(--text-secondary);
  line-height: 1.6;
  padding: var(--space-1) 0;
}

.conflict-warning {
  color: var(--warning-color) !important;
  font-weight: var(--font-semibold);
}

.install-actions, .rollback-actions {
  display: flex;
  justify-content: center;
  gap: var(--space-3);
  flex-wrap: wrap;
  padding-top: var(--space-3);
}

.install-actions .btn,
.rollback-actions .btn {
  font-weight: 600;
  padding: var(--space-3) var(--space-6);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.install-actions .btn:hover:not(:disabled),
.rollback-actions .btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.btn-sm {
  padding: var(--space-2) var(--space-4);
  font-size: var(--font-sm);
  font-weight: 600;
  transition: all 0.2s ease;
}

.btn-sm:hover {
  transform: translateY(-1px);
}

/* MikuGame 绑定相关样式 */
.binding-display {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
  flex: 1;
}

.binding-info {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.binding-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  background: rgba(52, 152, 219, 0.1);
  color: var(--primary-color);
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-full);
  font-size: var(--font-sm);
  font-weight: var(--font-semibold);
  border: 1px solid rgba(52, 152, 219, 0.3);
}

.binding-badge svg {
  flex-shrink: 0;
}

.binding-type {
  padding: var(--space-1) var(--space-2);
  background: var(--gray-100);
  color: var(--text-secondary);
  border-radius: var(--radius-base);
  font-size: var(--font-xs);
  font-family: var(--font-family-mono);
}

.binding-empty {
  color: var(--text-muted);
  font-style: italic;
}

.binding-actions {
  display: flex;
  gap: var(--space-2);
  margin-left: auto;
}

.binding-edit {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

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
  min-width: 0;
}

.miku-game-cover {
  width: 60px;
  height: 60px;
  object-fit: cover;
  border-radius: var(--radius-base);
  flex-shrink: 0;
}

.miku-game-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  flex: 1;
  min-width: 0;
}

.miku-game-details strong {
  font-size: var(--font-sm);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
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
  width: fit-content;
}

.miku-game-search {
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
  min-width: 0;
}

.result-info strong {
  font-size: var(--font-sm);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
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

/* 模组管理样式 */
.mods-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.mod-item {
  padding: var(--space-4);
  background: #fafbfc;
  border: 2px solid #e1e4e8;
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
}

.mod-item:hover {
  border-color: #a8b8d8;
  background: #f3f5f7;
  transform: translateX(2px);
}

.mod-item.enabled {
  border-color: var(--success-color);
  background: linear-gradient(135deg, #e7f5ff 0%, #d0ebff 100%);
  box-shadow: 0 2px 12px rgba(76, 200, 100, 0.2);
}

.mod-item.non-mikumod {
  border-left: 4px solid var(--warning-color);
  background: #fff9e6;
}

.mod-item.non-mikumod.enabled {
  background: linear-gradient(135deg, #fff9e6 0%, #ffe8a1 100%);
}

.badge-warning {
  background: linear-gradient(135deg, #f59f00 0%, #f76707 100%);
  color: white;
}

.mod-info-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.mod-header-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--space-4);
}

.mod-title-group {
  flex: 1;
  min-width: 0;
}

.mod-name {
  margin: 0 0 var(--space-2) 0;
  font-size: var(--font-lg);
  font-weight: 700;
  color: var(--text-primary);
}

.mod-meta-badges {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.badge {
  padding: 4px 10px;
  border-radius: var(--radius-full);
  font-size: var(--font-xs);
  font-weight: 600;
}

.badge-version {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.badge-author {
  background: var(--gray-200);
  color: var(--text-secondary);
}

.mod-description {
  margin: 0;
  color: var(--text-secondary);
  font-size: var(--font-sm);
  line-height: 1.6;
}

.mod-install-date {
  font-size: var(--font-xs);
  color: var(--text-muted);
  padding: var(--space-2) var(--space-3);
  background: white;
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--success-color);
}

/* Toggle Switch 样式 */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  cursor: pointer;
  user-select: none;
  flex-shrink: 0;
}

.toggle-switch input[type="checkbox"] {
  display: none;
}

.slider {
  position: relative;
  width: 48px;
  height: 24px;
  background-color: var(--gray-300);
  border-radius: 24px;
  transition: background-color var(--transition-base);
}

.slider::before {
  content: '';
  position: absolute;
  width: 18px;
  height: 18px;
  left: 3px;
  top: 3px;
  background-color: white;
  border-radius: 50%;
  transition: transform var(--transition-base);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-switch input[type="checkbox"]:checked + .slider {
  background-color: var(--success-color);
}

.toggle-switch input[type="checkbox"]:checked + .slider::before {
  transform: translateX(24px);
}

.toggle-label {
  font-size: var(--font-sm);
  font-weight: var(--font-semibold);
  color: var(--text-secondary);
  white-space: nowrap;
}
</style>