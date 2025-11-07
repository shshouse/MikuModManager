<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { searchGames, fetchGameById } from '../../../utils/supabase'
import type { MikuGameListItem, GameType } from '../../../types/mikugame'
import {
  NButton,
  NCard,
  NModal,
  NForm,
  NFormItem,
  NInput,
  NSpace,
  NEmpty,
  NBadge,
  NImage,
  NTag,
  NStatistic,
  NScrollbar,
  useMessage
} from 'naive-ui'

const message = useMessage()

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
  mikuGameId?: string
  mikuGameType?: 'games' | 'h_games' | 'galgames'
  images?: string[]
  coverImage?: string
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
    message.error('搜索失败: ' + error)
  } finally {
    isSearchingMikuGame.value = false
  }
}

function selectMikuGame(game: MikuGameListItem) {
  selectedMikuGame.value = game
  newGame.value.mikuGameId = game.id
  newGame.value.mikuGameType = game.game_type
  
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
      launchOptions: '',
      mikuGameId: newGame.value.mikuGameId || undefined,
      mikuGameType: newGame.value.mikuGameType || undefined
    }
    
    if (game.mikuGameId && game.mikuGameType) {
      try {
        const mikuGameData = await fetchGameById(game.mikuGameId, game.mikuGameType)
        if (mikuGameData && mikuGameData.image_urls && mikuGameData.image_urls.length > 0) {
          game.images = mikuGameData.image_urls
          game.coverImage = mikuGameData.image_urls[0]
          
          console.log('已保存游戏图片URL:', mikuGameData.image_urls)
        }
      } catch (error) {
        console.error('获取MikuGame图片URL失败:', error)
      }
    }
    
    games.value.push(game)
    showAddDialog.value = false
    
    saveGames()
    
    try {
      const appDirectory = await invoke('get_app_dir')
      
      const gameDir = `${appDirectory}/game`
      const gameDirExists = await invoke('file_exists', { path: gameDir })
      if (!gameDirExists) {
        await invoke('create_directory', { path: gameDir })
        console.log('Created main game directory:', gameDir)
      }
      
      const gameSpecificDir = `${gameDir}/${game.name}`
      const gameSpecificDirExists = await invoke('file_exists', { path: gameSpecificDir })
      if (!gameSpecificDirExists) {
        await invoke('create_directory', { path: gameSpecificDir })
        console.log('Created game-specific directory:', gameSpecificDir)
      }
      
      const modsDir = `${gameSpecificDir}/mods`
      const modsDirExists = await invoke('file_exists', { path: modsDir })
      if (!modsDirExists) {
        await invoke('create_directory', { path: modsDir })
        console.log('Created mods directory:', modsDir)
      }
      
      const backupDir = `${gameSpecificDir}/backup`
      const backupDirExists = await invoke('file_exists', { path: backupDir })
      if (!backupDirExists) {
        await invoke('create_directory', { path: backupDir })
        console.log('Created backup directory:', backupDir)
      }
      
      const installLogDir = `${gameSpecificDir}/installLog`
      const installLogDirExists = await invoke('file_exists', { path: installLogDir })
      if (!installLogDirExists) {
        await invoke('create_directory', { path: installLogDir })
        console.log('Created installLog directory:', installLogDir)
      }
      
      try {
        await invoke('create_game_status', {
          gameName: game.name,
          gamePath: gameSpecificDir,
          launchOptions: '',
          gameInstallPath: game.directory
        })
        console.log('Created game_status.json for:', game.name, 'at', gameSpecificDir)
      } catch (error) {
        console.error('Failed to create game_status.json:', error)
      }
    } catch (error) {
      console.error('Failed to create game directory structure:', error)
    }
    
    message.success('游戏添加成功')
  } catch (error) {
    console.error('Failed to add game:', error)
    message.error('添加游戏失败')
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
    message.success('游戏已删除')
  }
}

function saveGames() {
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

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.src = '/Miku.png'
}

