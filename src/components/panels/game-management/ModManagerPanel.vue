<template>
  <div class="mod-manager-panel panel">
    <div class="panel-header">
      <h2>模组管理器</h2>
      <div class="actions">
        <button @click="applyChanges" class="btn btn-success" :disabled="!hasChanges || isApplying">
          {{ isApplying ? '应用中...' : '应用更改' }}
        </button>
        <button @click="importMod" class="btn btn-primary">导入模组</button>
        <button @click="refreshMods" class="btn btn-primary">刷新模组列表</button>
        <button @click="openModDirectory" class="btn btn-secondary">打开模组目录</button>
      </div>
    </div>

    <div class="panel-body">
      <div v-if="mods.length === 0" class="empty-state">
        <p>暂无模组，请将模组文件夹放置在 mod/ 目录下</p>
        <p>每个文件夹对应一个模组，结构：mod/{文件夹名}/_mikumodinfo.json + 模组文件</p>
      </div>

      <div v-else class="mods-list">
        <div 
          v-for="mod in mods" 
          :key="mod.id" 
          class="card mod-item"
          :class="{ 'installed': mod.installed, 'conflict': mod.hasConflicts, 'non-mikumod': !mod.game }"
        >
          <div class="card-body">
            <div class="mod-info">
              <h3>{{ mod.name }}</h3>
              <div class="mod-meta">
                <span class="badge badge-secondary">版本: {{ mod.version }}</span>
                <span class="badge badge-secondary">作者: {{ mod.author }}</span>
                <span v-if="mod.game" class="badge badge-info">游戏: {{ mod.game }}</span>
                <span v-else class="badge badge-warning">需要配置文件</span>
              </div>
              <p class="description">{{ mod.description }}</p>
              
              <div v-if="mod.dependencies.length > 0" class="dependencies">
                <strong>依赖:</strong> {{ mod.dependencies.join(', ') }}
              </div>
              
              <div v-if="mod.conflicts.length > 0" class="conflicts">
                <strong>冲突:</strong> {{ mod.conflicts.join(', ') }}
              </div>
              
              <div v-if="mod.installDate" class="install-info">
                安装时间: {{ formatDate(mod.installDate) }}
              </div>
            </div>

            <div class="mod-actions">
              <label class="toggle-switch" :class="{ 'disabled': !mod.game }">
                <input 
                  type="checkbox" 
                  v-model="mod.enabled"
                  @change="onModToggle"
                  :disabled="!mod.game"
                >
                <span class="slider"></span>
                <span class="toggle-label">{{ !mod.game ? '无法安装' : (mod.enabled ? '已启用' : '未启用') }}</span>
              </label>
              
              <button @click="viewModDetails(mod)" class="btn btn-secondary">详情</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 模组详情模态框 -->
    <div v-if="selectedMod" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>{{ selectedMod.name }} - 详情</h3>
          <button @click="closeModal" class="btn-close">×</button>
        </div>
        
        <div class="modal-body">
          <div class="mod-details">
            <div class="detail-item">
              <label>模组ID:</label>
              <span>{{ selectedMod.id }}</span>
            </div>
            <div class="detail-item">
              <label>版本:</label>
              <span class="badge badge-secondary">{{ selectedMod.version }}</span>
            </div>
            <div class="detail-item">
              <label>作者:</label>
              <span>{{ selectedMod.author }}</span>
            </div>
            <div class="detail-item">
              <label>游戏:</label>
              <span class="badge badge-info">{{ selectedMod.game }}</span>
            </div>
            <div class="detail-item">
              <label>安装路径:</label>
              <span>{{ selectedMod.installPath }}</span>
            </div>
            <div class="detail-item">
              <label>描述:</label>
              <p>{{ selectedMod.description }}</p>
            </div>
            
            <div v-if="selectedMod.files.length > 0" class="detail-item">
              <label>包含文件:</label>
              <ul class="file-list">
                <li v-for="file in selectedMod.files" :key="file">{{ file }}</li>
              </ul>
            </div>
            
            <div vif="selectedMod.dependencies.length > 0" class="detail-item">
              <label>依赖模组:</label>
              <span>{{ selectedMod.dependencies.join(', ') }}</span>
            </div>
            
            <div v-if="selectedMod.conflicts.length > 0" class="detail-item">
              <label>冲突模组:</label>
              <span class="text-error">{{ selectedMod.conflicts.join(', ') }}</span>
            </div>
            
            <div v-if="selectedMod.installDate" class="detail-item">
              <label>安装时间:</label>
              <span>{{ formatDate(selectedMod.installDate) }}</span>
            </div>
          </div>
        </div>
        
        <div class="modal-footer">
          <label class="toggle-switch">
            <input 
              type="checkbox" 
              v-model="selectedMod.enabled"
              @change="onModToggle"
            >
            <span class="slider"></span>
            <span class="toggle-label">{{ selectedMod.enabled ? '已启用' : '未启用' }}</span>
          </label>
          <button @click="closeModal" class="btn btn-secondary">关闭</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

