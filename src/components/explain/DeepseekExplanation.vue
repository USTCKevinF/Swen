<template>
  <div class="option-item">
    <!-- <div class="option-header" @click="getDeepseekExplanation">
      <span>DeepSeek 解释</span>
      <div class="header-actions">
        <span class="loading-icon" v-if="isLoading">
          <span class="spinner"></span>
        </span>
        <el-icon v-else class="arrow"><ArrowRight /></el-icon>
      </div>
    </div> -->
    <div v-if="deepseekResponse" class="flex flex-col  selectable-text">
      <MdPreview 
        :modelValue="deepseekResponse"
        :preview-theme="'default'"
        class="custom-md-preview"
      />
      <div v-if="!isLoading" class="relative flex items-center gap-2.5 pl-3.5 pb-2.5 bg-gray-100">
        <div class="relative inline-block">
          <el-icon 
            @click="copyToClipboard"
            class="w-6 h-6 cursor-pointer opacity-90 hover:opacity-100"
          ><DocumentCopy /></el-icon>
          <span class="absolute bottom-full left-1/2 -translate-x-1/2 bg-[rgba(22,22,22,0.7)] text-white px-2 py-1 rounded text-[10px] whitespace-nowrap opacity-0 invisible transition-opacity duration-200 group-hover:opacity-100 group-hover:visible">复制</span>
        </div>
        <div class="relative inline-block">
          <el-icon 
            @click="handleRedo"
            class="w-6 h- cursor-pointer opacity-90 hover:opacity-100"
          ><RefreshRight /></el-icon>
          <span class="absolute bottom-full left-1/2 -translate-x-1/2 bg-[rgba(22,22,22,0.7)] text-white px-2 py-1 rounded text-[10px] whitespace-nowrap opacity-0 invisible transition-opacity duration-200 group-hover:opacity-100 group-hover:visible">重新生成</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import { DocumentCopy, RefreshRight } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useConfig } from '../../composables/useConfig'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core';
import { COMPREHENSIVE_EXPLANATION_PROMPT } from '../../utils/prompts';

const systemPrompt = COMPREHENSIVE_EXPLANATION_PROMPT;

const props = defineProps<{
  inputText: string
}>();

const currentRequestId = ref(0);
const deepseekResponse = ref("");
const isLoading = ref(false);

// 添加一个用于跟踪内容变化的 watch
watch(deepseekResponse, () => {
  nextTick(() => {
    // 获取 el-main 元素
    const mainElement = document.querySelector('.el-main');
    if (mainElement) {
      mainElement.scrollTop = mainElement.scrollHeight;
    }
  });
});

// 当前流事件监听取消函数
let fetchStreamUnlisten: (() => void) | null = null;

const { property: baseURL } = useConfig('llm.baseURL', '')
const { property: apiKey } = useConfig('llm.apiKey', '')
const { property: model } = useConfig('llm.model', '')

async function getDeepseekExplanation(payload: string) {
  // 生成新的时间戳作为请求ID
  if (fetchStreamUnlisten) {
    fetchStreamUnlisten();
    fetchStreamUnlisten = null;
  }
  
  isLoading.value = true;
  try {
    const timestampId = Date.now();
    fetchStreamUnlisten = await listen('fetch-stream-data', (event: any) => {
      const { message, responseId } = event.payload;  
      if (!responseId || responseId < timestampId) {
        return;
      }
      if (message && typeof message === 'string') {
        const messages = message.split('\n\n');
        messages.forEach(msg => {
          if (!msg.trim()) return;
          try {
            // 检测到 [DONE] 时，更新状态并清理监听器
            if (msg.includes('[DONE]')) {
              console.log('DONE');
              isLoading.value = false;
              if (fetchStreamUnlisten) {
                fetchStreamUnlisten();
                fetchStreamUnlisten = null;
              }
              return;
            }
            const jsonStr = msg.replace(/^data: /, '');
            const data = JSON.parse(jsonStr);
            // console.log('message: ', message, 'responseId: ', responseId, 'timestampId: ', timestampId);

            const content = data.choices[0]?.delta?.content;
            if (content) {
              deepseekResponse.value += content;
            }
          } catch (e) {
            console.error('解析消息失败:', e, msg);
          }
        });
      }
    });
    
    await invoke('receive_stream', {
      url: `${baseURL.value}/chat/completions`,
      authToken: `Bearer ${apiKey.value}`,
      prompt: JSON.stringify({
        model: model.value,
        messages: [
          { role: "system", content: systemPrompt },
          { role: "user", content: payload }
        ],
        stream: true
      }),
    });
  } catch (error) {
    console.error('流处理错误:', error);
    deepseekResponse.value = "抱歉，解释生成失败，请稍后重试。";
    isLoading.value = false;
    // 发生错误时也需要清理监听器
    if (fetchStreamUnlisten) {
      fetchStreamUnlisten();
      fetchStreamUnlisten = null;
    }
  } finally {
    // finally 块作为最后的保险，确保在异常情况下也能清理监听器
    if (fetchStreamUnlisten) {
      fetchStreamUnlisten();
      fetchStreamUnlisten = null;
    }
  }
}

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(deepseekResponse.value);
    ElMessage({
      message: '复制成功',
      type: 'success',
    });
  } catch (err) {
    console.error('复制失败:', err);
    ElMessage({
      message: '复制失败',
      type: 'error',
    });
  }
}

function handleRedo() {
  getDeepseekExplanation(props.inputText);
}

let unlistenInput: (() => void) | null = null;
const listenInputUpdate = async () => {
  unlistenInput = await listen('update-input', (event: any) => {
    const { payload, requestId } = event.payload;
        // 只处理最新的请求
    if (payload && requestId && requestId > currentRequestId.value) {
      currentRequestId.value = requestId;
      // 当输入更新时，清空旧内容并发起新的解释请求
      deepseekResponse.value = "";
      console.log('getDeepseekExplanation and payload: ', event); 
      getDeepseekExplanation(payload);
    }
  });
};

onMounted(async () => {
  await listenInputUpdate();
});

onUnmounted(() => {
  if (unlistenInput) {
    unlistenInput();
  }
  if (fetchStreamUnlisten) {
    fetchStreamUnlisten();
  }
});
</script>

<style scoped>
.copy-button {
  display: none;
}

.selectable-text {
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
}
</style> 