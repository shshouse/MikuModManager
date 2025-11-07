<script setup lang="ts">
import { ref, watch } from 'vue'
import { NLayoutSider, NMenu } from 'naive-ui'
import type { MenuOption } from 'naive-ui'

const props = defineProps<{ activeTab: string }>()

const emit = defineEmits<{ tabChange: [tab: string] }>()

const menuOptions: MenuOption[] = [
  { label: '游戏库', key: 'mods' },
  { label: '找模组', key: 'find-mods' },
  { label: '找游戏', key: 'download' },
  { label: '关于', key: 'about' }
]

const currentTab = ref(props.activeTab)

watch(
  () => props.activeTab,
  value => {
    if (currentTab.value !== value) {
      currentTab.value = value
    }
  }
)

watch(
  () => currentTab.value,
  value => emit('tabChange', value)
)
</script>

<template>
  <NLayoutSider
    bordered
    collapse-mode="width"
    :native-scrollbar="false"
    :show-trigger="false"
    :width="240"
    style="height: 100vh; flex-shrink: 0"
  >
    <div class="sidebar-header">
      <img src="/MikuModManager.png" alt="Miku Mod Manager" class="logo-image" />
    </div>
    <NMenu
      v-model:value="currentTab"
      :options="menuOptions"
    />
  </NLayoutSider>
</template>

<style scoped>
.sidebar-header {
  padding: 24px 16px;
  text-align: center;
  border-bottom: 1px solid #f0f0f0;
  background: linear-gradient(135deg, #EBF5FB 0%, #D6EAF8 100%);
}

.logo-image {
  max-width: 100%;
  height: auto;
  max-height: 60px;
  object-fit: contain;
}
</style>
