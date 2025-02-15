<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { RouterView, useRouter } from 'vue-router'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { useI18n } from 'vue-i18n'
import { useConfig } from './composables/useConfig'

const router = useRouter()
const { locale } = useI18n()
const { property: language } = useConfig('language', 'zh')

// 添加 watch 来处理语言变化
watch(language, (newValue: string) => {
  if (newValue) {
    console.log("语言更新为:", newValue)
    locale.value = newValue
  }
})

let appWindow: any
// 路由映射
const routeMap: Record<string, string> = {
  default: '/',
  home: '/home',
  settings: '/settings',
}
const currentLabel = ref('')
// 根据标签更新路由
const updateRouteByLabel = (label: string) => {
  if (routeMap[label]) {
    console.log("routeMap[label]是",routeMap[label])
    router.push(routeMap[label])
    currentLabel.value = label
  }
}

onMounted(async () => {
  try {
    appWindow = await getCurrentWebviewWindow()
    // 设置初始路由
    updateRouteByLabel(appWindow.label)
  } catch (error) {
    console.error('获取当前窗口失败:', error)
  }
})

</script>

<template>
  <div class="h-full">
    <RouterView />
  </div>
</template>

<style>
html, body {
  height: 100%;
}
</style>
