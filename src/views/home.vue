<script setup lang="ts">
import { ref } from "vue";
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import DeepseekExplanation from '../components/explain/DeepseekExplanation.vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { ElMessage } from 'element-plus';
import { StarFilled } from '@element-plus/icons-vue'
const isFavorite = ref(false);

const inputText = ref("");
const currentWindow = getCurrentWindow();
let blurTimeout: ReturnType<typeof setTimeout> | null = null;
let unlisten: any = null;
let unlistenInput: any = null;
let isWindowFullyShown = false;
const isPinned = ref(false);

// 监听失去焦点事件
const listenBlur = async () => {
  unlisten = await listen('tauri://blur', () => {
    if (currentWindow.label === 'home') {
      if (isWindowFullyShown && !isPinned.value) {
        if (blurTimeout) {
          clearTimeout(blurTimeout);
        }
        blurTimeout = setTimeout(async () => {
          await currentWindow.hide();
        }, 100);
      }
    }
  });
};

// 处理收藏点击事件
const handleFavoriteClick = () => {
  isFavorite.value = !isFavorite.value;
  // TODO: 在此处添加实际的收藏/取消收藏逻辑
  ElMessage({
    message: isFavorite.value ? '收藏成功' : '已取消收藏',
    type: isFavorite.value ? 'success' : 'info',
    duration: 2000
  });
};

// 监听后端发送的文本更新事件
const listenInputUpdate = async () => {
  unlistenInput = await listen('update-input', (event: any) => {
    inputText.value = (event.payload as string).trim();
  });
};

onMounted(async () => {
  try {
    await listenInputUpdate();
    await listenBlur();
    
    // 监听获得焦点事件，取消关闭计时
    await listen('tauri://focus', () => {
      if (blurTimeout) {
        clearTimeout(blurTimeout);
      }
    });
    
    // 监听移动事件，取消关闭计时
    await listen('tauri://move', () => {
      if (blurTimeout) {
        clearTimeout(blurTimeout);
      }
    });
    
    isWindowFullyShown = true;
    
    // 添加初始化完成事件
    const appWindow = await getCurrentWebviewWindow();
    await appWindow.emit("home-ready");
    appWindow.show();

  } catch (err) {
    console.error(err)
  }
});

// 清理事件监听
onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
  if (unlistenInput) {
    unlistenInput();
  }
});

// 处理pin点击事件
const handlePinClick = () => {
  isPinned.value = !isPinned.value;
};
</script>

<template>
  <div class="h-full rounded-lg backdrop-blur-sm relative bg-[#f9f9f9]">
    <el-container class="h-full">
      <el-header class="h-10  fixed top-0 left-0 right-0 z-10 flex items-center justify-end px-2" data-tauri-drag-region='true'>
        <div class="flex items-center gap-2">
          <div class="cursor-pointer" @click="handleFavoriteClick">
            <el-icon :class="[{ 'text-yellow-400': isFavorite }, 'star-icon']" :size="25">
              <StarFilled/>
            </el-icon>
          </div>
          <div class="cursor-pointer" @click="handlePinClick">
            <img 
              src= '../assets/pin.svg'
              class="w-6 h-6"
              :alt="isPinned ? 'Pinned' : 'Not Pinned'"
              :title="isPinned ? '取消钉住窗口' : '钉住窗口'"
              :class="{ 'pin-active': isPinned }"
            />
          </div>
        </div>
      </el-header>
      <el-main class="p-3 mt-8 overflow-y-auto">
        <el-input
            v-model="inputText"
            class="w-full text-sm outline-none mb-2"
            resize="none"
            :rows="4"
            type="textarea"
        />
        <DeepseekExplanation :inputText="inputText" />
      </el-main>
    </el-container>
  </div>
</template>


<style>
:global(body) {
  background: transparent;
  height: 100vh;
  overflow: hidden;
}

.el-icon {
  opacity: 0.4;
  transition: all 0.15s ease;
}

.el-icon:hover {
  opacity: 0.6;
}

.text-yellow-400 {
  opacity: 0.8;
}

.md-editor-preview-wrapper {
  @apply relative flex-1 box-border overflow-auto px-4;
}

.md-editor-preview {
  @apply text-sm break-all overflow-hidden;
}

.md-editor-preview ol, .md-editor-preview ul {
  @apply pl-3;
}

.options-section {
  @apply flex-1 flex flex-col gap-2.5 w-full;
}

.option-item {
  @apply rounded-lg overflow-hidden bg-gray-50;
}

.option-header {
  @apply text-sm py-2 px-3 flex justify-between items-center bg-[#ededed] cursor-pointer;
}

.arrow {
  @apply w-3 h-3 transition-transform duration-300 ease-in-out;
}

.arrow.active {
  @apply rotate-180;
}

.custom-md-preview {
  @apply w-full text-xs bg-gray-100;
}

.katex-error {
  @apply text-black;
}

.loading-icon {
  @apply inline-flex items-center;
}

.spinner {
  @apply w-3 h-3 border-2 border-gray-200 border-t-blue-500 rounded-full animate-spin;
}

.header-actions {
  @apply flex items-center gap-3;
}

.copy-button {
  @apply px-3 py-1 bg-gray-50 border border-gray-300 rounded-lg cursor-pointer text-sm text-gray-600
         hover:bg-gray-100 hover:border-gray-400 active:bg-gray-200 active:translate-y-px
         flex items-center justify-center min-w-[80px] h-7 transition-all duration-200;
}

.el-header {
  display: flex;
  align-items: center;
  padding: 0 8px;
}

img {
  opacity: 0.4;
  transition: all 0.15s ease;
}

.pin-active {
  filter: invert(40%) sepia(60%) saturate(1000%) hue-rotate(190deg) brightness(100%) contrast(100%);
  transform: rotate(45deg);
  opacity: 0.8;
}

.star-icon {
  opacity: 0.2;
  transition: all 0.15s ease;
}

.star-icon.text-yellow-400 {
  opacity: 0.8;
  color: #facc15;
}

/* 添加以下样式来禁止选中 */
.h-full {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

/* 允许输入框内的文本可以选中 */
.el-input textarea {
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
}
</style>