// 模组配置接口
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
  enabled?: boolean  // 新增：用户勾选状态
  hasConflicts?: boolean
}

// 安装日志接口（新版）
interface InstallLog {
  modId: string
  modName: string
  installDate: string
  installedFiles: string[]  // 模组安装的所有文件路径（目标路径）
  replacedFiles: string[]   // 被替换的游戏原文件路径（已备份到backup的）
}

// 文件映射接口
interface FileMapping {
  [filePath: string]: string[]  // 文件路径 -> 使用该文件的模组ID列表
}

// Game Status 接口
interface GameStatus {
  game_name: string
  game_path: string
  launch_options: string
  installed_mods: string[]  // 已安装的模组ID列表
  last_updated: string
}

const mods = ref<ModConfig[]>([])
const selectedMod = ref<ModConfig | null>(null)
const isApplying = ref(false)

// 计算是否有更改
const hasChanges = computed(() => {
  return mods.value.some(mod => mod.installed !== mod.enabled)
})

onMounted(async () => {
  await loadMods()
})

// 加载所有模组
async function loadMods() {
  try {
    const modsDir = 'mod'
    const modFolders = await invoke('scan_directory', { path: modsDir }) as string[]
    
    // 读取所有游戏的 game_status.json，获取已安装模组列表
    const installedModsMap = new Map<string, Set<string>>()  // 游戏名 -> 已安装模组ID集合
    
    try {
      const appDir = await invoke('get_app_dir') as string
      const gameDir = `${appDir}/game`
      const gameDirExists = await invoke('file_exists', { path: gameDir }) as boolean
      
      if (gameDirExists) {
        const gameFolders = await invoke('scan_directory', { path: gameDir }) as string[]
        
        for (const gameFolder of gameFolders) {
          const gameStatusPath = `${gameDir}/${gameFolder}/game_status.json`
          const statusExists = await invoke('file_exists', { path: gameStatusPath }) as boolean
          
          if (statusExists) {
            try {
              const statusContent = await invoke('read_file', { path: gameStatusPath }) as string
              const gameStatus = JSON.parse(statusContent) as GameStatus
              installedModsMap.set(gameFolder, new Set(gameStatus.installed_mods || []))
            } catch (error) {
              console.error(`读取游戏状态失败 ${gameFolder}:`, error)
            }
          }
        }
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
            game: '',
            installPath: '',
            files: [],
            dependencies: [],
            conflicts: [],
            installed: false,
            enabled: false
          }
        } else {
          const configContent = await invoke('read_file', { path: configPath }) as string
          config = JSON.parse(configContent) as ModConfig
          
          if (!config.id) {
            config.id = folder
          }
        }
        
        const allFiles = await invoke('get_all_files', { path: modPath }) as string[]
        config.files = allFiles
          .map(file => file.replace(modPath, '').replace(/^[/\\]/, ''))
          .filter(file => file !== '_mikumodinfo.json')
        
        if (config.game) {
          const installedMods = installedModsMap.get(config.game)
          config.installed = installedMods ? installedMods.has(config.id) : false
        }
        
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

// 处理模组勾选变化
function onModToggle() {
}

// =========================
// 应用更改主函数
// =========================
async function applyChanges() {
  isApplying.value = true
  
  try {
    // 获取需要安装和卸载的模组
    const modsToUninstall = mods.value.filter(m => m.installed && !m.enabled)
    const modsToInstall = mods.value.filter(m => !m.installed && m.enabled)
    
    if (modsToUninstall.length === 0 && modsToInstall.length === 0) {
      alert('没有需要应用的更改')
      return
    }
    
    // 步骤1: 先执行所有卸载
    if (modsToUninstall.length > 0) {
      for (const mod of modsToUninstall) {
        try {
          await uninstallMod(mod)
        } catch (error) {
          console.error(`卸载模组 ${mod.name} 失败:`, error)
          mod.enabled = mod.installed
          throw error
        }
      }
    }
    
    // 步骤2: 检测待安装模组间的文件冲突
    if (modsToInstall.length > 1) {
      const conflictResult = await detectInstallConflicts(modsToInstall)
      if (conflictResult.hasConflict) {
        // 显示冲突选择对话框
        const selectedMod = await showModSelectionDialog(conflictResult.conflictingMods, conflictResult.conflictingFiles)
        if (!selectedMod) {
          // 用户选择都不安装，重置所有冲突模组的enabled状态
          conflictResult.conflictingMods.forEach(mod => {
            mod.enabled = mod.installed
          })
          alert('安装已取消')
          isApplying.value = false
          return
        }
        
        // 只安装用户选择的模组，其他冲突模组重置状态
        conflictResult.conflictingMods.forEach(mod => {
          if (mod.id !== selectedMod.id) {
            mod.enabled = mod.installed
          }
        })
        
        try {
          await installMod(selectedMod)
        } catch (error) {
          console.error(`安装模组 ${selectedMod.name} 失败:`, error)
          const errorMessage = error instanceof Error ? error.message : String(error)
          if (errorMessage !== '用户取消安装') {
            selectedMod.enabled = selectedMod.installed
          }
          throw error
        }
      } else {
        // 无冲突，逐个安装
        for (const mod of modsToInstall) {
          try {
            await installMod(mod)
          } catch (error) {
            console.error(`安装模组 ${mod.name} 失败:`, error)
            const errorMessage = error instanceof Error ? error.message : String(error)
            if (errorMessage !== '用户取消安装') {
              mod.enabled = mod.installed
            }
            throw error
          }
        }
      }
    } else if (modsToInstall.length === 1) {
      try {
        await installMod(modsToInstall[0])
      } catch (error) {
        console.error(`安装模组 ${modsToInstall[0].name} 失败:`, error)
        modsToInstall[0].enabled = modsToInstall[0].installed
        throw error
      }
    }
    
    // 刷新模组列表
    await loadMods()
    
    alert('更改已成功应用！')
    
  } catch (error) {
    console.error('应用更改失败:', error)
    await loadMods()
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (errorMessage !== '用户取消安装') {
      alert(`应用更改失败: ${errorMessage}`)
    }
  } finally {
    isApplying.value = false
  }
}

// =========================
// 安装模组（新版）
// =========================
async function installMod(mod: ModConfig) {
  try {
    const appDir = await invoke('get_app_dir') as string
    const gameDir = `${appDir}/game/${mod.game}`
    const installLogDir = `${gameDir}/installLog`
    const backupDir = `${gameDir}/backup`
    
    // 创建必要的目录
    await invoke('create_directory', { path: installLogDir })
    await invoke('create_directory', { path: backupDir })
    
    const installLog: InstallLog = {
      modId: mod.id,
      modName: mod.name,
      installDate: new Date().toISOString(),
      installedFiles: [],
      replacedFiles: []
    }
    
    const modFilesPath = `mod/${mod.id}`
    
    // 检查与已安装模组的冲突
    const { hasConflict, conflictingMods, conflictingFiles } = await checkConflictWithInstalled(mod)
    
    if (hasConflict) {
      // 弹窗提示用户
      const confirmed = confirm(
        `警告：安装模组 "${mod.name}" 将会影响以下已安装模组的效果：\n\n` +
        `${conflictingMods.map(m => `- ${m}`).join('\n')}\n\n` +
        `冲突文件：\n${conflictingFiles.map(f => `- ${f}`).join('\n')}\n\n` +
        `是否继续安装？`
      )
      
      if (!confirmed) {
        mod.enabled = mod.installed
        throw new Error('用户取消安装')
      }
    }
    
    // 安装每个文件
    for (const file of mod.files) {
      const sourcePath = `${modFilesPath}/${file}`
      const targetPath = `${mod.installPath}/${file}`
      
      // 规范化路径（替换反斜杠为正斜杠）
      const normalizedTargetPath = targetPath.replace(/\\/g, '/')
      
      // 检查目标文件是否存在
      const targetExists = await invoke('file_exists', { path: targetPath }) as boolean
      
      if (targetExists) {
        // 检查backup中是否已有此文件
        const backupPath = `${backupDir}/${file}`
        const backupExists = await invoke('file_exists', { path: backupPath }) as boolean
        
        if (!backupExists) {
          const backupDirPath = backupPath.substring(0, backupPath.lastIndexOf('/'))
          await invoke('create_directory', { path: backupDirPath })
          await invoke('copy_file', { from: targetPath, to: backupPath })
          installLog.replacedFiles.push(normalizedTargetPath)
        }
      }
      
      // 创建目标目录结构
      const targetDirPath = targetPath.substring(0, targetPath.lastIndexOf('/'))
      if (targetDirPath) {
        await invoke('create_directory', { path: targetDirPath })
      }
      
      // 复制模组文件到目标位置      
      await invoke('copy_file', { from: sourcePath, to: targetPath })
      installLog.installedFiles.push(normalizedTargetPath)
    }
    
    // 保存安装日志
    const logPath = `${installLogDir}/${mod.id}.json`
    await invoke('write_file', {
      path: logPath,
      content: JSON.stringify(installLog, null, 2)
    })
    
    // 更新文件映射
    await updateFileMapping(mod.game, mod.id, mod.files.map(f => `${mod.installPath}/${f}`.replace(/\\/g, '/')), 'add')
    
    await updateGameStatus(mod.game, mod.id, 'install')
    
  } catch (error) {
    console.error('安装模组失败:', error)
    throw error
  }
}

// =========================
// 卸载模组（新版）
// =========================
async function uninstallMod(mod: ModConfig) {
  try {
    const appDir = await invoke('get_app_dir') as string
    const gameDir = `${appDir}/game/${mod.game}`
    const installLogPath = `${gameDir}/installLog/${mod.id}.json`
    const backupDir = `${gameDir}/backup`
    
    // 检查安装日志是否存在
    const logExists = await invoke('file_exists', { path: installLogPath }) as boolean
    if (!logExists) {
      console.warn(`未找到安装日志: ${installLogPath}`)
      return
    }
    
    // 读取安装日志
    const logContent = await invoke('read_file', { path: installLogPath }) as string
    const installLog: InstallLog = JSON.parse(logContent)
    
    // 删除所有安装的文件
    for (const filePath of installLog.installedFiles) {
      const fileExists = await invoke('file_exists', { path: filePath }) as boolean
      if (fileExists) {
        await invoke('delete_file', { path: filePath })
      }
    }
    
    // 恢复被替换的游戏原文件
    for (const filePath of installLog.replacedFiles) {
      // 获取相对路径
      const relativePath = filePath.replace(mod.installPath + '/', '')
      const backupPath = `${backupDir}/${relativePath}`
      
      const backupExists = await invoke('file_exists', { path: backupPath }) as boolean
      if (backupExists) {
        await invoke('copy_file', { from: backupPath, to: filePath })
      }
    }
    
    // 删除安装日志
    await invoke('delete_file', { path: installLogPath })
    
    // 更新文件映射
    await updateFileMapping(mod.game, mod.id, installLog.installedFiles, 'remove')
    
    await updateGameStatus(mod.game, mod.id, 'uninstall')
    
  } catch (error) {
    console.error('卸载模组失败:', error)
    throw error
  }
}

// 查看模组详情
function viewModDetails(mod: ModConfig) {
  selectedMod.value = mod
}

// 关闭模态框
function closeModal() {
  selectedMod.value = null
}

// 刷新模组列表
async function refreshMods() {
  await loadMods()
  alert('模组列表已刷新')
}

// 打开模组目录
async function openModDirectory() {
  try {
    await invoke('open_directory', { path: 'mod' })
  } catch (error) {
    console.error('打开模组目录失败:', error)
    alert('打开模组目录失败，请确保 mod 目录存在')
  }
}

async function importMod() {
  try {
    const selected = await open({
      multiple: false,
      directory: true,
      title: '选择要导入的模组文件夹'
    })
    
    if (selected) {
      const sourcePath = selected as string
      const modFolderName = sourcePath.split(/[/\\]/).pop() || 'mod'
      const targetPath = `mod/${modFolderName}`
      
      const targetExists = await invoke('file_exists', { path: targetPath }) as boolean
      if (targetExists) {
        if (!confirm(`模组文件夹 "${modFolderName}" 已存在，是否覆盖？`)) {
          return
        }
      }
      
      await invoke('copy_directory', { from: sourcePath, to: targetPath })
      
      await loadMods()
      alert(`模组 "${modFolderName}" 导入成功！`)
    }
  } catch (error) {
    console.error('导入模组失败:', error)
    alert('导入模组失败：' + error)
  }
}

// 格式化日期
function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString)
    return date.toLocaleString('zh-CN')
  } catch {
    return dateString
  }
}

