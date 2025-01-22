<script setup lang="ts">
import { ref,onMounted } from 'vue'
import { RouterView, useRouter } from 'vue-router'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const router = useRouter()

let appWindow: any
// 路由映射
const routeMap: Record<string, string> = {
  home: '/',
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
