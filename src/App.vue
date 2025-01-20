<script setup lang="ts">
import { watch, onMounted } from 'vue';
import { ref } from 'vue'
import { RouterView, useRouter } from 'vue-router'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const router = useRouter()

let appWindow: any
// 路由映射
const routeMap: Record<string, string> = {
  home: '/',
  settings: '/settings',
}

const windowInfo = ref('')
const currentLabel = ref('')
// 添加获取窗口信息的方法
const getWindowInfo = async () => {
  try {
    const window = await getCurrentWebviewWindow()
    windowInfo.value = JSON.stringify(window, null, 2)
  } catch (error) {
    console.error('获取窗口信息失败:', error)
    windowInfo.value = '获取失败: ' + error
  }
}

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
    
    // 监听标签变化
    watch(() => appWindow.label, (newLabel: string) => {
      updateRouteByLabel(newLabel)
    })
  } catch (error) {
    console.error('获取当前窗口失败:', error)
  }
})

</script>

<template>
  <div class="window-info">
    <div class="label-info">
      当前窗口标签: {{ currentLabel }}
    </div>
    <button @click="getWindowInfo">获取窗口信息</button>
    <pre v-if="windowInfo">{{ windowInfo }}</pre>
  </div>
  <RouterView />
</template>

<style>
.md-editor-preview-wrapper {
    position: relative;
    flex: 1;
    box-sizing: border-box;
    overflow: auto;
    padding: 0px 15px;
}

.md-editor-preview {
  font-size: 13px !important;
  word-break: break-all !important;
  overflow: hidden !important;
}

.label-info {
  margin-bottom: 10px;
  padding: 8px;
  background-color: #f5f5f5;
  border-radius: 4px;
}
</style>