// =========================
// 文件映射管理
// =========================
async function updateFileMapping(gameName: string, modId: string, files: string[], action: 'add' | 'remove') {
  try {
    const appDir = await invoke('get_app_dir') as string
    const mappingPath = `${appDir}/game/${gameName}/installLog/file_mapping.json`
    
    // 读取现有映射
    let mapping: FileMapping = {}
    const mappingExists = await invoke('file_exists', { path: mappingPath }) as boolean
    
    if (mappingExists) {
      const mappingContent = await invoke('read_file', { path: mappingPath }) as string
      mapping = JSON.parse(mappingContent)
    }
    
    // 更新映射
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
        // remove
        if (mapping[normalizedFile]) {
          mapping[normalizedFile] = mapping[normalizedFile].filter(id => id !== modId)
          if (mapping[normalizedFile].length === 0) {
            delete mapping[normalizedFile]
          }
        }
      }
    }
    
    // 保存映射
    await invoke('write_file', {
      path: mappingPath,
      content: JSON.stringify(mapping, null, 2)
    })
    
  } catch (error) {
    console.error('更新文件映射失败:', error)
  }
}

// =========================
// 更新 game_status.json
// =========================
async function updateGameStatus(gameName: string, modId: string, action: 'install' | 'uninstall') {
  try {
    const appDir = await invoke('get_app_dir') as string
    const gameStatusPath = `${appDir}/game/${gameName}/game_status.json`
    
    // 读取现有状态
    const statusExists = await invoke('file_exists', { path: gameStatusPath }) as boolean
    
    if (!statusExists) {
      console.warn(`game_status.json 不存在: ${gameStatusPath}`)
      return
    }
    
    const statusContent = await invoke('read_file', { path: gameStatusPath }) as string
    const gameStatus = JSON.parse(statusContent) as GameStatus
    
    // 确保 installed_mods 字段存在
    if (!gameStatus.installed_mods) {
      gameStatus.installed_mods = []
    }
    
    // 更新已安装模组列表
    if (action === 'install') {
      if (!gameStatus.installed_mods.includes(modId)) {
        gameStatus.installed_mods.push(modId)
      }
    } else {
      // uninstall
      gameStatus.installed_mods = gameStatus.installed_mods.filter(id => id !== modId)
    }
    
    gameStatus.last_updated = new Date().toISOString()
    
    await invoke('write_file', {
      path: gameStatusPath,
      content: JSON.stringify(gameStatus, null, 2)
    })
    
  } catch (error) {
    console.error('更新游戏状态失败:', error)
  }
}

