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
import { ref, onMounted, onUnmounted } from "vue";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import { ArrowRight, DocumentCopy, RefreshRight } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { useConfig } from '../../composables/useConfig'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  inputText: string
}>();

// 新增请求版本号，全局自增，每次请求时作为唯一标识
let requestVersion = 0;
const currentRequestVersion = ref(0);

const deepseekResponse = ref("");
const isLoading = ref(false);

// 当前流事件监听取消函数
let fetchStreamUnlisten: (() => void) | null = null;

const { property: baseURL } = useConfig('llm.baseURL', '')
const { property: apiKey } = useConfig('llm.apiKey', '')
const { property: model } = useConfig('llm.model', '')

async function getDeepseekExplanation() {
  if (!props.inputText) return;
  
  // 更新请求版本号
  requestVersion++;
  const thisRequestVersion = requestVersion;
  currentRequestVersion.value = thisRequestVersion;
  
  // 取消旧的流事件监听（如果存在）
  if (fetchStreamUnlisten) {
    fetchStreamUnlisten();
    fetchStreamUnlisten = null;
  }
  
  isLoading.value = true;
  deepseekResponse.value = "";
  const systemPrompt = "你是一个包罗万象的知识专家，擅长于给用户解释其提出的概念，用户是一名好学且好奇的学生，会提供一些名词或者概念给你。规则：- 你需要详细地解答，并且以易懂的方式叙述你的观点 - 你的目标是让用户更深入的理解其提供的概念 - 无论用户提供的是什么语言，均用中文进行回复 - 输出完解释之后立刻停止，不要说类似如果你有更多的问题，欢迎继续提问的话.Please use $ instead of \\( and \\) for LaTeX math expressions.";

  try {
    // 注册流事件监听，并在事件回调中检验 request_id
    fetchStreamUnlisten = await listen('fetch-stream-data', (event: any) => {
      // 检查返回的 request_id 是否与当前请求匹配，不匹配则忽略该数据
      console.log('thisRequestVersion:', thisRequestVersion);
      console.log('event.payload:', event.payload);
      if (event.payload.request_id !== thisRequestVersion) return;
      const { message } = event.payload;
      console.log('Received stream data:', message);
      if (message && typeof message === 'string') {
        // 根据两个换行符分割数据
        const messages = message.split('\n\n');
        messages.forEach(msg => {
          if (!msg.trim()) return;
          try {
            if (msg.includes('[DONE]')) return;
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
    
    // 调用后端流接口，同时传入 request_id 参数
    await invoke('receive_stream', {
      url: `${baseURL.value}/chat/completions`,
      cookie: `Bearer ${apiKey.value}`,
      prompt: JSON.stringify({
        model: model.value,
        messages: [
          { role: "system", content: systemPrompt },
          { role: "user", content: props.inputText }
        ],
        stream: true
      }),
      requestId: thisRequestVersion,  // 将当前请求的唯一标识传给后端
    });
  } catch (error) {
    console.error('流处理错误:', error);
    deepseekResponse.value = "抱歉，解释生成失败，请稍后重试。";
  } finally {
    isLoading.value = false;
    // 请求完成后取消当前监听，避免残留消息
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