export interface MikuGameMinimal {
  id: string
  title: string
  image_urls: string[]
  cover_image_url: string | null
  version: string | null
  tags: string[]
}

export type GameType = 'games' | 'h_games' | 'galgames'

export interface GameCategory {
  id: string
  name: string
  description: string | null
  icon_url: string | null
  created_at: string
}

export interface MikuGameListItem {
  id: string
  title: string
  cover_image_url: string | null
  game_type: GameType
  version: string | null
  tags: string[]
}

export interface MikuGameImages {
  id: string
  image_urls: string[]
}

