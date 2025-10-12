<template>
  <div class="mod-manager-panel">
    <div class="header">
      <h2>模组管理器</h2>
      <div class="actions">
        <button @click="refreshMods" class="btn btn-primary">刷新模组列表</button>
        <button @click="openModDirectory" class="btn btn-secondary">打开模组目录</button>
      </div>
    </div>

    <div class="mods-container">
      <div v-if="mods.length === 0" class="no-mods">
        <p>暂无模组，请将模组文件夹放置在 mod/ 目录下</p>
        <p>模组结构：mod/模组文件夹/mod.json + mod/模组文件</p>
      </div>

      <div v-else class="mods-list">
        <div 
          v-for="mod in mods" 
          :key="mod.id" 
          class="mod-item"
          :class="{ 'installed': mod.installed, 'conflict': mod.hasConflicts }"
        >
          <div class="mod-info">
            <h3>{{ mod.name }}</h3>
            <div class="mod-meta">
              <span class="version">版本: {{ mod.version }}</span>
              <span class="author">作者: {{ mod.author }}</span>
              <span class="game">游戏: {{ mod.game }}</span>
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
            <button 
              v-if="!mod.installed" 
              @click="installMod(mod)" 
              class="btn btn-primary"
              :disabled="!canInstall(mod)"
            >
              安装
            </button>
            
            <button 
              v-else 
              @click="uninstallMod(mod)" 
              class="btn btn-danger"
            >
              卸载
            </button>
            
            <button @click="viewModDetails(mod)" class="btn btn-secondary">详情</button>
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
              <span>{{ selectedMod.version }}</span>
            </div>
            <div class="detail-item">
              <label>作者:</label>
              <span>{{ selectedMod.author }}</span>
            </div>
            <div class="detail-item">
              <label>游戏:</label>
              <span>{{ selectedMod.game }}</span>
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
            
            <div v-if="selectedMod.dependencies.length > 0" class="detail-item">
              <label>依赖模组:</label>
              <span>{{ selectedMod.dependencies.join(', ') }}</span>
            </div>
            
            <div v-if="selectedMod.conflicts.length > 0" class="detail-item">
              <label>冲突模组:</label>
              <span>{{ selectedMod.conflicts.join(', ') }}</span>
            </div>
            
            <div v-if="selectedMod.installDate" class="detail-item">
              <label>安装时间:</label>
              <span>{{ formatDate(selectedMod.installDate) }}</span>
            </div>
          </div>
        </div>
        
        <div class="modal-footer">
          <button @click="closeModal" class="btn btn-secondary">关闭</button>
          <button 
            v-if="!selectedMod.installed" 
            @click="installMod(selectedMod)" 
            class="btn btn-primary"
            :disabled="!canInstall(selectedMod)"
          >
            安装
          </button>
          <button 
            v-else 
            @click="uninstallMod(selectedMod)" 
            class="btn btn-danger"
          >
            卸载
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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
  hasConflicts?: boolean
}

// 安装日志接口
interface InstallLog {
  modId: string
  installDate: string
  installedFiles: Array<{
    sourcePath: string
    targetPath: string
    backupPath?: string
    originalHash?: string
    installedHash?: string
  }>
  backupFiles: Array<{
    originalPath: string
    backupPath: string
    hash: string
  }>
}

const mods = ref<ModConfig[]>([])
const selectedMod = ref<ModConfig | null>(null)

onMounted(async () => {
  await loadMods()
})

