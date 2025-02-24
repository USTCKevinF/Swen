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

const props = defineProps<{
  inputText: string
}>();

const thisRequestVersion = ref(Date.now());
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

async function getDeepseekExplanation() {
  if (!props.inputText) return;
  
  // 生成新的时间戳作为请求ID
  thisRequestVersion.value = Date.now();
  const currentVersion = thisRequestVersion.value;
  
  if (fetchStreamUnlisten) {
    fetchStreamUnlisten();
    fetchStreamUnlisten = null;
  }
  
  isLoading.value = true;
  deepseekResponse.value = "";
  const systemPrompt = "你是一个包罗万象的知识专家，擅长于给用户解释其提出的概念，用户是一名好学且好奇的学生，会提供一些名词或者概念给你。规则：- 你需要详细地解答，并且以易懂的方式叙述你的观点 - 你的目标是让用户更深入的理解其提供的概念 - 无论用户提供的是什么语言，均用中文进行回复 - 输出完解释之后立刻停止，不要说类似如果你有更多的问题，欢迎继续提问的话.Please use $ instead of \\( and \\) for LaTeX math expressions.";

  try {
    fetchStreamUnlisten = await listen('fetch-stream-data', (event: any) => {
      // 只处理匹配当前版本的响应
      console.log('currentVersion: ', currentVersion, 'event.payload.request_id: ', event.payload.request_id);
      if (event.payload.request_id !== currentVersion) {
        console.log('忽略过期请求响应:', event.payload.request_id, '当前版本:', currentVersion);
        return;
      }
      const { message } = event.payload;
      if (message && typeof message === 'string') {
        const messages = message.split('\n\n');
        messages.forEach(msg => {
          if (!msg.trim()) return;
          try {
            // 检测到 [DONE] 时，更新状态并清理监听器
            if (msg.includes('[DONE]')) {
              isLoading.value = false;
              if (fetchStreamUnlisten) {
                fetchStreamUnlisten();
                fetchStreamUnlisten = null;
              }
              return;
            }
            const jsonStr = msg.replace(/^data: /, '');
            const data = JSON.parse(jsonStr);
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
          { role: "user", content: props.inputText }
        ],
        stream: true
      }),
      requestId: currentVersion,
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
  getDeepseekExplanation();
}

let unlistenInput: (() => void) | null = null;
const listenInputUpdate = async () => {
  unlistenInput = await listen('update-input', () => {
    // 当输入更新时，清空旧内容并发起新的解释请求
    deepseekResponse.value = "";
    if (props.inputText.trim()) {
      getDeepseekExplanation();
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