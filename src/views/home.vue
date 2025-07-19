<script setup lang="ts">
import { ElMessage } from 'element-plus';
import { listen } from '@tauri-apps/api/event';
// @ts-ignore 忽略Vue导入错误
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { COMPREHENSIVE_EXPLANATION_PROMPT, EXPLANATION_SUMMARY_PROMPT } from '../utils/prompts';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import DeepseekExplanation from '../components/explain/DeepseekExplanation.vue';
import { calculateTextComplexity } from '../utils/textAnalysis';

const isFavorite = ref(false);

const inputText = ref("");
const messages = ref<Array<{role: string, content: string}>>([]);
const currentRequestId = ref(0);
const currentWindow = getCurrentWindow();
let blurTimeout: ReturnType<typeof setTimeout> | null = null;
let unlisten: any = null;
let unlistenInput: any = null;
let unlistenFocus: any = null;
let unlistenMove: any = null;
let isWindowFullyShown = false;
const isPinned = ref(false);
const isWindowHiding = ref(false);


// 添加ref用于获取DeepseekExplanation组件实例
const deepseekExplanationRef = ref();

// 监听失去焦点事件
const setupWindowBlurListener = async () => {
  unlisten = await listen('tauri://blur', () => {
    if (currentWindow.label === 'home') {
      if (isWindowFullyShown && !isPinned.value && !isWindowHiding.value) {
        // 清理已存在的超时
        if (blurTimeout) {
          clearTimeout(blurTimeout);
          blurTimeout = null;
        }
        
        // 设置窗口隐藏状态
        isWindowHiding.value = true;
        
        // 立即隐藏窗口，然后清理内容
        (async () => {
          try {
            await currentWindow.hide();
            
            // 窗口隐藏后再清理内容
            deepseekExplanationRef.value?.cancelFetchStream();
            deepseekExplanationRef.value?.saveMultiChatHistory();
            deepseekExplanationRef.value?.clearState();
            inputText.value = "";
            messages.value = [];
          } catch (error) {
            console.error('Failed to hide window:', error);
          } finally {
            isWindowHiding.value = false;
          }
        })();
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
    // 清空组件状态
    deepseekExplanationRef.value?.clearState();
    const { payload, requestId } = event.payload;
    // 只处理最新的请求
    if (requestId && requestId > currentRequestId.value) {
      currentRequestId.value = requestId;
      inputText.value = payload.trim();
      let wordCount = calculateTextComplexity(payload.trim());
      console.log('wordCount', wordCount);
      if (wordCount > 600) {
        // 更新messages列表
        messages.value = [
          { role: "system", content: EXPLANATION_SUMMARY_PROMPT },
          { role: "user", content: payload.trim() }
        ];
      } else {
        // 更新messages列表
        messages.value = [
          { role: "system", content: COMPREHENSIVE_EXPLANATION_PROMPT },
          { role: "user", content: payload.trim() }
        ];
      }
    }
  });
};

// 取消隐藏窗口的公共函数
const cancelHideWindow = () => {
  if (blurTimeout) {
    clearTimeout(blurTimeout);
    blurTimeout = null;
  }
  isWindowHiding.value = false;
};

onMounted(async () => {
  try {
    await listenInputUpdate();
    await setupWindowBlurListener();
    
    // 监听获得焦点事件，取消关闭计时
    unlistenFocus = await listen('tauri://focus', () => {
      cancelHideWindow();
    });
    
    // 监听移动事件，取消关闭计时
    unlistenMove = await listen('tauri://move', () => {
      cancelHideWindow();
    });
    
    isWindowFullyShown = true;
    
    await new Promise(resolve => setTimeout(resolve, 200));

    // 添加初始化完成事件
    const appWindow = await getCurrentWebviewWindow();
    await appWindow.emit("home-ready");
    console.log('home-ready');
    
  } catch (err) {
    console.error('Failed to initialize window listeners:', err);
  }
});

// 清理事件监听
onUnmounted(() => {
  // 清理所有超时
  if (blurTimeout) {
    clearTimeout(blurTimeout);
    blurTimeout = null;
  }
  
  // 清理所有事件监听器
  if (unlisten) {
    unlisten();
  }
  if (unlistenInput) {
    unlistenInput();
  }
  if (unlistenFocus) {
    unlistenFocus();
  }
  if (unlistenMove) {
    unlistenMove();
  }
});

// 处理pin点击事件
const handlePinClick = () => {
  isPinned.value = !isPinned.value;
  // 如果取消钉住且窗口失去焦点，则立即隐藏窗口
  if (!isPinned.value && !document.hasFocus()) {
    cancelHideWindow();
    currentWindow.hide();
  }
};
</script>

<template>
  <div class="h-full rounded-lg backdrop-blur-sm relative bg-[#f9f9f9]">
    <el-container class="h-full">
      <el-header class="h-10 fixed top-0 left-0 right-0 z-10 flex items-center justify-end px-2" data-tauri-drag-region='true'>
        <div class="flex items-center gap-2">
          <div class="cursor-pointer" @click="handleFavoriteClick">
            <!-- <el-icon :class="[{ 'text-yellow-400': isFavorite }, 'star-icon']" :size="25">
              <StarFilled/>
            </el-icon> -->
          </div>
          <div class="cursor-pointer" @click="handlePinClick">
            <img 
              src="/logo/pin.svg"
              class="w-6 h-6 opacity-30 hover:opacity-60 transition-opacity duration-200"
              :alt="isPinned ? 'Pinned' : 'Not Pinned'"
              :title="isPinned ? '取消钉住窗口' : '钉住窗口'"
              :class="{ 'pin-active': isPinned }"
            />
          </div>
        </div>
      </el-header>
      <el-main class="p-3 mt-8 overflow-y-auto">
        <DeepseekExplanation 
          ref="deepseekExplanationRef"
          :messages="messages"
        />
      </el-main>
    </el-container>
  </div>
</template>


<style>
:global(body) {
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
