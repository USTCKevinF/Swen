<template>
  <div class="option-item">
    <div class="flex flex-col gap-4">
      <!-- 对话历史 -->
      <template v-for="(item, index) in chatHistory" :key="index">
        <!-- 用户问题 -->
        <div class="text-[13px] pl-3 bg-gray-100 rounded-lg h-10 flex items-center text-gray-500 overflow-hidden">
          <div class="overflow-hidden text-ellipsis whitespace-nowrap">
            {{"Q: " + item.question }}
          </div>
        </div>
        <!-- AI回答 -->
        <div v-if="item.answer" class="flex flex-col selectable-text bg-gray-100 p-3 rounded-lg mb-4">
          <MdPreview 
            :modelValue="item.answer"
            :preview-theme="'default'"
            class="custom-md-preview"
          />
        </div>
      </template>

      <!-- 当前回答（如果正在加载） -->
      <div v-if="isLoading" class="flex flex-col selectable-text bg-gray-100 p-3 rounded-lg mb-4">
        <MdPreview 
          :modelValue="deepseekResponse"
          :preview-theme="'default'"
          class="custom-md-preview"
        />
      </div>

      <!-- 输入框区域 -->
      <div class="flex gap-2 mt-4">
        <el-input
          v-model="newQuestion"
          :placeholder="isLoading ? '正在回答中...' : '请输入您的问题'"
          :disabled="isLoading"
          @keyup.enter="handleSendQuestion"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// @ts-ignore 忽略Vue导入错误
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import { useConfig } from '../../composables/useConfig'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core';
import { saveChatHistory } from '../../utils/database';

const props = defineProps<{
  messages: Array<{role: string, content: string}>
}>();

const emit = defineEmits(['update:messages']);

interface ChatMessage {
  question: string;
  answer: string;
}

const chatHistory = ref<ChatMessage[]>([]);
const newQuestion = ref("");
const deepseekResponse = ref("");
const isLoading = ref(false);
const localMessages = ref<Array<{role: string, content: string}>>([]);

// 添加一个用于跟踪内容变化的 watch
watch(deepseekResponse, () => {
  nextTick(() => {
    const mainElement = document.querySelector('.el-main');
    if (mainElement) {
      mainElement.scrollTop = mainElement.scrollHeight;
    }
  });
});

// 监听初始messages变化
watch(() => props.messages, (newMessages: Array<{role: string, content: string}>) => {
  if (newMessages && newMessages.length > 0) {
    if (localMessages.value.length < newMessages.length) {
      localMessages.value = [...newMessages];
      const userMessage = newMessages.find((msg: {role: string, content: string}) => msg.role === "user");
      if (userMessage && chatHistory.value.length === 0) {
        getDeepseekExplanation(userMessage.content);
      }
    }
  }
}, { immediate: true, deep: true });

// 当前流事件监听取消函数
let fetchStreamUnlisten: (() => void) | null = null;

const { property: baseURL } = useConfig('llm.baseURL', '')
const { property: apiKey } = useConfig('llm.apiKey', '')
const { property: model } = useConfig('llm.model', '')
const { property: maxContextLength } = useConfig('llm.maxContextLength', 6)

async function handleSendQuestion() {
  if (!newQuestion.value.trim() || isLoading.value) return;
  
  const question = newQuestion.value.trim();
  newQuestion.value = "";
  
  // 添加用户消息到messages
  localMessages.value.push({ role: "user", content: question });
  
  // 调用API获取回答
  await getDeepseekExplanation(question);
}

async function getDeepseekExplanation(payload: string) {
  if (fetchStreamUnlisten) {
    fetchStreamUnlisten();
    fetchStreamUnlisten = null;
  }
  
  isLoading.value = true;
  deepseekResponse.value = "";
  
  // 添加新的问题到历史记录
  chatHistory.value.push({
    question: payload,
    answer: ""
  });
  
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
            if (msg.includes('[DONE]')) {
              console.log('DONE');
              isLoading.value = false;
              
              // 更新最后一条对话的答案
              if (chatHistory.value.length > 0) {
                chatHistory.value[chatHistory.value.length - 1].answer = deepseekResponse.value;
              }
              
              // 添加assistant的回复到messages
              localMessages.value.push({ role: "assistant", content: deepseekResponse.value });
              emit('update:messages', localMessages.value);
              
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
    let contextMessages = [];
    
    if (localMessages.value.length > maxContextLength.value * 2 + 1) {
      contextMessages.push(localMessages.value[0]);

      // 根据maxContextLength截取消息
      const startIdx = localMessages.value.length - (maxContextLength.value * 2 + 1);
      const messages = localMessages.value.slice(startIdx, startIdx + maxContextLength.value * 2 + 1);
      contextMessages.push(...messages);
    }
    else{
      contextMessages = localMessages.value;
    }

    console.log('contextMessages:', contextMessages, localMessages.value)
    await invoke('receive_stream', {
      url: `${baseURL.value}/chat/completions`,
      authToken: `Bearer ${apiKey.value}`,
      prompt: JSON.stringify({
        model: model.value,
        messages: contextMessages,
        stream: true
      }),
    });
  } catch (error) {
    console.error('流处理错误:', error);
    deepseekResponse.value = "抱歉，解释生成失败，请稍后重试。";
    isLoading.value = false;
    if (fetchStreamUnlisten) {
      fetchStreamUnlisten();
      fetchStreamUnlisten = null;
    }
  } finally {
    if (fetchStreamUnlisten) {
      fetchStreamUnlisten();
      fetchStreamUnlisten = null;
    }
  }
}

// 添加清空状态的方法
const clearState = () => {
  chatHistory.value = [];
  newQuestion.value = "";
  deepseekResponse.value = "";
  isLoading.value = false;
  localMessages.value = [];
};

const saveMultiChatHistory = () => {
  const initialQuestion = localMessages.value[1].content; // 第一个用户问题
  const now = new Date();
    // 构建完整的对话历史
  let fullResponse = '';
  for (let i = 1; i < localMessages.value.length; i++) {
    const msg = localMessages.value[i];
    if (msg.role === 'user') {
      fullResponse += 'Q: ' + msg.content + '\n\n';
    } else if (msg.role === 'assistant') {
      fullResponse += 'A: ' + msg.content + '\n\n';
    }
  }
  saveChatHistory({
    content: initialQuestion,
    response: fullResponse.trim(),
    model: model.value,
    timestamp: now.toISOString()
  });
}

// 暴露方法给父组件
defineExpose({
  clearState,
  saveMultiChatHistory
});

onUnmounted(() => {
  if (fetchStreamUnlisten) {
    fetchStreamUnlisten();
  }
});
</script>

<style scoped>
.selectable-text {
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
}

.el-input {
  flex: 1;
}
</style> 