onMounted(async () => {
  loadGames()
  
  if (typeof window !== 'undefined' && !(window as any).__TAURI__) {
    console.warn('Tauri environment not detected - skipping backend operations')
    return
  }
  
  try {
    const appDirectory = await invoke('get_app_dir')
    const gamesWithoutStatus = await invoke('scan_games_for_status', { 
      appDir: appDirectory 
    }) as string[]
    
    for (const gamePath of gamesWithoutStatus) {
      try {
        const gameName = gamePath.split(/[/\\]/).pop() || 'Unknown Game'
        
        let gameInstallPath = ''
        const saved = localStorage.getItem('customGames')
        if (saved) {
          const savedGames: CustomGame[] = JSON.parse(saved)
          const matchedGame = savedGames.find(g => g.name === gameName)
          if (matchedGame) {
            gameInstallPath = matchedGame.directory
          }
        }
        
        await invoke('create_game_status', {
          gameName: gameName,
          gamePath: gamePath,
          launchOptions: '',
          gameInstallPath: gameInstallPath || undefined
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
      <h2>游戏列表</h2>
      <NSpace align="center">
        <NStatistic label="已添加游戏" :value="games.length" />
        <NButton type="primary" @click="showAddGameDialog">
          添加游戏
        </NButton>
      </NSpace>
    </div>

    <div class="panel-body">
      <NModal
        v-model:show="showAddDialog"
        preset="card"
        title="添加游戏"
        style="width: 600px; max-width: 90vw;"
        :bordered="false"
        size="huge"
        :segmented="{
          content: 'soft',
          footer: 'soft'
        }"
      >
        <NForm>
          <NFormItem label="游戏名称" required :validation-status="errors.name ? 'error' : undefined" :feedback="errors.name">
            <NInput
              v-model:value="newGame.name"
              placeholder="输入游戏名称..."
              @blur="validateName"
              @input="validateName"
            />
          </NFormItem>

          <NFormItem label="游戏目录" required :validation-status="errors.directory ? 'error' : undefined" :feedback="errors.directory">
            <NSpace style="width: 100%">
              <NInput
                v-model:value="newGame.directory"
                placeholder="选择游戏安装目录..."
                readonly
                style="flex: 1"
              />
              <NButton @click="selectDirectory">浏览</NButton>
            </NSpace>
          </NFormItem>

          <NFormItem label="绑定MikuGame游戏（可选）">
            <div style="width: 100%">
              <NCard v-if="selectedMikuGame" size="small" :bordered="false" style="background: #f5f5f5">
                <NSpace justify="space-between" align="center">
                  <NSpace align="center">
                    <NImage
                      v-if="selectedMikuGame.cover_image_url"
                      :src="selectedMikuGame.cover_image_url"
                      width="60"
                      height="60"
                      object-fit="cover"
                      style="border-radius: 8px"
                    />
                    <div>
                      <div style="font-weight: 600; margin-bottom: 4px">{{ selectedMikuGame.title }}</div>
                      <div style="font-size: 12px; color: #666; margin-bottom: 4px">{{ selectedMikuGame.version || '未知版本' }}</div>
                      <NTag size="small" type="info">{{ selectedMikuGame.game_type }}</NTag>
                    </div>
                  </NSpace>
                  <NButton type="error" size="small" @click="clearMikuGameSelection">
                    取消绑定
                  </NButton>
                </NSpace>
              </NCard>

              <div v-else>
                <NButton type="default" block @click="showMikuGameSearch = !showMikuGameSearch">
                  {{ showMikuGameSearch ? '隐藏搜索' : '搜索MikuGame游戏' }}
                </NButton>

                <NCard v-if="showMikuGameSearch" size="small" style="margin-top: 12px" :bordered="false">
                  <NSpace vertical style="width: 100%">
                    <NSpace style="width: 100%">
                      <NInput
                        v-model:value="mikuGameSearchKeyword"
                        placeholder="输入游戏名称搜索..."
                        style="flex: 1"
                        @keyup.enter="searchMikuGames"
                      />
                      <NButton
                        type="primary"
                        :disabled="isSearchingMikuGame || !mikuGameSearchKeyword.trim()"
                        :loading="isSearchingMikuGame"
                        @click="searchMikuGames"
                      >
                        搜索
                      </NButton>
                    </NSpace>

                    <NScrollbar v-if="mikuGameSearchResults.length > 0" style="max-height: 300px">
                      <NSpace vertical style="width: 100%">
                        <NCard
                          v-for="result in mikuGameSearchResults"
                          :key="result.id"
                          size="small"
                          hoverable
                          style="cursor: pointer"
                          @click="selectMikuGame(result)"
                        >
                          <NSpace align="center">
                            <NImage
                              v-if="result.cover_image_url"
                              :src="result.cover_image_url"
                              width="50"
                              height="50"
                              object-fit="cover"
                              style="border-radius: 4px"
                            />
                            <div style="flex: 1">
                              <div style="font-weight: 600; margin-bottom: 4px">{{ result.title }}</div>
                              <div style="font-size: 12px; color: #666; margin-bottom: 4px">{{ result.version || '未知版本' }}</div>
                              <NSpace>
                                <NTag v-for="tag in result.tags.slice(0, 3)" :key="tag" size="small">{{ tag }}</NTag>
                              </NSpace>
                            </div>
                          </NSpace>
                        </NCard>
                      </NSpace>
                    </NScrollbar>

                    <NEmpty v-else-if="mikuGameSearchKeyword && !isSearchingMikuGame" description="未找到相关游戏" size="small" />
                  </NSpace>
                </NCard>
              </div>
            </div>
          </NFormItem>
        </NForm>

        <template #footer>
          <NSpace justify="end">
            <NButton @click="cancelAdd">取消</NButton>
            <NButton type="primary" :disabled="!isFormValid" :loading="isLoading" @click="addGame">
              添加游戏
            </NButton>
          </NSpace>
        </template>
      </NModal>

      <NEmpty v-if="games.length === 0" description="暂无游戏，点击上方按钮添加您的第一个游戏" size="large" style="padding: 80px 0" />

      <div v-else class="games-grid">
        <NCard v-for="game in games" :key="game.id" hoverable style="cursor: pointer; position: relative" @click="navigateToGame(game.id)">
          <template #cover>
              <div style="position: relative; height: 150px; background: linear-gradient(135deg, #f5f5f5 0%, #e5e5e5 100%)">
                <img
                  v-if="game.coverImage || (game.images && game.images.length > 0)"
                  :src="game.coverImage || (game.images && game.images[0]) || ''"
                  :alt="game.name"
                  loading="lazy"
                  style="width: 100%; height: 100%; object-fit: contain; background: #fafafa"
                  @error="handleImageError"
                />
                <div v-else style="display: flex; align-items: center; justify-content: center; height: 100%; color: #ccc">
                  <svg width="80" height="80" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </div>

                <NBadge v-if="game.mikuGameId" value="已绑定" type="info" style="position: absolute; top: 8px; right: 8px" />

                <NButton
                  circle
                  type="error"
                  size="small"
                  style="position: absolute; top: 8px; left: 8px; opacity: 0; transition: opacity 0.2s"
                  class="delete-btn"
                  @click.stop="removeGame(game.id)"
                >
                  <template #icon>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M3 6H5H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                      <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </template>
                </NButton>
              </div>
            </template>

            <div>
              <h4 style="margin: 0 0 8px 0; font-size: 16px; font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap" :title="game.name">
                {{ game.name }}
              </h4>
              <p style="margin: 0 0 12px 0; font-size: 12px; color: #999; font-family: monospace; overflow: hidden; text-overflow: ellipsis; white-space: nowrap" :title="game.directory">
                {{ game.directory }}
              </p>

              <NSpace vertical size="small">
                <div v-if="game.lastPlayed" style="display: flex; align-items: center; gap: 8px; font-size: 12px; color: #666">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
                    <path d="M12 6V12L16 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                  </svg>
                  <span>{{ formatDate(game.lastPlayed) }}</span>
                </div>
                <div v-if="game.playTime" style="display: flex; align-items: center; gap: 8px; font-size: 12px; color: #666">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 3L19 12L5 21V3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                  <span>{{ formatPlayTime(game.playTime) }}</span>
                </div>
              </NSpace>
            </div>
        </NCard>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mods-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  border-bottom: 1px solid #e8e8e8;
  background: linear-gradient(to right, #fafafa, #ffffff);
}

.panel-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 700;
  background: linear-gradient(135deg, #3498db, #2980b9);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.panel-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.games-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.n-card:hover .delete-btn {
  opacity: 1 !important;
}

@media (max-width: 1400px) {
  .games-grid {
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }
}

@media (max-width: 900px) {
  .games-grid {
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  }
}

@media (max-width: 600px) {
  .games-grid {
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  }
}
</style>
