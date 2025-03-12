<template>
  <div>
    <img
      ref="imgRef"
      class="fixed top-0 left-0 w-full select-none"
      :src="imgUrl"
      :draggable="false"
      @load="handleImageLoad"
    />
    
    <div
      v-show="isDown"
      class="fixed border border-solid border-sky-500 bg-[#2080f020]"
      :style="selectionStyle"
    />
    
    <div
      class="fixed top-0 left-0 bottom-0 right-0 cursor-crosshair select-none"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
    />
  </div>
</template>

<script setup>
import { ref, computed, onUnmounted, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { appCacheDir, join } from '@tauri-apps/api/path'
import { getCurrentWindow, currentMonitor } from '@tauri-apps/api/window';
import { convertFileSrc } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'

const currentWindow = getCurrentWindow()
const imgRef = ref(null)
const imgUrl = ref('')
const isMoved = ref(false)
const isDown = ref(false)
const mouseDownX = ref(0)
const mouseDownY = ref(0)
const mouseMoveX = ref(0)
const mouseMoveY = ref(0)

const selectionStyle = computed(() => ({
  top: `${Math.min(mouseDownY.value, mouseMoveY.value)}px`,
  left: `${Math.min(mouseDownX.value, mouseMoveX.value)}px`,
  bottom: `${window.screen.height - Math.max(mouseDownY.value, mouseMoveY.value)}px`,
  right: `${window.screen.width - Math.max(mouseDownX.value, mouseMoveX.value)}px`,
}))

// 初始化截图
const initScreenshot = async () => {
  try {
    const monitor = await currentMonitor()
    const position = monitor.position
    
    const appCacheDirPath = await appCacheDir()
    const filePath = await join(appCacheDirPath, 'Swen_screenshot.png')
    const assetUrl = convertFileSrc(filePath)
    
    // 使用新的Image对象预加载
    const img = new Image()
    await new Promise((resolve, reject) => {
      img.onload = resolve
      img.onerror = reject
      img.src = assetUrl
    })
    
    // 图片加载完成后再设置URL并显示窗口
    imgUrl.value = assetUrl
    await currentWindow.show()
    await currentWindow.setFocus()
    
  } catch (error) {
    console.error('截图初始化失败:', error)
    await currentWindow.close()
    throw error
  }
}

const handleImageLoad = async () => {
  console.log('图片加载完成')
  if (imgUrl.value && imgRef.value?.complete) {
    console.log('图片尺寸:', imgRef.value.naturalWidth, 'x', imgRef.value.naturalHeight)
    try {
      await currentWindow.show()
      await currentWindow.setFocus()
      await currentWindow.setResizable(false)
    } catch (error) {
      console.error('窗口操作失败:', error)
    }
  }
}

const handleMouseDown = async (e) => {
  if (e.buttons === 1) {
    isDown.value = true
    mouseDownX.value = e.clientX
    mouseDownY.value = e.clientY
    mouseMoveX.value = e.clientX
    mouseMoveY.value = e.clientY
  } else {
    await currentWindow.close()
  }
}

const handleMouseMove = (e) => {
  if (isDown.value) {
    isMoved.value = true
    mouseMoveX.value = e.clientX
    mouseMoveY.value = e.clientY
  }
}

const handleMouseUp = async (e) => {
  await currentWindow.hide()
  isDown.value = false
  isMoved.value = false

  const imgWidth = imgRef.value.naturalWidth
  const dpi = imgWidth / window.screen.width
  const left = Math.floor(Math.min(mouseDownX.value, e.clientX) * dpi)
  const top = Math.floor(Math.min(mouseDownY.value, e.clientY) * dpi)
  const right = Math.floor(Math.max(mouseDownX.value, e.clientX) * dpi)
  const bottom = Math.floor(Math.max(mouseDownY.value, e.clientY) * dpi)
  const width = right - left
  const height = bottom - top

  if (width <= 0 || height <= 0) {
    console.warn('截图区域太小')
    await currentWindow.close()
  } else {
    await invoke('cut_image', { left, top, width, height })
    await emit('success')
    await currentWindow.close()
  }
}

// 组件挂载时初始化截图
onMounted(async () => {
  try {
    await initScreenshot()
  } catch (error) {
    console.error('初始化截图失败:', error)
    await currentWindow.close()
  }
})

// 添加组件卸载时的清理逻辑
onUnmounted(async () => {
  if (currentWindow) {
    await currentWindow.close()
  }
})
</script>

<style scoped>
img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  opacity: 1 !important; /* 确保图片不透明 */
}
</style>
