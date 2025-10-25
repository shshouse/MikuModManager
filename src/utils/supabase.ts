import { createClient, SupabaseClient } from '@supabase/supabase-js'
import type { MikuGameMinimal, MikuGameListItem, MikuGameImages } from '../types/mikugame'
import type { GameType } from '../types/mikugame'

export type { MikuGameMinimal, MikuGameListItem, MikuGameImages, GameType } from '../types/mikugame'

const SUPABASE_URL = import.meta.env.VITE_SUPABASE_URL
const SUPABASE_ANON_KEY = import.meta.env.VITE_SUPABASE_ANON_KEY
let supabaseClient: SupabaseClient | null = null

export function getSupabaseClient(): SupabaseClient {
  if (!supabaseClient) {
    supabaseClient = createClient(SUPABASE_URL, SUPABASE_ANON_KEY)
  }
  return supabaseClient
}

export function updateSupabaseConfig(url: string, anonKey: string) {
  supabaseClient = createClient(url, anonKey)
}

const MINIMAL_FIELDS = 'id, title, image_urls, cover_image_url, version, tags'

export async function fetchGamesFromTable(
  tableName: GameType
): Promise<MikuGameMinimal[]> {
  const client = getSupabaseClient()
  
  const { data, error } = await client
    .from(tableName)
    .select(MINIMAL_FIELDS)
    .eq('status', 'active')
    .order('created_at', { ascending: false })
  
  if (error) {
    throw new Error(`获取游戏列表失败: ${error.message}`)
  }
  
  return data || []
}

export async function fetchAllGames(): Promise<MikuGameListItem[]> {
  const gameTypes: GameType[] = ['games', 'h_games', 'galgames']
  const allGames: MikuGameListItem[] = []
  
  for (const gameType of gameTypes) {
    try {
      const games = await fetchGamesFromTable(gameType)
      const listItems = games.map(game => ({
        id: game.id,
        title: game.title,
        cover_image_url: game.cover_image_url,
        game_type: gameType,
        version: game.version,
        tags: game.tags || []
      }))
      allGames.push(...listItems)
    } catch (error) {
      console.error(`获取 ${gameType} 失败:`, error)
    }
  }
  
  return allGames
}

export async function fetchGameById(
  gameId: string,
  gameType: GameType
): Promise<MikuGameImages | null> {
  const client = getSupabaseClient()
  
  const { data, error } = await client
    .from(gameType)
    .select('id, image_urls')
    .eq('id', gameId)
    .single()
  
  if (error) {
    console.error('获取游戏图片失败:', error)
    return null
  }
  
  return data
}

export async function searchGames(
  keyword: string,
  gameType?: GameType
): Promise<MikuGameListItem[]> {
  const gameTypes: GameType[] = gameType ? [gameType] : ['games', 'h_games', 'galgames']
  const allGames: MikuGameListItem[] = []
  
  for (const type of gameTypes) {
    try {
      const client = getSupabaseClient()
      const { data, error } = await client
        .from(type)
        .select('id, title, cover_image_url, version, tags')
        .eq('status', 'active')
        .ilike('title', `%${keyword}%`)
        .limit(20)
      
      if (error) {
        console.error(`搜索 ${type} 失败:`, error)
        continue
      }
      
      const listItems = (data || []).map(game => ({
        id: game.id,
        title: game.title,
        cover_image_url: game.cover_image_url,
        game_type: type,
        version: game.version,
        tags: game.tags || []
      }))
      allGames.push(...listItems)
    } catch (error) {
      console.error(`搜索 ${type} 失败:`, error)
    }
  }
  
  return allGames
}

export async function checkSupabaseConnection(): Promise<boolean> {
  try {
    const client = getSupabaseClient()
    const { error } = await client.from('games').select('id').limit(1)
    return !error
  } catch {
    return false
  }
}