// =========================
// 检测待安装模组间的冲突
// =========================
async function detectInstallConflicts(modsToInstall: ModConfig[]): Promise<{
  hasConflict: boolean
  conflictingMods: ModConfig[]
  conflictingFiles: string[]
}> {
  const fileToMods = new Map<string, ModConfig[]>()
  
  // 构建文件到模组的映射
  for (const mod of modsToInstall) {
    for (const file of mod.files) {
      const targetPath = `${mod.installPath}/${file}`.replace(/\\/g, '/')
      
      if (!fileToMods.has(targetPath)) {
        fileToMods.set(targetPath, [])
      }
      fileToMods.get(targetPath)!.push(mod)
    }
  }
  
  // 查找有冲突的文件
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

// =========================
// 检测与已安装模组的冲突
// =========================
async function checkConflictWithInstalled(mod: ModConfig): Promise<{
  hasConflict: boolean
  conflictingMods: string[]
  conflictingFiles: string[]
}> {
  const conflictingFiles: string[] = []
  const conflictingMods: string[] = []
  
  try {
    const appDir = await invoke('get_app_dir') as string
    const mappingPath = `${appDir}/game/${mod.game}/installLog/file_mapping.json`
    
    // 读取文件映射
    const mappingExists = await invoke('file_exists', { path: mappingPath }) as boolean
    if (!mappingExists) {
      return { hasConflict: false, conflictingMods: [], conflictingFiles: [] }
    }
    
    const mappingContent = await invoke('read_file', { path: mappingPath }) as string
    const mapping: FileMapping = JSON.parse(mappingContent)
    
    // 检查模组的每个文件
    for (const file of mod.files) {
      const targetPath = `${mod.installPath}/${file}`.replace(/\\/g, '/')
      
      if (mapping[targetPath] && mapping[targetPath].length > 0) {
        conflictingFiles.push(file)
        
        // 获取冲突的模组名称
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

// =========================
// 显示模组选择对话框（待安装模组间冲突）
// =========================
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
          <h3>模组冲突检测</h3>
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
      .mod-selection-item input[type="radio"]:checked + .mod-selection-info {
        color: #3498db;
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
      .btn {
        padding: 10px 20px;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 600;
        transition: all 0.2s;
      }
      .btn-primary {
        background: #3498db;
        color: white;
      }
      .btn-primary:hover {
        background: #2980b9;
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(52, 152, 219, 0.3);
      }
      .btn-secondary {
        background: #95a5a6;
        color: white;
      }
      .btn-secondary:hover {
        background: #7f8c8d;
        transform: translateY(-1px);
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

<style scoped>
.mod-manager-panel {
  display: flex;
  flex-direction: column;
}

.panel-header {
  background: linear-gradient(135deg, #e7f3ff 0%, #d4e7ff 100%);
  border-bottom: 2px solid #5C7CFA;
}

.panel-header .actions {
  display: flex;
  gap: var(--space-3);
}

.panel-body {
  padding: var(--space-5);
  overflow-y: auto;
  background: linear-gradient(to bottom, #f8fbff 0%, #ffffff 100%);
}

.empty-state {
  text-align: center;
  padding: var(--space-10);
  color: var(--text-muted);
  background: linear-gradient(135deg, #e7f3ff 0%, #d4e7ff 100%);
  border-radius: var(--radius-md);
  border: 2px dashed #5C7CFA;
}

.mods-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--space-5);
}

.mod-item .card-body {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100%;
}

.mod-item.installed {
  border-left: 4px solid #5C7CFA;
  background: linear-gradient(135deg, #e7f3ff 0%, #d4e7ff 100%);
}

.mod-item.conflict {
  border-left: 4px solid var(--error-color);
}

.mod-item.non-mikumod {
  border-left: 4px solid var(--warning-color);
  background: #fff9e6;
}

.mod-info {
  flex: 1;
}

.mod-info h3 {
  margin: 0 0 var(--space-3) 0;
  color: var(--text-primary);
  font-size: var(--font-lg);
}

.mod-meta {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.description {
  margin: var(--space-4) 0;
  color: var(--text-secondary);
  font-size: var(--font-sm);
}

.dependencies, .conflicts, .install-info {
  font-size: var(--font-xs);
  margin: var(--space-2) 0;
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
}

.dependencies {
  background-color: var(--gray-100);
}

.conflicts {
  background-color: #fff0f0;
  color: var(--error-color);
}

.install-info {
  color: var(--text-muted);
}

.mod-actions {
  display: flex;
  gap: var(--space-3);
  margin-top: var(--space-4);
  border-top: 1px solid var(--border-color);
  padding-top: var(--space-4);
  align-items: center;
}

/* Toggle Switch 样式 */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  cursor: pointer;
  user-select: none;
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
}

.toggle-switch input[type="checkbox"]:checked + .slider {
  background: linear-gradient(135deg, #5C7CFA 0%, #4C6EF5 100%);
}

.toggle-switch input[type="checkbox"]:checked + .slider::before {
  transform: translateX(24px);
}

.toggle-label {
  font-size: var(--font-sm);
  font-weight: var(--font-medium);
  color: var(--text-secondary);
}

.toggle-switch.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toggle-switch.disabled .slider {
  background-color: var(--gray-400);
}

.badge-warning {
  background-color: var(--warning-color);
  color: white;
}

/* 模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-content {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  width: 700px;
  max-width: 90vw;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: var(--shadow-xl);
}

.modal-header {
  padding: var(--space-5);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: var(--font-lg);
}

.btn-close {
  background: none;
  border: none;
  font-size: 1.8em;
  cursor: pointer;
  color: var(--text-muted);
  transition: all var(--transition-base);
}

.btn-close:hover {
  color: var(--text-primary);
  transform: rotate(90deg);
}

.modal-body {
  padding: var(--space-5);
  flex: 1;
  overflow-y: auto;
}

.modal-footer {
  padding: var(--space-5);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  background-color: var(--gray-50);
}

.mod-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.detail-item label {
  font-weight: var(--font-semibold);
  color: var(--text-primary);
  font-size: var(--font-sm);
}

.detail-item span, .detail-item p {
  font-size: var(--font-sm);
  color: var(--text-secondary);
}

.file-list {
  max-height: 150px;
  overflow-y: auto;
  background: var(--gray-50);
  padding: var(--space-3);
  border-radius: var(--radius-base);
  font-family: var(--font-family-mono);
  font-size: var(--font-xs);
  border: 1px solid var(--border-color);
}

.file-list li {
  margin: var(--space-1) 0;
}

.text-error {
  color: var(--error-color);
  font-weight: var(--font-medium);
}
</style>