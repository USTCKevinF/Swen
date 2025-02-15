<script setup lang="ts">
import { ref } from "vue";
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import DeepseekExplanation from '../components/explain/DeepseekExplanation.vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

const inputText = ref("");
const currentWindow = getCurrentWindow();
let blurTimeout: ReturnType<typeof setTimeout> | null = null;
let unlisten: any = null;
let unlistenInput: any = null;
let isWindowFullyShown = false;

// 监听失去焦点事件
const listenBlur = async () => {
  unlisten = await listen('tauri://blur', () => {
    if (currentWindow.label === 'home') {
      // 增加一个标志位，避免窗口刚显示就关闭
      if (isWindowFullyShown) {  
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

// 监听后端发送的文本更新事件
const listenInputUpdate = async () => {
  unlistenInput = await listen('update-input', (event: any) => {
    inputText.value = event.payload as string;
  });
};

onMounted(async () => {
  try {
    currentWindow.show();
    await listenBlur();
    await listenInputUpdate();
    
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
</script>

<template>
  <div class="h-full rounded-lg bg-transparent backdrop-blur-sm">
    <el-container>
      <el-header class="h-8" data-tauri-drag-region='true'></el-header>
      <el-main class="p-3">
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
}

.md-editor-preview-wrapper {
  @apply relative flex-1 box-border overflow-auto px-4;
}
.md-editor-preview {
  @apply text-sm break-all overflow-hidden;
}

.options-section {
  @apply flex-1 flex flex-col gap-2.5 w-full;
}

.option-item {
  @apply rounded-lg overflow-hidden bg-gray-50;
}

.option-header {
  @apply text-sm py-2 px-4 flex justify-between items-center bg-[#ededed] cursor-pointer;
}

.arrow {
  @apply w-3 h-3 transition-transform duration-300 ease-in-out;
}

.arrow.active {
  @apply rotate-180;
}

.custom-md-preview {
  @apply w-full text-xs bg-gray-50;
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
</style>