// 加载所有模组
async function loadMods() {
  try {
    const modsDir = 'mod'
    const modFolders = await invoke('scan_directory', { path: modsDir }) as string[]
    
    const modPromises = modFolders.map(async (folder: string) => {
      const modPath = `${modsDir}/${folder}`
      const configPath = `${modPath}/mod.json`
      
      try {
        // 检查配置文件是否存在
        const configExists = await invoke('file_exists', { path: configPath }) as boolean
        if (!configExists) {
          console.warn(`模组 ${folder} 缺少 mod.json 文件`)
          return null
        }
        
        // 读取配置文件
        const configContent = await invoke('read_file', { path: configPath }) as string
        const config = JSON.parse(configContent) as ModConfig
        
        // 检查模组文件夹是否存在
        const modFilesPath = `${modPath}/mod`
        const modFilesExists = await invoke('file_exists', { path: modFilesPath }) as boolean
        if (!modFilesExists) {
          console.warn(`模组 ${folder} 缺少 mod 文件夹`)
          return null
        }
        
        // 获取模组文件列表
        const files = await invoke('get_all_files', { path: modFilesPath }) as string[]
        config.files = files.map(file => file.replace(modFilesPath, '').replace(/^[/\\]/, ''))
        
        // 检查是否已安装
        config.installed = await checkModInstalled(config)
        
        // 检查冲突
        config.hasConflicts = await checkModConflicts(config)
        
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

// 检查模组是否已安装
async function checkModInstalled(mod: ModConfig): Promise<boolean> {
  try {
    // 检查安装日志
    const installLogPath = `game/${mod.game}/mods/${mod.id}/install_log.json`
    const logExists = await invoke('file_exists', { path: installLogPath }) as boolean
    return logExists
  } catch (error) {
    return false
  }
}

// 检查模组冲突
async function checkModConflicts(mod: ModConfig): Promise<boolean> {
  if (mod.conflicts.length === 0) return false
  
  try {
    // 检查冲突模组是否已安装
    for (const conflictModId of mod.conflicts) {
      const conflictLogPath = `game/${mod.game}/mods/${conflictModId}/install_log.json`
      const conflictExists = await invoke('file_exists', { path: conflictLogPath }) as boolean
      if (conflictExists) {
        return true
      }
    }
    return false
  } catch (error) {
    return false
  }
}

// 检查是否可以安装模组
function canInstall(mod: ModConfig): boolean {
  // 检查依赖是否满足
  if (mod.dependencies.length > 0) {
    const installedModIds = mods.value.filter(m => m.installed).map(m => m.id)
    const missingDeps = mod.dependencies.filter(dep => !installedModIds.includes(dep))
    return missingDeps.length === 0
  }
  return true
}

// 安装模组
async function installMod(mod: ModConfig) {
  try {
    // 检查文件名冲突
    const { conflictingFiles, conflictingMods } = await checkFileConflicts(mod)
    
    let skipConflictingFiles = false
    
    if (conflictingFiles.length > 0) {
      // 显示冲突解决对话框
      const resolution = await showConflictResolutionDialog(conflictingFiles, conflictingMods)
      
      if (resolution === 'cancel') {
        alert('安装已取消')
        return
      }
      
      skipConflictingFiles = (resolution === 'skip')
    }
    
    // 创建模组安装目录
    const modInstallDir = `game/${mod.game}/mods/${mod.id}`
    await invoke('create_directory', { path: modInstallDir })
    
    // 创建备份目录
    const backupDir = `${modInstallDir}/backup`
    await invoke('create_directory', { path: backupDir })
    
    const installLog: InstallLog = {
      modId: mod.id,
      installDate: new Date().toISOString(),
      installedFiles: [],
      backupFiles: []
    }
    
    const modFilesPath = `mod/${mod.id}/mod`
    
    // 安装每个文件
    for (const file of mod.files) {
      const sourcePath = `${modFilesPath}/${file}`
      const targetPath = `${mod.installPath}/${file}`
      const backupPath = `${backupDir}/${file}`
      
      // 检查是否跳过冲突文件
      if (skipConflictingFiles && conflictingFiles.includes(file)) {
        continue
      }
      
      // 检查目标文件是否存在，存在则备份
      const targetExists = await invoke('file_exists', { path: targetPath }) as boolean
      if (targetExists) {
        // 创建备份目录结构
        const backupDirPath = backupPath.substring(0, backupPath.lastIndexOf('/'))
        await invoke('create_directory', { path: backupDirPath })
        
        // 备份原文件
        await invoke('copy_file', { from: targetPath, to: backupPath })
        
        // 计算文件哈希
        const originalHash = await calculateFileHash(targetPath)
        
        installLog.backupFiles.push({
          originalPath: targetPath,
          backupPath: backupPath,
          hash: originalHash
        })
      }
      
      // 创建目标目录结构
      const targetDirPath = targetPath.substring(0, targetPath.lastIndexOf('/'))
      await invoke('create_directory', { path: targetDirPath })
      
      // 安装模组文件
      await invoke('copy_file', { from: sourcePath, to: targetPath })
      
      // 计算安装文件哈希
      const installedHash = await calculateFileHash(targetPath)
      
      installLog.installedFiles.push({
        sourcePath: sourcePath,
        targetPath: targetPath,
        backupPath: targetExists ? backupPath : undefined,
        originalHash: targetExists ? await calculateFileHash(sourcePath) : undefined,
        installedHash: installedHash
      })
    }
    
    // 保存安装日志
    await invoke('write_file', {
      path: `${modInstallDir}/install_log.json`,
      content: JSON.stringify(installLog, null, 2)
    })
    
    // 更新模组状态
    mod.installed = true
    mod.installDate = installLog.installDate
    
    alert(`模组 "${mod.name}" 安装成功！`)
    
  } catch (error) {
    console.error('安装模组失败:', error)
    alert(`安装模组失败: ${error}`)
  }
}

// 卸载模组
async function uninstallMod(mod: ModConfig) {
  if (!confirm(`确定要卸载模组 "${mod.name}" 吗？`)) {
    return
  }
  
  try {
    const modInstallDir = `game/${mod.game}/mods/${mod.id}`
    const installLogPath = `${modInstallDir}/install_log.json`
    
    // 读取安装日志
    const logContent = await invoke('read_file', { path: installLogPath }) as string
    const installLog: InstallLog = JSON.parse(logContent)
    
    // 恢复备份文件
    for (const backupFile of installLog.backupFiles) {
      const backupExists = await invoke('file_exists', { path: backupFile.backupPath }) as boolean
      if (backupExists) {
        // 验证备份文件完整性
        const backupHash = await calculateFileHash(backupFile.backupPath)
        if (backupHash === backupFile.hash) {
          await invoke('copy_file', { from: backupFile.backupPath, to: backupFile.originalPath })
        } else {
          console.warn(`备份文件完整性验证失败: ${backupFile.backupPath}`)
        }
      }
    }
    
    // 删除安装的文件（如果没有备份的）
    for (const installedFile of installLog.installedFiles) {
      const hasBackup = installLog.backupFiles.some(bf => bf.originalPath === installedFile.targetPath)
      if (!hasBackup) {
        const fileExists = await invoke('file_exists', { path: installedFile.targetPath }) as boolean
        if (fileExists) {
          await invoke('delete_file', { path: installedFile.targetPath })
        }
      }
    }
    
    // 删除模组安装目录
    await invoke('delete_directory', { path: modInstallDir })
    
    // 更新模组状态
    mod.installed = false
    mod.installDate = undefined
    
    alert(`模组 "${mod.name}" 卸载成功！`)
    
  } catch (error) {
    console.error('卸载模组失败:', error)
    alert(`卸载模组失败: ${error}`)
  }
}

// 计算文件哈希
async function calculateFileHash(filePath: string): Promise<string> {
  try {
    // 简化实现，使用文件大小作为哈希
    const fileSize = await invoke('get_file_size', { path: filePath }).catch(() => 0)
    return `size:${fileSize}`
  } catch {
    return 'unknown'
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

// 格式化日期
function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString)
    return date.toLocaleString('zh-CN')
  } catch {
    return dateString
  }
}

// 检查文件名冲突
async function checkFileConflicts(mod: ModConfig): Promise<{conflictingFiles: string[], conflictingMods: string[]}> {
  const conflictingFiles: string[] = []
  const conflictingMods: string[] = []
  
  try {
    // 获取所有已安装模组的安装日志
    const installedMods = mods.value.filter(m => m.installed && m.id !== mod.id)
    
    for (const installedMod of installedMods) {
      const installLogPath = `game/${installedMod.game}/mods/${installedMod.id}/install_log.json`
      
      try {
        const logContent = await invoke('read_file', { path: installLogPath }) as string
        const installLog: InstallLog = JSON.parse(logContent)
        
        // 检查当前模组的每个文件是否与已安装模组冲突
        for (const file of mod.files) {
          const targetPath = `${mod.installPath}/${file}`
          
          // 检查是否在已安装模组的安装日志中存在相同路径的文件
          const hasConflict = installLog.installedFiles.some(installedFile => 
            installedFile.targetPath === targetPath
          )
          
          if (hasConflict) {
            conflictingFiles.push(file)
            if (!conflictingMods.includes(installedMod.name)) {
              conflictingMods.push(installedMod.name)
            }
          }
        }
      } catch (error) {
        console.warn(`无法读取模组 ${installedMod.name} 的安装日志:`, error)
      }
    }
  } catch (error) {
    console.error('检查文件冲突时出错:', error)
  }
  
  return { conflictingFiles, conflictingMods }
}

// 显示冲突解决对话框
async function showConflictResolutionDialog(
  // mod: ModConfig,
  conflictingFiles: string[], 
  conflictingMods: string[]
): Promise<'skip' | 'overwrite' | 'cancel'> {
  return new Promise((resolve) => {
    // 创建冲突解决对话框
    const dialog = document.createElement('div')
    dialog.className = 'conflict-dialog-overlay'
    dialog.innerHTML = `
      <div class="conflict-dialog">
        <div class="conflict-dialog-header">
          <h3>文件名冲突检测</h3>
        </div>
        <div class="conflict-dialog-body">
          <p>检测到模组 <strong>{{ mod.name }}</strong> 与以下已安装模组存在文件名冲突：</p>
          <ul>
            ${conflictingMods.map(modName => `<li>${modName}</li>`).join('')}
          </ul>
          <p>冲突文件：</p>
          <ul class="conflict-file-list">
            ${conflictingFiles.map(file => `<li>${file}</li>`).join('')}
          </ul>
          <p>请选择处理方式：</p>
        </div>
        <div class="conflict-dialog-footer">
          <button class="btn btn-secondary" id="skip-btn">跳过冲突文件</button>
          <button class="btn btn-warning" id="overwrite-btn">覆盖冲突文件</button>
          <button class="btn btn-danger" id="cancel-btn">取消安装</button>
        </div>
      </div>
    `
    
    // 添加样式
    const style = document.createElement('style')
    style.textContent = `
      .conflict-dialog-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
      }
      .conflict-dialog {
        background: white;
        border-radius: 8px;
        padding: 20px;
        max-width: 500px;
        max-height: 80vh;
        overflow-y: auto;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      }
      .conflict-dialog-header h3 {
        margin: 0 0 15px 0;
        color: #e74c3c;
      }
      .conflict-dialog-body {
        margin-bottom: 20px;
      }
      .conflict-file-list {
        max-height: 150px;
        overflow-y: auto;
        background: #f8f9fa;
        padding: 10px;
        border-radius: 4px;
        font-family: monospace;
        font-size: 12px;
      }
      .conflict-dialog-footer {
        display: flex;
        gap: 10px;
        justify-content: flex-end;
      }
      .btn {
        padding: var(--space-3) var(--space-5);
        border: none;
        border-radius: var(--radius-base);
        cursor: pointer;
        font-size: var(--font-sm);
        font-weight: var(--font-medium);
        text-decoration: none;
        display: inline-block;
        transition: all var(--transition-base);
      }
      .btn-secondary {
        background: var(--gray-400);
        color: var(--text-white);
      }
      .btn-secondary:hover {
        background: var(--gray-500);
        transform: translateY(-1px);
      }
      .btn-warning {
        background: var(--warning-color);
        color: var(--text-white);
      }
      .btn-warning:hover {
        background: #e67e22;
        transform: translateY(-1px);
        box-shadow: var(--shadow-warning);
      }
      .btn-danger {
        background: var(--error-color);
        color: var(--text-white);
      }
      .btn-danger:hover {
        background: #c0392b;
        transform: translateY(-1px);
        box-shadow: var(--shadow-error);
      }
    `
    
    document.head.appendChild(style)
    document.body.appendChild(dialog)
    
    // 添加事件监听器
    document.getElementById('skip-btn')?.addEventListener('click', () => {
      document.body.removeChild(dialog)
      document.head.removeChild(style)
      resolve('skip')
    })
    
    document.getElementById('overwrite-btn')?.addEventListener('click', () => {
      document.body.removeChild(dialog)
      document.head.removeChild(style)
      resolve('overwrite')
    })
    
    document.getElementById('cancel-btn')?.addEventListener('click', () => {
      document.body.removeChild(dialog)
      document.head.removeChild(style)
      resolve('cancel')
    })
  })
}
</script>

<style scoped>
.mods-list {
  display: grid;
  gap: var(--space-4);
}

.mod-item {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  background: var(--bg-card);
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  transition: all var(--transition-base);
}

.mod-item.installed {
  border-color: var(--success-color);
  background: #f8fff9;
}

.mod-item.conflict {
  border-color: var(--error-color);
  background: #fff5f5;
}

.mod-info {
  flex: 1;
}

.mod-info h3 {
  margin: 0 0 var(--space-3) 0;
  color: var(--text-primary);
}

.mod-meta {
  display: flex;
  gap: var(--space-4);
  margin-bottom: var(--space-3);
  font-size: var(--font-sm);
  color: var(--text-muted);
}

.description {
  margin: var(--space-3) 0;
  color: var(--text-secondary);
}

.dependencies, .conflicts, .install-info {
  font-size: var(--font-xs);
  margin: var(--space-1) 0;
}

.conflicts {
  color: var(--error-color);
}

.mod-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  min-width: 100px;
}

/* 模态框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  width: 600px;
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
}

.btn-close {
  background: none;
  border: none;
  font-size: 1.5em;
  cursor: pointer;
  color: var(--text-muted);
  transition: color var(--transition-base);
}

.btn-close:hover {
  color: var(--text-primary);
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
}

.mod-details {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.detail-item label {
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.file-list {
  max-height: 150px;
  overflow-y: auto;
  background: var(--gray-50);
  padding: var(--space-3);
  border-radius: var(--radius-sm);
  font-family: var(--font-family-mono);
  font-size: var(--font-xs);
}

.file-list li {
  margin: var(--space-1) 0;
}

/* 按钮样式 */
.btn {
  padding: var(--space-2) var(--space-4);
  border: none;
  border-radius: var(--radius-base);
  cursor: pointer;
  font-size: var(--font-sm);
  text-decoration: none;
  display: inline-block;
  transition: all var(--transition-base);
}

.btn-primary {
  background: var(--primary-color);
  color: var(--text-white);
}

.btn-primary:hover {
  background: var(--primary-dark);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.btn-primary:disabled {
  background: var(--gray-300);
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--gray-400);
  color: var(--text-white);
}

.btn-secondary:hover {
  background: var(--gray-500);
  transform: translateY(-1px);
}

.btn-danger {
  background: var(--error-color);
  color: var(--text-white);
}

.btn-danger:hover {
  background: #c0392b;
  transform: translateY(-1px);
  box-shadow: var(--shadow-error);
}
</style>